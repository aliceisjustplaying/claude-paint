-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 2

--@ chunk 1
canvas{size=480, aspect=0.72, linen={14, 12}, seed=41,
  ground={
    {pile={{"red earth",3},{"yellow ochre",3},{"lead white",1}}, um=50, apply="knife", texture=0.55},
    {pile={{"lead white",7},{"yellow ochre",1},{"raw umber",1},{"red earth",1}}, um=55, apply="roller"},
    {pile={{"lead white",10},{"yellow ochre",1}}, um=28, apply="brush"}}}

-- palette, first piles
skypale  = pile{{"lead white",8},{"pale smalt",1},{"yellow ochre",1}, medium=0.15}
skyblue  = pile{{"pale smalt",3},{"lead white",3}, medium=0.15}
skydeep  = pile{{"smalt",5},{"lead white",2}, medium=0.15}
skywarm  = pile{{"lead white",8},{"yellow ochre",2},{"red earth",1}, medium=0.2}
hillfar   = pile{{"pale smalt",3},{"smalt",2},{"lead white",3}, medium=0.25}
waterpale= pile{{"lead white",6},{"pale smalt",3},{"yellow ochre",1}, medium=0.3}
waterdark= pile{{"smalt",3},{"raw umber",1},{"lead white",1}, medium=0.35}
barkpale = pile{{"lead white",7},{"yellow ochre",2},{"raw umber",1}, medium=0.1}
barkdark = pile{{"raw umber",4},{"bone black",1},{"lead white",1}, medium=0.1}
earthdark= pile{{"raw umber",4},{"bone black",2},{"yellow ochre",1}, medium=0.2}
print("H", H, "sky", skypale, "water", waterpale, "bark", barkpale)

--@ chunk 2
-- underdrawing: the far shore, the ridge beyond it, the mist line
h2 = pencil("2H")
h2:line({{-10, 624}, {220, 620}, {470, 623}, {700, 618}, {1010, 621}}, {pressure={0.22, 0.3, 0.24}})
h2:line({{-10, 626}, {200, 623}, {430, 627}, {660, 621}, {1010, 624}}, {pressure={0.14, 0.2, 0.15}})
-- the low ridge beyond the water
h2:line({{-10, 601}, {120, 594}, {250, 584}, {380, 591}, {505, 583}, {640, 590}, {770, 580}, {900, 588}, {1010, 585}}, {pressure={0.24, 0.3, 0.22}})
h2:line({{-10, 608}, {140, 600}, {300, 592}, {520, 594}, {700, 599}, {1010, 596}}, {pressure={0.12, 0.18, 0.13}})
-- the bank in the foreground, its edge running across the bottom
h2:line({{-10, 1252}, {130, 1276}, {265, 1303}, {405, 1292}, {560, 1252}, {700, 1229}, {855, 1239}, {1010, 1264}}, {pressure={0.3, 0.36, 0.28}})
print("sketched far shore, ridge, bank")

--@ chunk 3
-- the drowned birch grove: every trunk its own path, leaning its own way
h2:line({{145, 566}, {140, 620}, {136, 684}, {138, 748}}, {pressure={0.3, 0.34, 0.28}})
h2:line({{150, 568}, {146, 622}, {142, 686}}, {pressure={0.2, 0.24, 0.2}})
h2:line({{205, 588}, {209, 640}, {212, 694}, {211, 752}}, {pressure={0.28, 0.3, 0.24}})
h2:line({{262, 552}, {256, 606}, {250, 668}, {250, 740}}, {pressure={0.28, 0.32, 0.26}})
h2:line({{312, 574}, {322, 632}, {331, 694}, {330, 756}}, {pressure={0.32, 0.34, 0.28}})
h2:line({{318, 576}, {328, 634}, {337, 696}}, {pressure={0.2, 0.24, 0.2}})
h2:line({{372, 560}, {366, 614}, {360, 678}, {360, 744}}, {pressure={0.28, 0.3, 0.24}})
h2:line({{425, 586}, {432, 640}, {438, 694}, {438, 750}}, {pressure={0.3, 0.32, 0.26}})
h2:line({{425, 586}, {436, 594}, {450, 606}}, {pressure={0.3, 0.3, 0.26}})   -- the snapped one
h2:line({{480, 566}, {476, 618}, {470, 682}, {470, 746}}, {pressure={0.28, 0.3, 0.24}})
h2:line({{528, 578}, {534, 636}, {541, 696}, {540, 754}}, {pressure={0.32, 0.34, 0.28}})
h2:line({{534, 580}, {540, 638}, {547, 698}}, {pressure={0.2, 0.24, 0.2}})
h2:line({{585, 558}, {580, 612}, {574, 678}, {574, 742}}, {pressure={0.28, 0.3, 0.24}})
h2:line({{640, 588}, {646, 642}, {652, 696}, {652, 750}}, {pressure={0.28, 0.3, 0.24}})
h2:line({{700, 570}, {694, 622}, {690, 686}, {690, 748}}, {pressure={0.28, 0.3, 0.24}})
h2:line({{762, 598}, {768, 646}, {770, 700}, {768, 752}}, {pressure={0.24, 0.26, 0.2}})
h2:line({{845, 604}, {850, 650}, {852, 700}, {850, 750}}, {pressure={0.2, 0.22, 0.18}})
-- a few bare side branches reaching out of the mist
h2:line({{330, 700}, {362, 684}, {390, 676}}, {pressure={0.2, 0.2, 0.14}})
h2:line({{534, 700}, {568, 686}, {596, 682}}, {pressure={0.2, 0.2, 0.14}})
h2:line({{140, 662}, {108, 650}, {86, 648}}, {pressure={0.18, 0.18, 0.12}})
print("grove drawn")

--@ chunk 4
-- the near trunks that frame the left of the view, running out of the top
h2:line({{-16, 1389}, {4, 1120}, {28, 820}, {47, 460}, {59, 120}, {65, -12}}, {pressure={0.34, 0.3, 0.28}})
h2:line({{22, 1389}, {47, 1120}, {68, 820}, {85, 460}, {96, 120}, {100, -12}}, {pressure={0.3, 0.28, 0.26}})
h2:line({{118, 1288}, {140, 1050}, {160, 800}, {176, 480}, {188, 120}, {193, -12}}, {pressure={0.28, 0.26, 0.24}})
h2:line({{146, 1286}, {164, 1050}, {182, 800}, {196, 480}, {206, 120}, {210, -12}}, {pressure={0.22, 0.2, 0.2}})
h2:line({{-4, 700}, {40, 664}, {86, 648}, {124, 646}}, {pressure={0.16, 0.18, 0.12}})   -- high branch, left
-- the fallen birch lying on the bank, its tip out over the water
h2:line({{-20, 1332}, {110, 1316}, {240, 1306}, {352, 1289}, {438, 1252}}, {pressure={0.3, 0.32, 0.26}})
h2:line({{-20, 1354}, {112, 1338}, {242, 1328}, {354, 1311}, {440, 1272}}, {pressure={0.24, 0.26, 0.2}})
h2:line({{352, 1292}, {382, 1274}, {404, 1258}}, {pressure={0.2, 0.2, 0.14}})
h2:line({{168, 1318}, {206, 1300}, {246, 1292}}, {pressure={0.16, 0.18, 0.12}})

-- the figure, small, seen from behind on the bank
hb = pencil("HB")
hb:line({{756, 1258}, {757, 1240}, {757, 1224}, {756, 1213}}, {pressure={0.4, 0.5, 0.4}})
hb:line({{769, 1258}, {768, 1240}, {767, 1224}, {768, 1213}}, {pressure={0.4, 0.5, 0.4}})
hb:line({{756, 1213}, {752, 1224}, {753, 1240}}, {pressure={0.4, 0.4, 0.34}})
hb:line({{768, 1213}, {771, 1224}, {771, 1240}}, {pressure={0.4, 0.4, 0.34}})
hb:line({{762, 1200}, {762, 1212}}, {pressure={0.4, 0.4}})
hb:line({{757, 1203}, {760, 1197}, {765, 1198}, {767, 1203}}, {pressure={0.4, 0.5, 0.4}})
hb:line({{757, 1240}, {771, 1240}}, {pressure={0.3, 0.3}})
print("foreground and figure drawn")

--@ chunk 5
-- firmer contour lines: Friedrich's drawing stays half visible under thin paint
hH = pencil("H")
hH:line({{-10, 624}, {220, 620}, {470, 623}, {700, 618}, {1010, 621}}, {pressure={0.4, 0.5, 0.38}})
hH:line({{-10, 601}, {120, 594}, {250, 584}, {380, 591}, {505, 583}, {640, 590}, {770, 580}, {900, 588}, {1010, 585}}, {pressure={0.38, 0.46, 0.36}})
hH:line({{-10, 1252}, {130, 1276}, {265, 1303}, {405, 1292}, {560, 1252}, {700, 1229}, {855, 1239}, {1010, 1264}}, {pressure={0.44, 0.52, 0.42}})
hH:line({{145, 566}, {140, 620}, {136, 684}, {138, 748}}, {pressure={0.4, 0.44, 0.36}})
hH:line({{205, 588}, {209, 640}, {212, 694}, {211, 752}}, {pressure={0.38, 0.42, 0.34}})
hH:line({{262, 552}, {256, 606}, {250, 668}, {250, 740}}, {pressure={0.38, 0.42, 0.34}})
hH:line({{312, 574}, {322, 632}, {331, 694}, {330, 756}}, {pressure={0.44, 0.48, 0.38}})
hH:line({{372, 560}, {366, 614}, {360, 678}, {360, 744}}, {pressure={0.36, 0.4, 0.32}})
hH:line({{425, 586}, {432, 640}, {438, 694}, {438, 750}}, {pressure={0.38, 0.42, 0.34}})
hH:line({{480, 566}, {476, 618}, {470, 682}, {470, 746}}, {pressure={0.36, 0.4, 0.32}})
hH:line({{528, 578}, {534, 636}, {541, 696}, {540, 754}}, {pressure={0.42, 0.46, 0.36}})
hH:line({{585, 558}, {580, 612}, {574, 678}, {574, 742}}, {pressure={0.36, 0.4, 0.32}})
hH:line({{640, 588}, {646, 642}, {652, 696}, {652, 750}}, {pressure={0.36, 0.4, 0.32}})
hH:line({{700, 570}, {694, 622}, {690, 686}, {690, 748}}, {pressure={0.36, 0.4, 0.32}})
hH:line({{762, 598}, {768, 646}, {770, 700}, {768, 752}}, {pressure={0.32, 0.34, 0.28}})
hH:line({{845, 604}, {850, 650}, {852, 700}, {850, 750}}, {pressure={0.28, 0.3, 0.24}})
-- near trunks
hH:line({{-16, 1389}, {4, 1120}, {28, 820}, {47, 460}, {59, 120}, {65, -12}}, {pressure={0.5, 0.44, 0.4}})
hH:line({{22, 1389}, {47, 1120}, {68, 820}, {85, 460}, {96, 120}, {100, -12}}, {pressure={0.44, 0.4, 0.36}})
hH:line({{118, 1288}, {140, 1050}, {160, 800}, {176, 480}, {188, 120}, {193, -12}}, {pressure={0.42, 0.38, 0.34}})
hH:line({{146, 1286}, {164, 1050}, {182, 800}, {196, 480}, {206, 120}, {210, -12}}, {pressure={0.34, 0.32, 0.3}})
-- the fallen birch
hH:line({{-20, 1332}, {110, 1316}, {240, 1306}, {352, 1289}, {438, 1252}}, {pressure={0.46, 0.48, 0.4}})
hH:line({{-20, 1354}, {112, 1338}, {242, 1328}, {354, 1311}, {440, 1272}}, {pressure={0.36, 0.38, 0.32}})
print("contours firmed")

--@ chunk 6
horiz = {{-30, 624}, {220, 620}, {470, 623}, {700, 618}, {1030, 621}}
ridge = {{-30, 601}, {120, 594}, {250, 584}, {380, 591}, {505, 583}, {640, 590}, {770, 580}, {900, 588}, {1030, 585}}
bank  = {{-30, 1250}, {130, 1276}, {265, 1303}, {405, 1292}, {560, 1252}, {700, 1229}, {855, 1239}, {1030, 1264}}

sky    = above(horiz)
ridge_m = above(ridge) * below(horiz)
water  = below(horiz) * above(bank)
land   = below(bank)
print("areas", sky:area(), ridge_m:area(), water:area(), land:area())

--@ chunk 7
ridge_m = below(ridge) * above(horiz)
skyab  = above(ridge)
up1 = pile{{"smalt",4},{"lead white",4}, medium=0.5}
up2 = pile{{"pale smalt",4},{"lead white",5},{"yellow ochre",1}, medium=0.5}
up3 = pile{{"smalt",5},{"lead white",4},{"yellow ochre",2}, medium=0.45}

work(skyab * rect(-30, -30, 1060, 180):soften(60), {hand="glaze", pile=up1, tool="filbert 20", pressure={0.3, 0.16}, coverage=1.2, clip=true})
work(skyab * rect(-30, 110, 1060, 240):soften(70), {hand="glaze", pile=up2, tool="filbert 20", pressure={0.3, 0.16}, coverage=1.1, clip=true})
work(skyab * rect(-30, 300, 1060, 240):soften(70), {hand="glaze", pile=up3, tool="filbert 20", pressure={0.28, 0.16}, coverage=1.0, clip=true})
work(skyab * rect(-30, 470, 1060, 170):soften(60), {hand="glaze", pile=up2, tool="filbert 20", pressure={0.26, 0.14}, coverage=0.9, clip=true})
print("sky underpainting laid")

--@ chunk 8
print(wait(120))
skymid = pile{{"pale smalt",3},{"lead white",8},{"yellow ochre",1}, medium=0.4}
-- even the sky out: a broad, thin veil over the streaky washes
work(skyab, {hand="broad", pile=skymid, tool="filbert 24", length={130, 240}, coverage=2.0,
             pressure={0.28, 0.2}, fill=true, clip=true, angle=0.06})
print(drying(500, 200), drying(500, 400))

--@ chunk 9
ridge_m = below(ridge) * above(horiz)
-- the far ridge, pale and hazy, seen through the mist
ridgewash = pile{{"pale smalt",4},{"lead white",6},{"raw umber",1}, medium=0.45}
work(ridge_m, {hand="body", pile=ridgewash, tool="filbert 12", coverage=1.6, pressure={0.3, 0.18}, clip=true, edge="lost"})

-- the water: broad cool washes, deeper toward the near shore
w1 = pile{{"pale smalt",3},{"lead white",7}, medium=0.5}
w2 = pile{{"pale smalt",3},{"lead white",5},{"raw umber",1}, medium=0.5}
w3 = pile{{"smalt",3},{"raw umber",1},{"lead white",3}, medium=0.55}
work(water * rect(-30, 600, 1060, 130):soften(45), {hand="broad", pile=w1, tool="filbert 24", length={140, 260}, coverage=1.5, pressure={0.3, 0.2}, clip=true, angle=0.04})
work(water * rect(-30, 720, 1060, 220):soften(60), {hand="broad", pile=w2, tool="filbert 24", length={140, 260}, coverage=1.4, pressure={0.3, 0.2}, clip=true, angle=0.04})
work(water * rect(-30, 930, 1060, 230):soften(70), {hand="broad", pile=w3, tool="filbert 24", length={140, 260}, coverage=1.4, pressure={0.32, 0.22}, clip=true, angle=0.04})
work(water * rect(-30, 1140, 1060, 160):soften(60), {hand="broad", pile=w3, tool="filbert 24", length={140, 260}, coverage=1.3, pressure={0.34, 0.22}, clip=true, angle=0.04})
print("ridge and water underpainting laid")

--@ chunk 10
weven = pile{{"pale smalt",3},{"lead white",6},{"raw umber",1}, medium=0.3}
work(water, {hand="body", pile=weven, tool="filbert 14", length={45, 95}, coverage=3.2,
             pressure={0.3, 0.2}, dips={3, 0.9, 0.3}, clip=true, angle=0.05})
print(drying(500, 900))

--@ chunk 11
grove = {
 {pts={{145,566},{140,620},{136,684},{138,748}}, w=5.0},
 {pts={{205,588},{209,640},{212,694},{211,752}}, w=4.2},
 {pts={{262,552},{256,606},{250,668},{250,740}}, w=4.6},
 {pts={{312,574},{322,632},{331,694},{330,756}}, w=6.0},
 {pts={{372,560},{366,614},{360,678},{360,744}}, w=4.0},
 {pts={{425,586},{432,640},{438,694},{438,750}}, w=5.2},
 {pts={{480,566},{476,618},{470,682},{470,746}}, w=4.2},
 {pts={{528,578},{534,636},{541,696},{540,754}}, w=5.8},
 {pts={{585,558},{580,612},{574,678},{574,742}}, w=4.4},
 {pts={{640,588},{646,642},{652,696},{652,750}}, w=4.6},
 {pts={{700,570},{694,622},{690,686},{690,748}}, w=4.2},
 {pts={{762,598},{768,646},{770,700},{768,752}}, w=3.8},
 {pts={{845,604},{850,650},{852,700},{850,750}}, w=3.4}}

birch = pile{{"lead white",7},{"raw umber",1},{"yellow ochre",1}, medium=0.2}
for i, t in ipairs(grove) do
  m = ribbon(t.pts, t.w)
  work(m, {hand="body", pile=birch, tool="filbert 9", coverage=1.8, pressure={0.4, 0.3},
           edge="soft", clip=true, length={25, 60}})
end
print("grove trunks laid in pale bark")

--@ chunk 12
weven2 = pile{{"lead white",7},{"pale smalt",3},{"raw umber",1}, medium=0.22}
work(water, {hand="body", pile=weven2, tool="filbert 18", length={60, 120}, coverage=2.6,
             pressure={0.26, 0.18}, dips={4, 0.85, 0.3}, clip=true, angle=0.04, fill=true})
print("water evened")

--@ chunk 13
mistv = pile{{"lead white",9},{"pale smalt",2}, medium=0.3}
mband = rect(-30, 588, 1060, 105):soften(55)
work(mband, {hand="body", pile=mistv, tool="filbert 18", length={70, 140}, coverage=2.6,
             pressure={0.26, 0.18}, dips={4, 0.85, 0.3}, angle=0.03, fill=true})
print("mist band laid")

--@ chunk 14
birch = pile{{"lead white",7},{"raw umber",1},{"yellow ochre",1}, medium=0.15}
press = {0.62, 0.55, 0.6, 0.8, 0.58, 0.66, 0.58, 0.78, 0.6, 0.62, 0.58, 0.52, 0.46}
for i, t in ipairs(grove) do
  b = brush{kind="round", width=t.w * 1.7, point=0.5, stiffness=0.45}
  b:load(birch, 0.85)
  b:stroke(t.pts, {pressure={press[i], press[i] + 0.28, press[i] + 0.1}, orient="across", shake=0.8})
  b:stroke(t.pts, {pressure={0.22, 0.44, 0.66}, orient="across", shake=1.2})
end
print("trunks stroked")

--@ chunk 15
barklight = pile{{"lead white",8},{"raw umber",1}, medium=0.15}
barkshade = pile{{"raw umber",2},{"lead white",4},{"pale smalt",1}, medium=0.2}
press = {0.62, 0.55, 0.6, 0.8, 0.58, 0.66, 0.58, 0.78, 0.6, 0.62, 0.58, 0.52, 0.46}
for i, t in ipairs(grove) do
  b = brush{kind="round", width=t.w * 1.9, point=0.45, stiffness=0.45}
  b:load(barklight, 0.95)
  b:stroke(t.pts, {pressure={press[i] + 0.06, press[i] + 0.3, press[i] + 0.18}, orient="across", shake=0.7})
  b:stroke(t.pts, {pressure={0.2, 0.44, 0.7}, orient="across", shake=1.1})
end
shade = {
 {{147, 604}, {143, 662}, {140, 712}},
 {{203, 642}, {207, 692}, {209, 732}},
 {{254, 600}, {250, 660}, {249, 716}},
 {{317, 612}, {324, 672}, {328, 732}},
 {{365, 610}, {359, 670}, {359, 720}},
 {{428, 632}, {433, 682}, {436, 732}},
 {{474, 612}, {469, 672}, {468, 722}},
 {{531, 622}, {537, 682}, {539, 732}},
 {{578, 606}, {573, 666}, {572, 716}},
 {{643, 626}, {649, 686}, {651, 732}},
 {{693, 616}, {688, 676}, {687, 726}},
 {{765, 636}, {768, 686}, {767, 732}},
 {{847, 640}, {849, 686}, {848, 728}}}
sb = brush{kind="round", width=2.6, point=0.6, stiffness=0.5}
for i, p in ipairs(shade) do
  sb:load(barkshade, 0.7)
  sb:stroke(p, {pressure={0.35, 0.62, 0.7}, orient="across", shake=1.4})
end
print("trunks repainted pale, shaded edges added")

--@ chunk 16
-- knock the values back so the pale trunks can read against them
wv = pile{{"pale smalt",3},{"lead white",4},{"raw umber",1}, medium=0.35}
work(water * rect(-30, 700, 1060, 300):soften(80), {hand="body", pile=wv, tool="filbert 18", length={70, 140}, coverage=1.5, pressure={0.26, 0.18}, dips={4, 0.8, 0.3}, clip=true, angle=0.03, fill=true})
work(water * rect(-30, 950, 1060, 330):soften(90), {hand="body", pile=wv, tool="filbert 18", length={70, 140}, coverage=1.7, pressure={0.3, 0.2}, dips={4, 0.8, 0.3}, clip=true, angle=0.03, fill=true})
-- the sky deeper at the top
sv = pile{{"smalt",4},{"pale smalt",3},{"lead white",3}, medium=0.3}
work(rect(-30, -30, 1060, 220):soften(90), {hand="body", pile=sv, tool="filbert 18", length={70, 140}, coverage=1.6, pressure={0.28, 0.2}, dips={4, 0.8, 0.3}, clip=true, angle=0.05, fill=true})
work(rect(-30, 120, 1060, 240):soften(90), {hand="body", pile=sv, tool="filbert 18", length={70, 140}, coverage=1.3, pressure={0.26, 0.18}, dips={4, 0.8, 0.3}, clip=true, angle=0.05, fill=true})
print("values knocked back")

--@ chunk 17
skyveil = pile{{"smalt",3},{"pale smalt",3},{"lead white",6}, medium=0.3}
skyab = above(ridge)
work(skyab, {hand="body", pile=skyveil, tool="filbert 18", length={80, 150}, coverage=2.4,
             pressure={0.26, 0.18}, dips={4, 0.85, 0.3}, clip=true, angle=0.04, fill=true})
print("sky veiled evenly")

--@ chunk 18
-- cloud forms, drawn as shapes and glazed in, not computed into the sky
cl1 = pile{{"smalt",4},{"pale smalt",3},{"lead white",3}, medium=0.35}
cl2 = pile{{"pale smalt",4},{"lead white",5},{"yellow ochre",2}, medium=0.35}
c1a = ellipse(180, 96, 260, 34):soften(34)
c1b = ellipse(430, 78, 200, 26):soften(30)
c1c = ellipse(660, 112, 300, 30):soften(36)
c1d = ellipse(880, 86, 190, 22):soften(28)
work((c1a + c1b + c1c + c1d) * skyab, {hand="glaze", pile=cl1, tool="filbert 26", pressure={0.3, 0.2}, coverage=1.0})
c2a = ellipse(300, 262, 230, 20):soften(28)
c2b = ellipse(720, 232, 260, 17):soften(26)
c2c = ellipse(540, 316, 190, 14):soften(22)
work((c2a + c2b + c2c) * skyab, {hand="glaze", pile=cl1, tool="filbert 26", pressure={0.26, 0.16}, coverage=0.9})
-- the warm evening light, low and broad above the vapour
g1 = ellipse(520, 470, 520, 40):soften(50)
g2 = ellipse(300, 520, 340, 34):soften(46)
g3 = ellipse(780, 528, 330, 32):soften(46)
work((g1 + g2 + g3) * skyab, {hand="glaze", pile=cl2, tool="filbert 26", pressure={0.3, 0.18}, coverage=1.0})
print("clouds and evening light")

--@ chunk 19
haze = pile{{"lead white",8},{"pale smalt",1},{"yellow ochre",1}, medium=0.3}
work(skyab * rect(-30, 380, 1060, 250):soften(80), {hand="body", pile=haze, tool="filbert 18", length={80, 150}, coverage=1.8, pressure={0.24, 0.16}, dips={4, 0.85, 0.3}, clip=true, angle=0.03, fill=true})
-- the glow, right down on the vapour
glow = pile{{"lead white",7},{"yellow ochre",2},{"red earth",1}, medium=0.3}
q1 = ellipse(480, 560, 560, 44):soften(56)
q2 = ellipse(180, 572, 330, 34):soften(50)
q3 = ellipse(860, 566, 300, 32):soften(50)
work((q1 + q2 + q3) * skyab, {hand="glaze", pile=glow, tool="filbert 26", pressure={0.3, 0.2}, coverage=0.9})
print("glow laid on the horizon")

--@ chunk 20
-- wider, cooler bark: the trunks have to read as trunks
gw = {7, 6, 6.5, 9, 5.5, 7.5, 6, 8.5, 6.5, 7, 6, 5.5, 5}
barksil = pile{{"lead white",6},{"raw umber",1},{"pale smalt",1.5}, medium=0.15}
press = {0.62, 0.55, 0.6, 0.8, 0.58, 0.66, 0.58, 0.78, 0.6, 0.62, 0.58, 0.52, 0.46}
for i, t in ipairs(grove) do
  t.w = gw[i]
  b = brush{kind="filbert", width=gw[i] * 1.5, stiffness=0.55}
  b:load(barksil, 0.95)
  b:stroke(t.pts, {pressure={press[i] + 0.1, press[i] + 0.3, press[i] + 0.2}, orient="across", shake=0.6})
  b:stroke(t.pts, {pressure={0.18, 0.45, 0.72}, orient="across", shake=1.0})
end
print("grove repainted in cool bark")

--@ chunk 21
-- the vapour knocked back, so the pale bark can carry
mv = pile{{"pale smalt",3},{"lead white",5},{"raw umber",1}, medium=0.35}
work(rect(-30, 592, 1060, 120):soften(60), {hand="body", pile=mv, tool="filbert 18", length={80, 150},
     coverage=1.7, pressure={0.26, 0.18}, dips={4, 0.85, 0.3}, angle=0.03, fill=true})

barkdk = pile{{"raw umber",3},{"bone black",1},{"lead white",2}, medium=0.12}
db = brush{kind="filbert", width=3.2, stiffness=0.5}
db:load(barkdk, 0.9)
bases = {{{134, 742}, {136, 762}, {137, 782}}, {{210, 748}, {211, 768}, {210, 786}},
         {{249, 736}, {250, 758}, {250, 778}}, {{329, 752}, {331, 774}, {330, 794}},
         {{360, 740}, {359, 762}, {360, 780}}, {{436, 746}, {437, 768}, {437, 788}},
         {{469, 742}, {468, 764}, {469, 782}}, {{539, 750}, {540, 772}, {540, 792}},
         {{574, 738}, {573, 760}, {573, 780}}, {{651, 746}, {651, 766}, {650, 784}},
         {{690, 744}, {689, 764}, {690, 782}}, {{768, 748}, {767, 768}, {768, 784}},
         {{851, 746}, {850, 764}, {850, 780}}}
for _, p in ipairs(bases) do
  db:load(barkdk, 0.9)
  db:stroke(p, {pressure={0.4, 0.7, 0.25}, orient="across", shake=1.6})
end
print("mist knocked back, trunk feet darkened")

--@ chunk 22
-- birch markings: lenticels and the scars of old branches
lb = brush{kind="round", width=2.0, point=0.5, stiffness=0.5}
lent = {
 {137, 664, 5}, {139, 692, 4}, {136, 724, 5},
 {210, 682, 4}, {211, 714, 5},
 {251, 640, 5}, {249, 674, 5}, {250, 708, 4},
 {327, 642, 7}, {331, 674, 6}, {330, 708, 7}, {328, 734, 6},
 {361, 692, 4},
 {433, 650, 6}, {436, 686, 5}, {437, 722, 6},
 {471, 660, 5}, {469, 702, 4},
 {535, 646, 7}, {539, 682, 6}, {540, 716, 7}, {538, 738, 6},
 {575, 668, 5}, {573, 706, 5},
 {648, 662, 6}, {651, 700, 5}, {650, 732, 5},
 {690, 676, 5}, {688, 714, 4},
 {768, 692, 4}, {851, 702, 4}}
for _, L in ipairs(lent) do
  lb:load(barkdk, 0.6)
  lb:touch(L[1], L[2], {pressure=0.5, drag={L[3], 0.05}, twist=0.3})
end
-- scars where limbs were broken off
sc = brush{kind="round", width=3.4, point=0.5, stiffness=0.55}
sc:load(barkdk, 0.9)
sc:touch(332, 700, {pressure=0.75, drag={5, 2}})
sc:touch(330, 664, {pressure=0.65, drag={4, -2}})
sc:touch(541, 682, {pressure=0.8, drag={6, 2}})
sc:touch(538, 646, {pressure=0.6, drag={4, -2}})
sc:touch(436, 700, {pressure=0.7, drag={5, 2}})
sc:touch(212, 700, {pressure=0.6, drag={4, -2}})
sc:touch(574, 690, {pressure=0.55, drag={4, 2}})
print("lenticels and scars")

--@ chunk 23
-- a real step down in value behind the grove
wd = pile{{"smalt",3},{"raw umber",2},{"lead white",2}, medium=0.3}
work(water, {hand="body", pile=wd, tool="filbert 18", length={80, 150}, coverage=1.9,
             pressure={0.3, 0.2}, dips={4, 0.85, 0.3}, clip=true, angle=0.03, fill=true})
work(rect(-30, 588, 1060, 115):soften(55), {hand="body", pile=wd, tool="filbert 18", length={80, 150},
     coverage=1.2, pressure={0.28, 0.18}, dips={4, 0.85, 0.3}, angle=0.03, fill=true})
print("water and vapour darkened")

--@ chunk 24
barkop = pile{{"lead white",8},{"raw umber",0.8},{"pale smalt",0.8}, medium=0.04}
for i, t in ipairs(grove) do
  b = brush{kind="filbert", width=gw[i] * 1.5, stiffness=0.6}
  b:load(barkop, 1.0)
  b:stroke(t.pts, {pressure={0.5, 0.85, 0.9}, orient="across", shake=0.5})
end
print("trunks laid opaquely")

--@ chunk 25
barkdk = pile{{"raw umber",3},{"bone black",1},{"lead white",2}, medium=0.12}
-- the shaded side of each trunk, and its markings
sb = brush{kind="filbert", width=3.0, stiffness=0.5}
for i, p in ipairs(shade) do
  sb:load(barkdk, 0.75)
  sb:stroke(p, {pressure={0.2, 0.5, 0.62}, orient="across", shake=1.3})
end
lb = brush{kind="round", width=2.0, point=0.5, stiffness=0.5}
for _, L in ipairs(lent) do
  lb:load(barkdk, 0.65)
  lb:touch(L[1], L[2], {pressure=0.55, drag={L[3] * 1.5, 0.05}, twist=0.3})
end
-- branch stubs, each its own small mark
tb = brush{kind="round", width=2.6, point=0.7, stiffness=0.5}
tb:load(barkdk, 0.9)
tb:stroke({{332, 652}, {346, 641}, {358, 638}}, {pressure={0.6, 0.4, 0.15}, orient="across"})
tb:stroke({{330, 700}, {346, 692}, {360, 690}}, {pressure={0.6, 0.4, 0.15}, orient="across"})
tb:stroke({{540, 664}, {556, 655}, {570, 653}}, {pressure={0.6, 0.4, 0.15}, orient="across"})
tb:stroke({{538, 712}, {552, 704}, {566, 702}}, {pressure={0.55, 0.35, 0.12}, orient="across"})
tb:stroke({{437, 678}, {450, 670}, {462, 668}}, {pressure={0.55, 0.35, 0.12}, orient="across"})
tb:stroke({{212, 664}, {224, 657}, {235, 656}}, {pressure={0.5, 0.3, 0.1}, orient="across"})
tb:stroke({{574, 636}, {588, 629}, {600, 628}}, {pressure={0.5, 0.3, 0.1}, orient="across"})
tb:stroke({{330, 726}, {346, 720}, {360, 719}}, {pressure={0.5, 0.32, 0.1}, orient="across"})
print("shading, markings, stubs")

--@ chunk 26
-- what the water does with each trunk: a broken reflection
rb = brush{kind="filbert", width=4.0, stiffness=0.45}
refl = {
 {136, 766, 16, 0.8}, {136, 792, 13, 0.7}, {135, 816, 10, 0.6}, {135, 838, 7, 0.5},
 {211, 772, 17, 0.85}, {211, 798, 14, 0.75}, {210, 822, 10, 0.6}, {210, 844, 7, 0.5},
 {250, 760, 15, 0.8}, {250, 784, 12, 0.7}, {249, 806, 9, 0.55}, {249, 826, 6, 0.45},
 {330, 776, 22, 0.9}, {330, 806, 18, 0.8}, {331, 834, 13, 0.65}, {330, 860, 9, 0.5},
 {360, 762, 15, 0.8}, {359, 786, 12, 0.7}, {359, 808, 9, 0.55},
 {437, 768, 19, 0.88}, {437, 796, 15, 0.75}, {436, 820, 11, 0.6}, {436, 842, 8, 0.45},
 {469, 764, 16, 0.82}, {468, 790, 13, 0.7}, {468, 812, 9, 0.55},
 {540, 776, 22, 0.9}, {540, 806, 17, 0.8}, {540, 832, 13, 0.65}, {539, 856, 9, 0.5},
 {573, 760, 15, 0.8}, {573, 784, 12, 0.7}, {572, 806, 9, 0.55},
 {651, 768, 17, 0.85}, {651, 794, 14, 0.75}, {650, 816, 10, 0.6}, {650, 836, 7, 0.45},
 {690, 766, 16, 0.82}, {689, 792, 12, 0.7}, {689, 814, 9, 0.55},
 {768, 770, 15, 0.8}, {767, 796, 12, 0.7}, {767, 818, 9, 0.55},
 {850, 768, 14, 0.78}, {850, 792, 11, 0.68}, {850, 812, 8, 0.5}}
for _, r in ipairs(refl) do
  rb:load(barkdk, 0.8)
  rb:touch(r[1], r[2], {pressure=r[3 + 1] * 0.8, drag={0.4, r[3]}, twist=0.2})
end
print("reflections laid in")

--@ chunk 27
-- calm the water: long horizontal films over the pebbly underpainting
wsm = pile{{"smalt",3},{"raw umber",2},{"lead white",2.5}, medium=0.18}
work(water, {hand="glaze", pile=wsm, tool="filbert 26", coverage=1.5, pressure={0.3, 0.22}, clip=true, angle=0.02})
work(water, {hand="glaze", pile=wsm, tool="filbert 26", coverage=1.3, pressure={0.28, 0.2}, clip=true, angle=-0.03})
print("water calmed")

--@ chunk 28
-- the luminous band of vapour, and the water stepped back down below it
mistv = pile{{"lead white",8},{"pale smalt",2}, medium=0.25}
work(rect(-30, 592, 1060, 115):soften(40), {hand="glaze", pile=mistv, tool="filbert 26", coverage=1.7, pressure={0.3, 0.2}, angle=0.02})
wdeep = pile{{"smalt",3},{"raw umber",2.5},{"lead white",2}, medium=0.16}
work(rect(-30, 700, 1060, 180):soften(55), {hand="glaze", pile=wdeep, tool="filbert 26", coverage=1.5, pressure={0.32, 0.24}, clip=true, angle=0.015})
work(rect(-30, 850, 1060, 210):soften(65), {hand="glaze", pile=wdeep, tool="filbert 26", coverage=1.6, pressure={0.34, 0.26}, clip=true, angle=-0.02})
work(rect(-30, 1040, 1060, 240):soften(70), {hand="glaze", pile=wdeep, tool="filbert 26", coverage=1.7, pressure={0.36, 0.28}, clip=true, angle=0.025})
print("mist band and water steps")

--@ chunk 29
-- the surface of the water: a few long lights and the shadow between them
glint = pile{{"lead white",8},{"pale smalt",2},{"yellow ochre",1}, medium=0.22}
lights = {
 {150, 836, 170, 4}, {520, 858, 210, 4}, {860, 830, 150, 3.5},
 {90, 918, 210, 5}, {430, 940, 250, 5}, {790, 906, 190, 4},
 {240, 1002, 230, 5}, {660, 1024, 240, 5}, {60, 1078, 180, 5},
 {380, 1092, 260, 6}, {740, 1080, 220, 5}, {150, 1160, 250, 6},
 {520, 1178, 280, 6}, {860, 1148, 200, 5}}
for _, g in ipairs(lights) do
  work(ellipse(g[1], g[2], g[3], g[4]):soften(g[4] * 1.3), {hand="glaze", pile=glint, tool="filbert 26", coverage=1.0, pressure={0.3, 0.18}, clip=true})
end
darkw = pile{{"smalt",3},{"raw umber",3},{"lead white",1.5}, medium=0.2}
darks = {
 {300, 806, 190, 5}, {700, 880, 200, 5}, {200, 986, 170, 5},
 {600, 1080, 210, 6}, {920, 1010, 160, 5}, {330, 1210, 220, 6}, {760, 1216, 190, 6}}
for _, g in ipairs(darks) do
  work(ellipse(g[1], g[2], g[3], g[4]):soften(g[4] * 1.4), {hand="glaze", pile=darkw, tool="filbert 26", coverage=0.9, pressure={0.3, 0.2}, clip=true})
end
print("water lights and darks")

--@ chunk 30
barkop = pile{{"lead white",8},{"raw umber",0.8},{"pale smalt",0.8}, medium=0.05}
for i, t in ipairs(grove) do
  b = brush{kind="filbert", width=gw[i] * 1.5, stiffness=0.6}
  b:load(barkop, 1.0)
  b:stroke(t.pts, {pressure={0.92, 0.88, 0.35}, orient="across", shake=0.5})
end
print("grove repainted")

--@ chunk 31
-- the feet of the trunks: solid, meeting the water without a taper
bb = brush{kind="filbert", width=7.0, stiffness=0.65}
feet = {{{136, 742}, {138, 760}, {139, 780}}, {{211, 748}, {211, 766}, {211, 786}},
        {{250, 736}, {250, 754}, {250, 776}}, {{330, 752}, {331, 770}, {330, 792}},
        {{360, 740}, {359, 758}, {360, 780}}, {{437, 746}, {437, 764}, {437, 788}},
        {{469, 742}, {468, 760}, {469, 782}}, {{540, 750}, {540, 768}, {540, 792}},
        {{573, 738}, {573, 756}, {573, 780}}, {{651, 746}, {651, 764}, {650, 784}},
        {{690, 744}, {689, 762}, {690, 782}}, {{768, 748}, {767, 766}, {768, 786}},
        {{851, 746}, {850, 762}, {850, 780}}}
for _, f in ipairs(feet) do
  bb:load(barkop, 1.0)
  bb:stroke(f, {pressure={0.5, 0.75, 0.8}, orient="across", shake=0.4})
end
-- shading and markings again, now that they can be seen
sb = brush{kind="filbert", width=3.0, stiffness=0.5}
for _, p in ipairs(shade) do
  sb:load(barkdk, 0.8)
  sb:stroke(p, {pressure={0.15, 0.45, 0.6}, orient="across", shake=1.3})
end
lb = brush{kind="round", width=2.0, point=0.5, stiffness=0.5}
for _, L in ipairs(lent) do
  lb:load(barkdk, 0.7)
  lb:touch(L[1], L[2], {pressure=0.55, drag={L[3] * 1.5, 0.05}, twist=0.3})
end
print("feet and markings")

--@ chunk 32
-- vapour over the grove: the tops go into it
mistv = pile{{"lead white",8},{"pale smalt",2}, medium=0.28}
work(rect(-30, 548, 1060, 130):soften(48), {hand="glaze", pile=mistv, tool="filbert 26", coverage=1.3, pressure={0.3, 0.2}, angle=0.02})
print("tops veiled")

--@ chunk 33
-- the trunks again, this time running unbroken from the vapour to the water
grove = {
 {pts={{145, 576}, {141, 622}, {137, 676}, {138, 730}, {139, 792}}, w=7},
 {pts={{205, 596}, {209, 642}, {212, 692}, {211, 740}, {211, 800}}, w=6},
 {pts={{262, 566}, {257, 608}, {251, 662}, {250, 712}, {250, 786}}, w=6.5},
 {pts={{312, 586}, {322, 634}, {331, 690}, {330, 740}, {330, 808}}, w=9},
 {pts={{372, 574}, {366, 616}, {360, 674}, {360, 718}, {360, 792}}, w=5.5},
 {pts={{425, 598}, {432, 642}, {438, 692}, {437, 732}, {437, 802}}, w=7.5},
 {pts={{480, 580}, {476, 620}, {471, 676}, {469, 720}, {469, 794}}, w=6},
 {pts={{528, 592}, {534, 638}, {540, 692}, {540, 736}, {540, 806}}, w=8.5},
 {pts={{585, 572}, {580, 614}, {575, 672}, {574, 716}, {573, 792}}, w=6.5},
 {pts={{640, 600}, {646, 644}, {651, 692}, {651, 736}, {650, 796}}, w=7},
 {pts={{700, 584}, {695, 626}, {691, 680}, {690, 724}, {690, 794}}, w=6},
 {pts={{762, 610}, {768, 648}, {769, 696}, {768, 730}, {768, 798}}, w=5.5},
 {pts={{845, 614}, {850, 652}, {851, 696}, {850, 730}, {850, 792}}, w=5}}
gw = {7, 6, 6.5, 9, 5.5, 7.5, 6, 8.5, 6.5, 7, 6, 5.5, 5}
barkop = pile{{"lead white",8},{"raw umber",1.4},{"pale smalt",0.8}, medium=0.05}
for i, t in ipairs(grove) do
  b = brush{kind="filbert", width=gw[i] * 1.5, stiffness=0.6}
  b:load(barkop, 1.0)
  b:stroke(t.pts, {pressure={0.55, 0.85, 0.9}, orient="across", shake=0.5})
end
print("trunks run into the water")

--@ chunk 34
barkdk2 = pile{{"raw umber",4},{"bone black",1.5},{"lead white",1}, medium=0.05}
-- the shaded flank of each trunk, and a wet dark foot
sb = brush{kind="filbert", width=3.6, stiffness=0.5}
for _, p in ipairs(shade) do
  sb:load(barkdk, 0.85)
  sb:stroke(p, {pressure={0.12, 0.42, 0.55}, orient="across", shake=1.2})
end
fb = brush{kind="filbert", width=5.0, stiffness=0.55}
wet = {{{137, 762}, {138, 786}, {139, 806}}, {{210, 768}, {211, 792}, {210, 810}},
        {{249, 754}, {250, 778}, {250, 798}}, {{329, 778}, {330, 802}, {330, 822}},
        {{359, 758}, {359, 782}, {360, 800}}, {{436, 772}, {437, 796}, {437, 816}},
        {{468, 760}, {468, 784}, {469, 802}}, {{539, 776}, {540, 800}, {540, 820}},
        {{572, 756}, {572, 780}, {573, 798}}, {{650, 772}, {651, 796}, {650, 812}},
        {{689, 760}, {689, 784}, {690, 802}}, {{767, 766}, {767, 790}, {768, 808}},
        {{850, 762}, {850, 784}, {850, 802}}}
for _, f in ipairs(wet) do
  fb:load(barkdk2, 0.9)
  fb:stroke(f, {pressure={0.35, 0.7, 0.2}, orient="across", shake=1.6})
end
print("flanks and wet feet")

--@ chunk 35
-- broken reflections
rb = brush{kind="filbert", width=4.5, stiffness=0.5}
refl = {
 {139, 812, 20, 0.85}, {138, 842, 16, 0.7}, {138, 870, 11, 0.55}, {137, 894, 7, 0.4},
 {211, 816, 21, 0.9}, {210, 848, 17, 0.75}, {210, 876, 12, 0.6}, {210, 900, 8, 0.45},
 {250, 804, 18, 0.85}, {250, 832, 14, 0.7}, {249, 858, 10, 0.55}, {249, 878, 6, 0.4},
 {330, 828, 26, 0.95}, {330, 862, 21, 0.8}, {331, 894, 15, 0.65}, {330, 922, 10, 0.5},
 {360, 806, 18, 0.85}, {359, 834, 14, 0.7}, {359, 860, 10, 0.55},
 {437, 822, 23, 0.92}, {437, 854, 18, 0.78}, {436, 884, 13, 0.62}, {436, 908, 8, 0.45},
 {469, 808, 19, 0.86}, {468, 838, 15, 0.72}, {468, 866, 11, 0.58}, {468, 888, 7, 0.42},
 {540, 826, 26, 0.95}, {540, 860, 21, 0.8}, {540, 892, 15, 0.65}, {539, 920, 10, 0.5},
 {573, 804, 18, 0.85}, {572, 832, 14, 0.7}, {572, 858, 10, 0.55},
 {651, 818, 20, 0.88}, {650, 850, 16, 0.74}, {650, 880, 11, 0.58}, {650, 902, 7, 0.42},
 {690, 808, 19, 0.86}, {689, 838, 15, 0.72}, {689, 866, 11, 0.58},
 {768, 814, 18, 0.85}, {767, 844, 14, 0.7}, {767, 870, 10, 0.55},
 {850, 808, 17, 0.84}, {850, 836, 13, 0.7}, {850, 862, 9, 0.52}}
for _, r in ipairs(refl) do
  rb:load(barkdk2, 0.9)
  rb:touch(r[1], r[2], {pressure=r[4], drag={0.5, r[3]}, twist=0.15})
end
print("reflections")

--@ chunk 36
rb = brush{kind="filbert", width=7.5, stiffness=0.5}
for _, r in ipairs(refl) do
  rb:load(barkdk2, 0.95)
  rb:touch(r[1], r[2], {pressure=r[4] * 1.15, drag={0.8, r[3] * 1.4}, twist=0.15})
end
print("reflections widened")

--@ chunk 37
-- the ripple that breaks the reflections
glint = pile{{"lead white",8},{"pale smalt",2},{"yellow ochre",1}, medium=0.22}
brk = {{120, 852, 150, 3}, {330, 878, 190, 3.5}, {560, 846, 160, 3}, {790, 884, 170, 3.5},
       {240, 916, 200, 4}, {640, 918, 180, 3.5}, {60, 946, 140, 4}, {450, 948, 210, 4},
       {850, 950, 140, 3.5}, {320, 982, 190, 4}, {700, 986, 170, 4}}
for _, g in ipairs(brk) do
  work(ellipse(g[1], g[2], g[3], g[4]):soften(g[4] * 1.2), {hand="glaze", pile=glint, tool="filbert 26", coverage=1.1, pressure={0.3, 0.2}, clip=true})
end
print("ripples over the reflections")

--@ chunk 38
earth = pile{{"raw umber",5},{"bone black",2},{"yellow ochre",1.5}, medium=0.05}
work(land, {hand="body", pile=earth, tool="filbert 14", coverage=2.4, pressure={0.5, 0.4},
            clip=true, edge="firm", length={50, 110}, angle=0.1, fill=true})
print("bank laid in")

--@ chunk 39
earth2 = pile{{"raw umber",6},{"bone black",3},{"pale smalt",0.8}, medium=0.04}
work(land, {hand="body", pile=earth2, tool="filbert 14", coverage=1.9, pressure={0.5, 0.38},
            clip=true, edge="soft", length={50, 110}, angle=0.1, fill=true})
print("bank cooled")

--@ chunk 40
earth3 = pile{{"raw umber",7},{"bone black",4}, medium=0.02}
work(land, {hand="body", pile=earth3, tool="filbert 14", coverage=2.6, pressure={0.6, 0.5},
            clip=true, edge="soft", length={50, 110}, angle=0.1, fill=true})
print("bank darkened")

--@ chunk 41
earth4 = pile{{"raw umber",6},{"red earth",2},{"bone black",2},{"pale smalt",0.5}, medium=0.02}
work(land, {hand="body", pile=earth4, tool="filbert 14", coverage=2.2, pressure={0.58, 0.48},
            clip=true, edge="soft", length={50, 110}, angle=0.1, fill=true})
print("bank warmed to brown")

--@ chunk 42
earth5 = pile{{"raw umber",8},{"red earth",1},{"bone black",2.5}, medium=0.02}
work(land, {hand="body", pile=earth5, tool="filbert 14", coverage=2.0, pressure={0.55, 0.45},
            clip=true, edge="soft", length={50, 110}, angle=0.1, fill=true})
print("bank toned")

--@ chunk 43
fb2 = brush{kind="filbert", width=13, stiffness=0.6}
fb2:load(barkop, 1.0)
fb2:stroke({{-20, 1342}, {110, 1326}, {240, 1316}, {352, 1299}, {438, 1262}},
           {pressure={0.75, 0.85, 0.8, 0.7, 0.45}, orient="across", shake=0.5})
print("fallen birch laid in")

--@ chunk 44
fb3 = brush{kind="filbert", width=9, stiffness=0.6}
fb3:load(barkop, 1.0)
fb3:stroke({{330, 1303}, {380, 1288}, {420, 1272}}, {pressure={0.9, 0.85, 0.6}, orient="across", shake=0.4})
-- the shadow it throws on the bank, and its dark markings
sd = brush{kind="filbert", width=7, stiffness=0.5}
sd:load(barkdk2, 0.8)
sd:stroke({{-20, 1358}, {110, 1342}, {240, 1332}, {352, 1315}}, {pressure={0.6, 0.55, 0.45, 0.15}, orient="across", shake=1.4})
lm = brush{kind="round", width=2.6, point=0.5, stiffness=0.5}
marks = {{40, 1332, 9}, {96, 1324, 7}, {158, 1318, 10}, {214, 1310, 6}, {268, 1306, 9},
         {312, 1300, 7}, {356, 1294, 8}, {396, 1286, 5}, {62, 1350, 6}, {186, 1332, 8}, {286, 1322, 5}}
for _, m in ipairs(marks) do
  lm:load(barkdk, 0.75)
  lm:touch(m[1], m[2], {pressure=0.5, drag={m[3], 0.08}, twist=0.4})
end
print("fallen birch finished")

--@ chunk 45
neardark = pile{{"raw umber",5},{"bone black",3},{"pale smalt",0.5}, medium=0.02}
ta = brush{kind="filbert", width=38, stiffness=0.7}
ta:load(neardark, 1.0)
ta:stroke({{3, 1389}, {25, 1120}, {48, 820}, {66, 460}, {77, 120}, {82, -12}},
          {pressure={0.9, 0.92, 0.88, 0.8, 0.7, 0.62}, orient="across", shake=0.4})
tb2 = brush{kind="filbert", width=22, stiffness=0.7}
tb2:load(neardark, 1.0)
tb2:stroke({{132, 1292}, {152, 1050}, {171, 800}, {186, 480}, {197, 120}, {201, -12}},
           {pressure={0.85, 0.85, 0.8, 0.72, 0.62, 0.55}, orient="across", shake=0.5})
print("near trunks")

--@ chunk 46
near2 = pile{{"raw umber",5},{"bone black",4}, medium=0.01}
segsA = {{{3, 1389}, {14, 1255}, {25, 1120}}, {{25, 1120}, {36, 970}, {48, 820}},
         {{48, 820}, {57, 640}, {66, 460}}, {{66, 460}, {72, 290}, {77, 120}}, {{77, 120}, {80, 50}, {82, -12}}}
ta = brush{kind="filbert", width=38, stiffness=0.75}
for _, s in ipairs(segsA) do
  ta:load(near2, 1.0)
  ta:stroke(s, {pressure={0.95, 0.98, 0.95}, orient="across", shake=0.3})
end
segsB = {{{132, 1292}, {142, 1171}, {152, 1050}}, {{152, 1050}, {161, 925}, {171, 800}},
         {{171, 800}, {178, 640}, {186, 480}}, {{186, 480}, {192, 300}, {197, 120}}, {{197, 120}, {199, 50}, {201, -12}}}
tb2 = brush{kind="filbert", width=23, stiffness=0.75}
for _, s in ipairs(segsB) do
  tb2:load(near2, 1.0)
  tb2:stroke(s, {pressure={0.9, 0.92, 0.88}, orient="across", shake=0.4})
end
print("near trunks, solid")

--@ chunk 47
near2 = pile{{"raw umber",5},{"bone black",4}, medium=0.01}
mA = ribbon({{3, 1389}, {25, 1120}, {48, 820}, {66, 460}, {77, 120}, {82, -12}}, 44)
work(mA, {hand="body", pile=near2, tool="filbert 30", coverage=3.0, pressure={0.85, 0.95},
          clip=true, edge="soft", length={40, 90}, dips={2, 0.95, 0.35}})
mB = ribbon({{132, 1292}, {152, 1050}, {171, 800}, {186, 480}, {197, 120}, {201, -12}}, 29)
work(mB, {hand="body", pile=near2, tool="filbert 22", coverage=3.0, pressure={0.8, 0.92},
          clip=true, edge="soft", length={35, 80}, dips={2, 0.95, 0.35}})
print("near trunks worked in")

--@ chunk 48
-- the silhouettes, drawn again with a loaded brush
defA = {-- left edge
  {{-14, 1389}, {4, 1120}, {28, 820}, {47, 460}, {59, 120}, {65, -12}},
  -- right edge
  {{22, 1389}, {47, 1120}, {70, 820}, {87, 460}, {97, 120}, {101, -12}}}
defB = {
  {{114, 1292}, {138, 1050}, {158, 800}, {174, 480}, {186, 120}, {192, -12}},
  {{150, 1290}, {166, 1050}, {184, 800}, {198, 480}, {208, 120}, {210, -12}}}
de = brush{kind="filbert", width=14, stiffness=0.8}
for _, e in ipairs(defA) do
  for i = 1, #e - 1 do
    de:load(near2, 1.0)
    de:stroke({e[i], e[i + 1]}, {pressure={0.95, 0.95}, orient="across", shake=0.25})
  end
end
de2 = brush{kind="filbert", width=10, stiffness=0.8}
for _, e in ipairs(defB) do
  for i = 1, #e - 1 do
    de2:load(near2, 1.0)
    de2:stroke({e[i], e[i + 1]}, {pressure={0.9, 0.9}, orient="across", shake=0.3})
  end
end
print("near trunk edges drawn")

--@ chunk 49
print(drying(60, 1000), drying(300, 900))
bgmix = pile{{"lead white",6},{"pale smalt",3},{"raw umber",0.8}, medium=0.12}
mA = ribbon({{3, 1389}, {25, 1120}, {48, 820}, {66, 460}, {77, 120}, {82, -12}}, 44)
mB = ribbon({{132, 1292}, {152, 1050}, {171, 800}, {186, 480}, {197, 120}, {201, -12}}, 29)
work(mA:grow(30) - mA, {hand="body", pile=bgmix, tool="filbert 16", coverage=2.6, pressure={0.6, 0.45},
                        clip=true, edge="soft", length={45, 100}, angle=1.45, fill=true})
work(mB:grow(22) - mB, {hand="body", pile=bgmix, tool="filbert 12", coverage=2.6, pressure={0.6, 0.45},
                        clip=true, edge="soft", length={40, 90}, angle=1.45, fill=true})
print("edges cut back to the mist")

--@ chunk 50
print("areas", mA:area(), mB:area())
work(mA, {hand="body", pile=near2, tool="filbert 34", coverage=3.2, pressure={0.92, 0.95},
          clip=true, edge="firm", length={50, 110}, angle=1.45, fill=true, dips={2, 0.95, 0.3}})
work(mB, {hand="body", pile=near2, tool="filbert 22", coverage=3.2, pressure={0.9, 0.92},
          clip=true, edge="firm", length={45, 100}, angle=1.45, fill=true, dips={2, 0.95, 0.3}})
print("near trunks relaid")

--@ chunk 51
print(wait(240))
print("sky", drying(500, 200), "water", drying(500, 900), "trunk", drying(60, 1000), "bank", drying(500, 1300))

--@ chunk 52
figdark = pile{{"raw umber",4},{"bone black",4},{"red earth",0.6}, medium=0.01}
fb4 = brush{kind="filbert", width=13, stiffness=0.6}
fb4:load(figdark, 1.0)
fb4:stroke({{756, 1211}, {759, 1226}, {763, 1244}}, {pressure={0.5, 0.8, 0.85}, orient="across", shake=0.3})
fb4:stroke({{768, 1211}, {765, 1226}, {762, 1244}}, {pressure={0.5, 0.8, 0.85}, orient="across", shake=0.3})
hd = brush{kind="round", width=9, point=0.6, stiffness=0.5}
hd:load(figdark, 0.9)
hd:touch(762, 1200, {pressure=0.8, twist=0.2})
-- her feet and the wet ground under her
ft = brush{kind="filbert", width=5, stiffness=0.6}
ft:load(figdark, 0.9)
ft:stroke({{756, 1244}, {755, 1258}}, {pressure={0.8, 0.9}, orient="across"})
ft:stroke({{768, 1244}, {769, 1258}}, {pressure={0.8, 0.9}, orient="across"})
-- the shawl over her shoulders
sh = brush{kind="filbert", width=7, stiffness=0.55}
sh:load(figdark, 0.85)
sh:stroke({{754, 1214}, {762, 1210}, {770, 1214}}, {pressure={0.7, 0.85, 0.7}, orient="across", shake=0.5})
print("the figure")

--@ chunk 53
fb5 = brush{kind="filbert", width=15, stiffness=0.7}
fb5:load(figdark, 1.0)
fb5:stroke({{754, 1213}, {753, 1226}, {752, 1242}}, {pressure={0.95, 0.95, 0.9}, orient="across", shake=0.2})
fb5:stroke({{770, 1213}, {771, 1226}, {772, 1242}}, {pressure={0.95, 0.95, 0.9}, orient="across", shake=0.2})
fb5:stroke({{754, 1213}, {762, 1210}, {770, 1213}}, {pressure={0.8, 0.9, 0.8}, orient="across", shake=0.3})
print("coat made solid")

--@ chunk 54
skyveil = pile{{"smalt",3},{"pale smalt",3},{"lead white",6}, medium=0.3}
work(skyab, {hand="glaze", pile=skyveil, tool="filbert 24", coverage=1.3, pressure={0.26, 0.18}, clip=true, angle=0.05})
wsm2 = pile{{"smalt",3},{"raw umber",2},{"lead white",3}, medium=0.2}
work(water, {hand="glaze", pile=wsm2, tool="filbert 24", coverage=1.3, pressure={0.28, 0.2}, clip=true, angle=0.02})
print("background re-glazed")

--@ chunk 55
-- more wood behind, so the grove reads as a wood
farbark = pile{{"lead white",5},{"pale smalt",2},{"raw umber",1.5}, medium=0.15}
far = {
 {pts={{186, 604}, {188, 650}, {190, 700}, {190, 770}}, w=3.4},
 {pts={{238, 592}, {240, 640}, {241, 692}, {241, 762}}, w=3.0},
 {pts={{288, 606}, {290, 650}, {292, 700}, {292, 774}}, w=3.6},
 {pts={{344, 590}, {346, 638}, {348, 690}, {348, 766}}, w=3.2},
 {pts={{400, 608}, {401, 652}, {402, 702}, {402, 776}}, w=3.8},
 {pts={{456, 598}, {457, 646}, {458, 696}, {458, 768}}, w=3.4},
 {pts={{506, 610}, {507, 652}, {508, 700}, {508, 772}}, w=3.6},
 {pts={{562, 596}, {561, 644}, {560, 694}, {560, 766}}, w=3.2},
 {pts={{618, 608}, {616, 652}, {614, 700}, {613, 772}}, w=3.4},
 {pts={{674, 592}, {671, 640}, {668, 692}, {666, 764}}, w=3.0},
 {pts={{732, 606}, {729, 650}, {726, 700}, {724, 770}}, w=3.4},
 {pts={{806, 598}, {803, 646}, {800, 696}, {798, 768}}, w=3.2},
 {pts={{898, 612}, {895, 654}, {892, 700}, {890, 772}}, w=3.0}}
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 1.9, stiffness=0.5}
  b:load(farbark, 0.9)
  b:stroke(t.pts, {pressure={0.4, 0.7, 0.85}, orient="across", shake=0.8})
end
print("distant wood laid in")

--@ chunk 56
-- the upper trunks, dark against the light of the sky
topsd = pile{{"raw umber",3},{"lead white",3},{"pale smalt",1}, medium=0.08}
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=gw[1] * 0 + t.w * 1.4, stiffness=0.5}
  b:load(topsd, 0.9)
  b:stroke({t.pts[1], t.pts[2]}, {pressure={0.75, 0.5}, orient="across", shake=0.9})
end
print("upper trunks darkened")

--@ chunk 57
-- the dark of the backlit tops, carried down into the pale
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.5, stiffness=0.5}
  b:load(topsd, 0.85)
  b:stroke({t.pts[1], t.pts[2], t.pts[3]}, {pressure={0.6, 0.45, 0.06}, orient="across", shake=1.1})
end
print("tops carried down")

--@ chunk 58
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.5, stiffness=0.6}
  b:load(barkop, 1.0)
  b:stroke({t.pts[3], t.pts[4], t.pts[5]}, {pressure={0.45, 0.8, 0.9}, orient="across", shake=0.5})
end
print("lower trunks made solid")

--@ chunk 59
refl2 = pile{{"raw umber",6},{"bone black",3},{"pale smalt",1}, medium=0.04}
rb = brush{kind="filbert", width=10, stiffness=0.55}
for _, r in ipairs(refl) do
  rb:load(refl2, 1.0)
  rb:touch(r[1], r[2], {pressure=r[4] * 1.2, drag={1.2, r[3] * 1.5}, twist=0.1})
end
print("reflections again")

--@ chunk 60
gm = mask(function(x, y) return 0 end)
for _, t in ipairs(grove) do gm = gm + ribbon(t.pts, t.w * 1.15) end
for _, t in ipairs(far) do gm = gm + ribbon(t.pts, t.w * 1.15) end
wateropen = water - gm
wfilm = pile{{"lead white",5},{"pale smalt",3},{"raw umber",2}, medium=0.05}
work(wateropen, {hand="body", pile=wfilm, tool="filbert 24", coverage=3.0, pressure={0.45, 0.35},
                 clip=true, edge="lost", length={110, 210}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
print("open water filmed")

--@ chunk 61
wdeep = pile{{"smalt",3},{"raw umber",3},{"lead white",2}, medium=0.14}
work(water * rect(-30, 980, 1060, 330):soften(80), {hand="glaze", pile=wdeep, tool="filbert 26", coverage=1.3, pressure={0.32, 0.24}, clip=true, angle=0.02})
work(water * rect(-30, 800, 1060, 240):soften(70), {hand="glaze", pile=wdeep, tool="filbert 26", coverage=0.8, pressure={0.3, 0.22}, clip=true, angle=0.02})
print("water graded")

--@ chunk 62
-- the glow knocked back to a pale warmth, and the ridge given back its distance
coolpale = pile{{"lead white",6},{"pale smalt",3},{"raw umber",1}, medium=0.2}
work(skyab * rect(-30, 430, 1060, 200):soften(60), {hand="glaze", pile=coolpale, tool="filbert 26", coverage=1.0, pressure={0.3, 0.2}, clip=true, angle=0.04})
ridgewash = pile{{"pale smalt",4},{"lead white",4},{"raw umber",1}, medium=0.3}
work(ridge_m, {hand="glaze", pile=ridgewash, tool="filbert 26", coverage=1.2, pressure={0.3, 0.2}, clip=true, angle=0.02})
glow2 = pile{{"lead white",7},{"yellow ochre",2}, medium=0.3}
q = ellipse(500, 574, 560, 40):soften(60)
work(q * skyab, {hand="glaze", pile=glow2, tool="filbert 26", coverage=1.0, pressure={0.3, 0.2}, clip=true})
print("sky settled")

--@ chunk 63
wdark = pile{{"smalt",4},{"raw umber",4},{"lead white",1.5}, medium=0.08}
work(water * rect(-30, 1000, 1060, 320):soften(90), {hand="body", pile=wdark, tool="filbert 22", coverage=1.8, pressure={0.4, 0.3}, clip=true, edge="lost", length={120, 220}, angle=0.02, fill=true})
work(water * rect(-30, 830, 1060, 230):soften(80), {hand="body", pile=wdark, tool="filbert 22", coverage=1.2, pressure={0.4, 0.3}, clip=true, edge="lost", length={120, 220}, angle=0.02, fill=true})
work(water * rect(-30, 700, 1060, 190):soften(70), {hand="body", pile=wdark, tool="filbert 22", coverage=0.8, pressure={0.4, 0.3}, clip=true, edge="lost", length={120, 220}, angle=0.02, fill=true})
print("near water darkened")

--@ chunk 64
print(drying(500, 1000))
blend(water, {angle=0.02})
print("water blended")

--@ chunk 65
refl3 = pile{{"raw umber",5},{"bone black",2},{"pale smalt",2}, medium=0.1}
rb = brush{kind="filbert", width=9, stiffness=0.5}
rstr = {
 {{139, 796}, {139, 850}, {140, 906}}, {{211, 804}, {211, 862}, {212, 926}},
 {{250, 792}, {250, 846}, {250, 898}}, {{330, 816}, {330, 880}, {331, 948}},
 {{360, 794}, {359, 848}, {359, 902}}, {{437, 812}, {437, 874}, {436, 940}},
 {{469, 796}, {468, 854}, {468, 910}}, {{540, 816}, {540, 880}, {539, 950}},
 {{573, 792}, {572, 846}, {572, 900}}, {{651, 806}, {651, 866}, {650, 930}},
 {{690, 798}, {689, 856}, {689, 912}}, {{768, 804}, {767, 858}, {767, 912}},
 {{850, 798}, {849, 852}, {849, 902}}}
for _, p in ipairs(rstr) do
  rb:load(refl3, 1.0)
  rb:stroke(p, {pressure={0.6, 0.45, 0.12}, orient="across", shake=1.8})
end
print("reflections as soft smears")

--@ chunk 66
-- long lights lying across the water
glint = pile{{"lead white",8},{"pale smalt",2},{"yellow ochre",1}, medium=0.25}
lights2 = {
 {170, 812, 230, 5}, {560, 830, 260, 5}, {880, 820, 180, 4},
 {120, 900, 280, 6}, {520, 906, 300, 6}, {880, 898, 200, 5},
 {260, 976, 260, 6}, {700, 990, 260, 6}, {60, 1050, 220, 6},
 {400, 1064, 300, 7}, {820, 1052, 240, 6}, {180, 1150, 300, 7},
 {600, 1176, 300, 7}, {900, 1140, 200, 6}}
for _, g in ipairs(lights2) do
  work(ellipse(g[1], g[2], g[3], g[4]):soften(g[4] * 1.6), {hand="glaze", pile=glint, tool="filbert 26", coverage=1.0, pressure={0.3, 0.2}, clip=true})
end
print("lights on the water")

--@ chunk 67
-- a calm luminous band of vapour for the wood to stand in
mistband = pile{{"lead white",8},{"pale smalt",2.5}, medium=0.12}
work(rect(-30, 596, 1060, 150):soften(50), {hand="body", pile=mistband, tool="filbert 22", coverage=2.2,
     pressure={0.42, 0.3}, edge="lost", length={120, 220}, angle=0.02, fill=true})
blend(rect(-30, 600, 1060, 140):soften(45), {angle=0.02})
print("mist band made luminous")

--@ chunk 68
-- the wood, darker than the vapour it stands in
graybirch = pile{{"raw umber",3},{"lead white",4},{"pale smalt",1.5}, medium=0.04}
for i, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.7, stiffness=0.6}
  b:load(graybirch, 1.0)
  b:stroke(t.pts, {pressure={0.55, 0.85, 0.95}, orient="across", shake=0.6})
end
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.5}
  b:load(graybirch, 0.8)
  b:stroke(t.pts, {pressure={0.4, 0.65, 0.8}, orient="across", shake=1.0})
end
print("wood repainted in grey")

--@ chunk 69
-- blunt feet where the trunks enter the water
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.7, stiffness=0.6}
  b:load(graybirch, 1.0)
  b:stroke({t.pts[5], {t.pts[5][1] + 1, t.pts[5][2] + 14}}, {pressure={0.9, 0.85}, orient="across", shake=0.4})
end
-- the reflections fade into the water instead of standing in it
watermid = pile{{"lead white",5},{"pale smalt",3},{"raw umber",2}, medium=0.08}
work(rect(-30, 880, 1060, 150):soften(70), {hand="body", pile=watermid, tool="filbert 20", coverage=1.3,
     pressure={0.4, 0.3}, edge="lost", length={110, 200}, angle=0.02, fill=true})
work(rect(-30, 745, 1060, 110):soften(55), {hand="body", pile=watermid, tool="filbert 20", coverage=1.0,
     pressure={0.38, 0.28}, edge="lost", length={110, 200}, angle=0.02, fill=true})
print("feet blunted, reflections settled")

--@ chunk 70
wfilm2 = pile{{"lead white",5},{"pale smalt",3},{"raw umber",2.5}, medium=0.12}
work(water, {hand="glaze", pile=wfilm2, tool="filbert 26", coverage=1.5, pressure={0.3, 0.22}, clip=true, angle=0.02})
work(water, {hand="glaze", pile=wfilm2, tool="filbert 26", coverage=1.2, pressure={0.28, 0.2}, clip=true, angle=-0.04})
blend(water, {angle=0.02})
print("water re-laid with long films")

--@ chunk 71
-- three definite steps of tone down the water
bA = pile{{"lead white",5},{"pale smalt",3},{"raw umber",1.5}, medium=0.1}
bB = pile{{"lead white",4},{"pale smalt",3},{"raw umber",2.5}, medium=0.1}
bC = pile{{"lead white",3},{"pale smalt",2.5},{"raw umber",4}, medium=0.1}
work(water * rect(-30, 700, 1060, 220):soften(90), {hand="glaze", pile=bA, tool="filbert 26", coverage=1.3, pressure={0.32, 0.24}, clip=true, angle=0.02})
work(water * rect(-30, 850, 1060, 240):soften(95), {hand="glaze", pile=bB, tool="filbert 26", coverage=1.3, pressure={0.34, 0.26}, clip=true, angle=0.02})
work(water * rect(-30, 1020, 1060, 280):soften(100), {hand="glaze", pile=bC, tool="filbert 26", coverage=1.3, pressure={0.36, 0.28}, clip=true, angle=0.02})
blend(water, {angle=0.02})
print("water stepped")

--@ chunk 72
obank = outline{pts = {{-30, 1250}, {130, 1276}, {265, 1303}, {405, 1292}, {560, 1252}, {700, 1229}, {855, 1239}, {1030, 1264}},
                 char="searching", amount=1.4, seed=11, edge=3}
land2 = obank:below()
print("land area", land:area(), land2:area())
earth5 = pile{{"raw umber",8},{"red earth",1},{"bone black",2.5}, medium=0.02}
work(land2, {hand="body", pile=earth5, tool="filbert 14", coverage=1.6, pressure={0.5, 0.4},
             clip=true, edge="soft", length={45, 100}, angle=0.1, fill=true})
print("bank edge redrawn")

--@ chunk 73
earth6 = pile{{"raw umber",8},{"bone black",3}, medium=0.02}
work(land2, {hand="body", pile=earth6, tool="filbert 16", coverage=2.4, pressure={0.6, 0.5},
             clip=true, edge="firm", length={60, 120}, angle=0.08, fill=true})
work(land2, {hand="body", pile=earth5, tool="filbert 12", coverage=1.2, pressure={0.55, 0.45},
             clip=true, edge="soft", length={40, 90}, angle=-0.15, fill=true})
blend(land2, {angle=0.1})
print("bank filled solid")

--@ chunk 74
barkop = pile{{"lead white",8},{"raw umber",1.4},{"pale smalt",0.8}, medium=0.05}
fb6 = brush{kind="filbert", width=21, stiffness=0.6}
fb6:load(barkop, 1.0)
fb6:stroke({{-30, 1328}, {100, 1316}, {230, 1305}, {340, 1290}, {446, 1260}},
           {pressure={0.85, 0.9, 0.88, 0.8, 0.55}, orient="across", shake=0.5})
sd = brush{kind="filbert", width=12, stiffness=0.5}
sd:load(barkdk2, 0.85)
sd:stroke({{-30, 1348}, {100, 1336}, {230, 1325}, {340, 1310}, {430, 1286}},
          {pressure={0.6, 0.55, 0.45, 0.3, 0.1}, orient="across", shake=1.5})
print("fallen birch laid")

--@ chunk 75
segs = {{{-30, 1328}, {60, 1321}, {160, 1313}}, {{160, 1313}, {250, 1306}, {330, 1295}},
        {{330, 1295}, {390, 1282}, {446, 1262}}}
fb7 = brush{kind="filbert", width=24, stiffness=0.7}
for _, s in ipairs(segs) do
  fb7:load(barkop, 1.0)
  fb7:stroke(s, {pressure={0.9, 0.95, 0.85}, orient="across", shake=0.4})
end
sd = brush{kind="filbert", width=13, stiffness=0.55}
sd:load(barkdk2, 0.9)
sd:stroke({{-30, 1350}, {120, 1340}, {270, 1329}, {360, 1316}, {440, 1290}},
          {pressure={0.65, 0.6, 0.5, 0.35, 0.1}, orient="across", shake=1.5})
print("log redrawn")

--@ chunk 76
mlog = ribbon({{-30, 1328}, {60, 1321}, {160, 1313}, {250, 1306}, {330, 1295}, {390, 1282}, {446, 1262}}, 25):soften(3)
work(mlog, {hand="body", pile=barkop, tool="filbert 16", coverage=3.0, pressure={0.8, 0.85},
            clip=true, edge="soft", length={50, 110}, angle=0.06, fill=true, dips={2, 0.95, 0.3}})
mlog2 = ribbon({{-30, 1350}, {120, 1340}, {270, 1329}, {360, 1316}, {440, 1290}}, 11):soften(4)
work(mlog2, {hand="body", pile=barkdk2, tool="filbert 10", coverage=2.4, pressure={0.75, 0.5},
             clip=true, edge="soft", length={40, 90}, angle=0.06, fill=true})
print("log worked")

--@ chunk 77
mshadow = ribbon({{-30, 1342}, {120, 1332}, {270, 1321}, {360, 1308}, {436, 1286}}, 20):soften(4)
work(mshadow, {hand="body", pile=earth6, tool="filbert 12", coverage=2.6, pressure={0.7, 0.55},
               clip=true, edge="soft", length={50, 100}, angle=0.06, fill=true})
print("log narrowed")

--@ chunk 78
-- the water must sit below the vapour in value, or the picture has no middle
w1 = pile{{"smalt",3},{"raw umber",2},{"lead white",4}, medium=0.08}
w2 = pile{{"smalt",3},{"raw umber",4},{"lead white",2}, medium=0.06}
work(water * rect(-30, 690, 1060, 210):soften(80), {hand="glaze", pile=w1, tool="filbert 26", coverage=1.3, pressure={0.32, 0.24}, clip=true, angle=0.02})
work(water * rect(-30, 860, 1060, 260):soften(90), {hand="glaze", pile=w2, tool="filbert 26", coverage=1.4, pressure={0.34, 0.26}, clip=true, angle=0.02})
work(water * rect(-30, 1050, 1060, 260):soften(90), {hand="glaze", pile=w2, tool="filbert 26", coverage=1.6, pressure={0.36, 0.28}, clip=true, angle=0.02})
blend(water, {angle=0.02})
print("water deepened")

--@ chunk 79
wdk = pile{{"raw umber",7},{"bone black",2},{"pale smalt",2}, medium=0.02}
work(water, {hand="body", pile=wdk, tool="filbert 20", coverage=1.7, pressure={0.5, 0.4},
             clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
print("water solid")

--@ chunk 80
wsm3 = pile{{"lead white",4},{"pale smalt",3},{"raw umber",3}, medium=0.12}
work(water, {hand="glaze", pile=wsm3, tool="filbert 26", coverage=2.0, pressure={0.32, 0.24}, clip=true, angle=0.02})
work(water, {hand="glaze", pile=wsm3, tool="filbert 26", coverage=1.5, pressure={0.3, 0.22}, clip=true, angle=-0.03})
blend(water, {angle=0.02})
print("water smoothed and lifted")

--@ chunk 81
coolw = pile{{"pale smalt",4},{"lead white",5},{"raw umber",1.5}, medium=0.1}
work(water, {hand="glaze", pile=coolw, tool="filbert 26", coverage=1.5, pressure={0.3, 0.22}, clip=true, angle=0.02})
coolw2 = pile{{"smalt",3},{"raw umber",2.5},{"pale smalt",2}, medium=0.08}
work(water * rect(-30, 960, 1060, 340):soften(110), {hand="glaze", pile=coolw2, tool="filbert 26", coverage=1.4, pressure={0.32, 0.24}, clip=true, angle=0.02})
blend(water, {angle=0.02})
print("water cooled")

--@ chunk 82
cfilm = pile{{"lead white",6},{"pale smalt",4},{"raw umber",1}, medium=0.02}
work(water, {hand="body", pile=cfilm, tool="filbert 18", coverage=2.6, pressure={0.45, 0.36},
             clip=true, edge="lost", length={100, 190}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
print("cool film over the water")

--@ chunk 83
blend(water, {angle=0.02})
coolp = pile{{"lead white",5},{"pale smalt",4},{"raw umber",2.5}, medium=0.06}
stipple(water, {pile=coolp, width=3.2, coverage=2.4, pressure={0.5, 0.3}, clip=true,
                dips={40, 0.9, 0.4}, feather=0.3, mix_jitter=0.3})
blend(water, {angle=-0.03})
print("water stippled")

--@ chunk 84
darkp = pile{{"lead white",3},{"pale smalt",3},{"raw umber",4}, medium=0.05}
stipple(water, {pile=darkp, width=3.2, coverage=2.0, pressure={0.5, 0.3}, clip=true,
                dips={40, 0.9, 0.4}, feather=0.3, mix_jitter=0.3})
blend(water, {angle=0.02})
print("water darkened by stipple")

--@ chunk 85
graybirch = pile{{"raw umber",3},{"lead white",4},{"pale smalt",1.5}, medium=0.04}
for i, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.7, stiffness=0.6}
  b:load(graybirch, 1.0)
  b:stroke(t.pts, {pressure={0.5, 0.85, 0.95}, orient="across", shake=0.6})
end
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.5}
  b:load(graybirch, 0.85)
  b:stroke(t.pts, {pressure={0.35, 0.6, 0.8}, orient="across", shake=1.0})
end
print("wood brought back")

--@ chunk 86
refl3 = pile{{"raw umber",5},{"bone black",2},{"pale smalt",2}, medium=0.06}
rb = brush{kind="filbert", width=9, stiffness=0.5}
for _, p in ipairs(rstr) do
  rb:load(refl3, 1.0)
  rb:stroke(p, {pressure={0.6, 0.45, 0.12}, orient="across", shake=1.8})
end
print("reflections back")

--@ chunk 87
reflm = pile{{"raw umber",4},{"bone black",1.5},{"pale smalt",3}, medium=0.08}
rb = brush{kind="filbert", width=10, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(reflm, 1.0)
  rb:stroke({{x, y - 4}, {x, y + 46}, {x, y + 96}}, {pressure={0.5, 0.35, 0.06}, orient="across", shake=1.9})
end
print("reflections from the feet")

--@ chunk 88
-- vapour lying on the water, so the far edge is not a wall
mveil = pile{{"lead white",8},{"pale smalt",2.5}, medium=0.2}
work(rect(-30, 592, 1060, 130):soften(46), {hand="glaze", pile=mveil, tool="filbert 26", coverage=1.1, pressure={0.3, 0.2}, angle=0.02})
print("veil on the water")

--@ chunk 89
-- the reflections had gone black; bring them back to grey
softg = pile{{"lead white",4},{"pale smalt",3},{"raw umber",2}, medium=0.12}
work(water * rect(-30, 780, 1060, 180):soften(60), {hand="glaze", pile=softg, tool="filbert 26", coverage=1.2, pressure={0.3, 0.22}, clip=true, angle=0.02})
blend(water * rect(-30, 780, 1060, 180):soften(60), {angle=0.02})
-- and the trunks reach a little further into the water
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.7, stiffness=0.6}
  b:load(graybirch, 1.0)
  b:stroke({{x, y - 6}, {x, y + 10}}, {pressure={0.9, 0.85}, orient="across", shake=0.4})
end
print("reflections softened")

--@ chunk 90
barkdk = pile{{"raw umber",3},{"bone black",1},{"lead white",2}, medium=0.1}
sb = brush{kind="filbert", width=3.4, stiffness=0.5}
for _, p in ipairs(shade) do
  sb:load(barkdk, 0.8)
  sb:stroke(p, {pressure={0.1, 0.4, 0.5}, orient="across", shake=1.4})
end
lb = brush{kind="round", width=2.2, point=0.5, stiffness=0.5}
for _, L in ipairs(lent) do
  lb:load(barkdk, 0.7)
  lb:touch(L[1], L[2], {pressure=0.5, drag={L[3] * 1.6, 0.05}, twist=0.3})
end
print("trunks marked")

--@ chunk 91
mA = ribbon({{3, 1389}, {25, 1120}, {48, 820}, {66, 460}, {77, 120}, {82, -12}}, 44)
mB = ribbon({{132, 1292}, {152, 1050}, {171, 800}, {186, 480}, {197, 120}, {201, -12}}, 29)
halo = (mA:grow(34) - mA) + (mB:grow(26) - mB)
stipple(halo * water, {pile=darkp, width=3.0, coverage=2.2, pressure={0.5, 0.3}, clip=true, feather=0.5, dips={40, 0.9, 0.4}})
stipple(halo * skyab, {pile=skyveil, width=3.0, coverage=1.9, pressure={0.5, 0.3}, clip=true, feather=0.5, dips={40, 0.9, 0.4}})
print("halos painted back into the air")

--@ chunk 92
print("halo", drying(30, 600), drying(30, 1000), "trunk", drying(60, 600), "water", drying(400, 900), "sky", drying(400, 300), "bank", drying(500, 1300))

--@ chunk 93
halo2 = (mA:grow(64) - mA) + (mB:grow(46) - mB)
work(halo2 * water, {hand="body", pile=darkp, tool="filbert 14", coverage=2.6, pressure={0.5, 0.4},
                      clip=true, edge="soft", length={50, 100}, angle=0.05, fill=true})
work(halo2 * skyab, {hand="body", pile=skyveil, tool="filbert 14", coverage=2.2, pressure={0.5, 0.4},
                     clip=true, edge="soft", length={50, 100}, angle=0.05, fill=true})
blend((halo2 * water):soften(30), {angle=0.05})
blend((halo2 * skyab):soften(30), {angle=0.05})
print("halos painted out")

--@ chunk 94
stipple(halo2 * water, {pile=darkp, width=3.2, coverage=2.0, pressure={0.5, 0.3}, clip=true,
                        dips={40, 0.9, 0.4}, feather=0.3, mix_jitter=0.3})
stipple(halo2 * water, {pile=softg, width=3.6, coverage=0.9, pressure={0.5, 0.3}, clip=true,
                        dips={40, 0.9, 0.4}, feather=0.5, mix_jitter=0.3})
print("water texture matched")

--@ chunk 95
-- the sky's clouds and warmth carried across behind the near trunks
c1a = ellipse(180, 96, 260, 34):soften(34)
c1b = ellipse(430, 78, 200, 26):soften(30)
c1c = ellipse(660, 112, 300, 30):soften(36)
c1d = ellipse(880, 86, 190, 22):soften(28)
work((c1a + c1b + c1c + c1d) * skyab, {hand="glaze", pile=cl1, tool="filbert 26", pressure={0.3, 0.2}, coverage=1.0})
c2a = ellipse(300, 262, 230, 20):soften(28)
c2b = ellipse(720, 232, 260, 17):soften(26)
c2c = ellipse(540, 316, 190, 14):soften(22)
c2e = ellipse(110, 258, 220, 18):soften(26)
work((c2a + c2b + c2c + c2e) * skyab, {hand="glaze", pile=cl1, tool="filbert 26", pressure={0.26, 0.16}, coverage=0.9})
g1 = ellipse(520, 470, 520, 40):soften(50)
g2 = ellipse(300, 520, 340, 34):soften(46)
g3 = ellipse(780, 528, 330, 32):soften(46)
g4 = ellipse(120, 508, 320, 32):soften(46)
work((g1 + g2 + g3 + g4) * skyab, {hand="glaze", pile=cl2, tool="filbert 26", pressure={0.3, 0.18}, coverage=1.0})
print("sky carried across")

--@ chunk 96
skyblue2 = pile{{"pale smalt",4},{"smalt",2},{"lead white",3}, medium=0.15}
work(skyab, {hand="glaze", pile=skyblue2, tool="filbert 26", coverage=1.5, pressure={0.3, 0.22}, clip=true, angle=0.04})
work(skyab, {hand="glaze", pile=skyblue2, tool="filbert 26", coverage=1.1, pressure={0.28, 0.2}, clip=true, angle=-0.05})
print("sky unified")

--@ chunk 97
-- the wood as dark verticals in the bright vapour
darkbark = pile{{"raw umber",5},{"lead white",2},{"pale smalt",1}, medium=0.04}
for i, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(darkbark, 1.0)
  b:stroke(t.pts, {pressure={0.5, 0.85, 0.95}, orient="across", shake=0.6})
end
midbark = pile{{"raw umber",3},{"lead white",4},{"pale smalt",1.5}, medium=0.05}
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.5}
  b:load(midbark, 0.9)
  b:stroke(t.pts, {pressure={0.35, 0.6, 0.8}, orient="across", shake=1.0})
end
print("wood darkened")

--@ chunk 98
-- the feet back to dark, so the trunks stand in the water
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.9, stiffness=0.6}
  b:load(darkbark, 1.0)
  b:stroke({{x, y - 18}, {x, y + 6}}, {pressure={0.7, 0.9}, orient="across", shake=0.5})
end
print("feet darkened")

--@ chunk 99
-- vapour drawn over the wood, so the edges are eaten into
mistv2 = pile{{"lead white",7},{"pale smalt",3}, medium=0.22}
work(rect(-30, 546, 1060, 90):soften(40), {hand="glaze", pile=mistv2, tool="filbert 26", coverage=1.3, pressure={0.28, 0.2}, angle=0.02})
work(rect(-30, 600, 1060, 110):soften(50), {hand="glaze", pile=mistv2, tool="filbert 26", coverage=0.8, pressure={0.24, 0.16}, angle=0.02})
print("vapour over the wood")

--@ chunk 100
cw1 = pile{{"pale smalt",4},{"lead white",4},{"raw umber",1.5}, medium=0.1}
cw2 = pile{{"pale smalt",4},{"lead white",3},{"raw umber",2.5}, medium=0.08}
cw3 = pile{{"smalt",3},{"pale smalt",2},{"lead white",2.5},{"raw umber",3}, medium=0.06}
work(water * rect(-30, 700, 1060, 200):soften(85), {hand="glaze", pile=cw1, tool="filbert 26", coverage=1.4, pressure={0.32, 0.24}, clip=true, angle=0.02})
work(water * rect(-30, 870, 1060, 230):soften(90), {hand="glaze", pile=cw2, tool="filbert 26", coverage=1.4, pressure={0.34, 0.26}, clip=true, angle=0.02})
work(water * rect(-30, 1060, 1060, 250):soften(95), {hand="glaze", pile=cw3, tool="filbert 26", coverage=1.5, pressure={0.36, 0.28}, clip=true, angle=0.02})
blend(water, {angle=0.02})
print("water cooled and graded")

--@ chunk 101
halo3 = (mA:grow(52) - mA) + (mB:grow(38) - mB)
erase(halo3, {strength=0.8, width=12})
print("halos lifted")

--@ chunk 102
near2 = pile{{"raw umber",5},{"bone black",4}, medium=0.01}
work(mA, {hand="body", pile=near2, tool="filbert 30", coverage=3.4, pressure={0.92, 0.95},
          clip=true, edge="firm", length={50, 110}, angle=1.45, fill=true, dips={2, 0.95, 0.3}})
work(mB, {hand="body", pile=near2, tool="filbert 22", coverage=3.4, pressure={0.9, 0.92},
          clip=true, edge="firm", length={45, 100}, angle=1.45, fill=true, dips={2, 0.95, 0.3}})
print("near trunks repainted")

--@ chunk 103
de = brush{kind="filbert", width=9, stiffness=0.85}
for _, e in ipairs(defA) do
  for i = 1, #e - 1 do
    de:load(near2, 1.0)
    de:stroke({e[i], e[i + 1]}, {pressure={0.95, 0.95}, orient="across", shake=0.2})
  end
end
for _, e in ipairs(defB) do
  for i = 1, #e - 1 do
    de:load(near2, 1.0)
    de:stroke({e[i], e[i + 1]}, {pressure={0.9, 0.9}, orient="across", shake=0.25})
  end
end
print("trunk edges redrawn")

--@ chunk 104
-- bark: the light that grazes the near trunks
barklit = pile{{"raw umber",2.5},{"lead white",3},{"pale smalt",1}, medium=0.06}
bt = brush{kind="filbert", width=7, stiffness=0.7}
streaksA = {{{10, 1360}, {22, 1160}, {36, 900}, {48, 640}, {60, 380}}, {{28, 1340}, {38, 1120}, {52, 840}, {64, 560}, {74, 260}}}
streaksB = {{{140, 1280}, {150, 1080}, {162, 860}, {174, 600}, {184, 300}}, {{158, 1270}, {166, 1060}, {178, 840}, {190, 580}}}
for _, s in ipairs(streaksA) do
  for i = 1, #s - 1 do
    bt:load(barklit, 0.8)
    bt:stroke({s[i], s[i + 1]}, {pressure={0.45, 0.3}, orient="across", shake=1.4})
  end
end
for _, s in ipairs(streaksB) do
  for i = 1, #s - 1 do
    bt:load(barklit, 0.7)
    bt:stroke({s[i], s[i + 1]}, {pressure={0.4, 0.25}, orient="across", shake=1.4})
  end
end
print("bark light on the near trunks")

--@ chunk 105
watercool = pile{{"lead white",4},{"pale smalt",4},{"smalt",1.5}, medium=0.05}
stipple(water, {pile=watercool, width=3.2, coverage=2.2, pressure={0.5, 0.3}, clip=true,
                dips={40, 0.9, 0.4}, feather=0.3, mix_jitter=0.3})
blend(water, {angle=0.02})
print("water cooled")

--@ chunk 106
waterdeep = pile{{"lead white",2},{"pale smalt",3},{"smalt",2},{"raw umber",2}, medium=0.05}
stipple(water * rect(-30, 950, 1060, 340):soften(90), {pile=waterdeep, width=3.4, coverage=2.0, pressure={0.5, 0.3},
           clip=true, dips={40, 0.9, 0.4}, feather=0.4, mix_jitter=0.3})
blend(water * rect(-30, 950, 1060, 340):soften(90), {angle=0.02})
print("near water deepened")

--@ chunk 107
-- the wood, dark, without a veil over it
darkbark = pile{{"raw umber",6},{"lead white",1.5},{"pale smalt",1}, medium=0.03}
for i, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(darkbark, 1.0)
  b:stroke(t.pts, {pressure={0.4, 0.8, 0.95}, orient="across", shake=0.6})
end
midbark = pile{{"raw umber",4},{"lead white",3},{"pale smalt",1.5}, medium=0.04}
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.5}
  b:load(midbark, 0.9)
  b:stroke(t.pts, {pressure={0.3, 0.55, 0.8}, orient="across", shake=1.0})
end
print("wood darkened again")

--@ chunk 108
-- the water brought down in the foreground
wdeepc = pile{{"lead white",2},{"pale smalt",2.5},{"smalt",2},{"raw umber",3}, medium=0.04}
work(water * rect(-30, 1080, 1060, 240):soften(95), {hand="body", pile=wdeepc, tool="filbert 18", coverage=1.6,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
work(water * rect(-30, 900, 1060, 220):soften(90), {hand="body", pile=wdeepc, tool="filbert 18", coverage=1.0,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
blend(water * rect(-30, 880, 1060, 440):soften(90), {angle=0.02})
print("foreground water deepened")

--@ chunk 109
-- a patch of lit water where she stands
lightw = pile{{"lead white",7},{"pale smalt",3}, medium=0.14}
work(ellipse(770, 1180, 190, 66):soften(70), {hand="glaze", pile=lightw, tool="filbert 26", coverage=1.2, pressure={0.3, 0.2}, clip=true, angle=0.02})
print("lit water by the shore")

--@ chunk 110
figdark = pile{{"raw umber",4},{"bone black",4},{"red earth",0.6}, medium=0.01}
fb8 = brush{kind="filbert", width=12, stiffness=0.75}
-- the coat, drawn as one shape: shoulders to hem
fb8:load(figdark, 1.0)
fb8:stroke({{756, 1213}, {752, 1228}, {750, 1248}}, {pressure={0.95, 0.95, 0.9}, orient="across", shake=0.2})
fb8:stroke({{768, 1213}, {772, 1228}, {774, 1248}}, {pressure={0.95, 0.95, 0.9}, orient="across", shake=0.2})
fb8:stroke({{754, 1214}, {762, 1210}, {770, 1214}}, {pressure={0.85, 0.95, 0.85}, orient="across", shake=0.3})
-- the head and the shawl
hd = brush{kind="round", width=8.5, point=0.6, stiffness=0.55}
hd:load(figdark, 0.95)
hd:touch(762, 1201, {pressure=0.85, twist=0.3})
-- the hem and the feet
ft = brush{kind="filbert", width=5, stiffness=0.7}
ft:load(figdark, 0.95)
ft:stroke({{756, 1244}, {756, 1258}}, {pressure={0.85, 0.9}, orient="across"})
ft:stroke({{768, 1244}, {769, 1258}}, {pressure={0.85, 0.9}, orient="across"})
ft:stroke({{750, 1249}, {774, 1249}}, {pressure={0.8, 0.8}, orient="across", shake=0.4})
print("the figure, drawn")

--@ chunk 111
-- the patch of lit water, softened
lightw2 = pile{{"lead white",7},{"pale smalt",3}, medium=0.14}
work(ellipse(770, 1176, 200, 70):soften(80), {hand="glaze", pile=lightw2, tool="filbert 26", coverage=1.3, pressure={0.3, 0.2}, clip=true, angle=0.02})
print("lit water softened")

--@ chunk 112
mfig = poly({{757, 1195}, {767, 1195}, {770, 1207}, {769, 1215}, {775, 1249}, {749, 1249}, {755, 1215}, {754, 1207}})
work(mfig, {hand="body", pile=figdark, tool="filbert 8", coverage=2.6, pressure={0.9, 0.85},
            clip=true, edge="soft", length={10, 26}, fill=true})
ft = brush{kind="filbert", width=4.5, stiffness=0.7}
ft:load(figdark, 0.95)
ft:stroke({{757, 1248}, {756, 1262}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
ft:stroke({{767, 1248}, {768, 1262}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
print("figure as a silhouette")

--@ chunk 113
-- vapour eating the tops of the wood, and softening where it meets the water
misttop = pile{{"lead white",8},{"pale smalt",2.5}, medium=0.2}
work(rect(-30, 556, 1060, 100):soften(44), {hand="glaze", pile=misttop, tool="filbert 26", coverage=1.4, pressure={0.3, 0.2}, angle=0.02})
work(rect(-30, 636, 1060, 90):soften(50), {hand="glaze", pile=misttop, tool="filbert 26", coverage=0.9, pressure={0.26, 0.18}, angle=0.02})
blend(rect(-30, 600, 1060, 130):soften(50), {angle=0.02})
print("the wood goes into the vapour")

--@ chunk 114
mutesky = pile{{"pale smalt",4},{"lead white",4}, medium=0.2}
work(skyab * rect(-30, 380, 1060, 240):soften(70), {hand="glaze", pile=mutesky, tool="filbert 26", coverage=1.1, pressure={0.3, 0.2}, clip=true, angle=0.04})
-- a narrower warmth kept down on the horizon
glow3 = pile{{"lead white",7},{"yellow ochre",1.6},{"red earth",0.6}, medium=0.25}
q1 = ellipse(500, 556, 520, 36):soften(56)
q2 = ellipse(200, 566, 300, 28):soften(50)
q3 = ellipse(830, 560, 280, 28):soften(50)
work((q1 + q2 + q3) * skyab, {hand="glaze", pile=glow3, tool="filbert 26", coverage=0.9, pressure={0.28, 0.18}, clip=true})
print("sky muted, warmth kept low")

--@ chunk 115
-- the puddle by the shore brought back to the water's tone
watermid2 = pile{{"lead white",3},{"pale smalt",3},{"smalt",1},{"raw umber",2.5}, medium=0.1}
work(ellipse(770, 1178, 220, 80):soften(85), {hand="glaze", pile=watermid2, tool="filbert 26", coverage=1.4, pressure={0.3, 0.22}, clip=true, angle=0.02})
work(ellipse(762, 1176, 120, 46):soften(52), {hand="glaze", pile=lightw2, tool="filbert 26", coverage=0.8, pressure={0.26, 0.18}, clip=true, angle=0.02})
print("shore water toned")

--@ chunk 116
-- the wood's reflections, once more, and the depth of the water
reflm2 = pile{{"raw umber",4},{"bone black",1.5},{"pale smalt",3}, medium=0.1}
rb = brush{kind="filbert", width=11, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(reflm2, 1.0)
  rb:stroke({{x, y + 10}, {x, y + 70}, {x, y + 128}}, {pressure={0.55, 0.4, 0.08}, orient="across", shake=2.2})
end
print("reflections again")

--@ chunk 117
work(ellipse(770, 1176, 210, 76):soften(70), {hand="body", pile=watermid2, tool="filbert 16", coverage=2.2,
     pressure={0.45, 0.36}, clip=true, edge="lost", length={90, 170}, angle=0.02, fill=true})
blend(ellipse(770, 1176, 200, 70):soften(60), {angle=0.02})
print("puddle covered")

--@ chunk 118
-- bands of deeper water, and light lying along them
deepb = pile{{"smalt",3},{"raw umber",3},{"pale smalt",2}, medium=0.1}
bands = {{180, 830, 260, 16}, {560, 862, 300, 18}, {860, 900, 220, 16}, {300, 940, 280, 20},
         {680, 986, 280, 20}, {120, 1040, 240, 22}, {470, 1090, 300, 24}, {860, 1140, 240, 22},
         {260, 1180, 280, 24}, {640, 1210, 260, 24}}
for _, b in ipairs(bands) do
  work(ellipse(b[1], b[2], b[3], b[4]):soften(b[4] * 1.5), {hand="glaze", pile=deepb, tool="filbert 26", coverage=0.9, pressure={0.3, 0.22}, clip=true})
end
print("bands of deeper water")

--@ chunk 119
-- the sparkle knocked back into a soft, even vapour
softveil = pile{{"lead white",6},{"pale smalt",4},{"raw umber",1}, medium=0.18}
work(rect(-30, 630, 1060, 260):soften(90), {hand="glaze", pile=softveil, tool="filbert 26", coverage=1.7, pressure={0.3, 0.22}, angle=0.02})
work(rect(-30, 850, 1060, 200):soften(80), {hand="glaze", pile=softveil, tool="filbert 26", coverage=1.4, pressure={0.3, 0.22}, angle=0.02})
blend(rect(-30, 620, 1060, 430):soften(80), {angle=0.02})
print("the sparkle evened out")

--@ chunk 120
-- the reflections back to a soft grey, touching the feet of the wood
reflsoft = pile{{"lead white",3},{"pale smalt",3},{"raw umber",2.5}, medium=0.12}
work(rect(-30, 770, 1060, 200):soften(60), {hand="glaze", pile=reflsoft, tool="filbert 26", coverage=1.5, pressure={0.3, 0.22}, clip=true, angle=0.02})
blend(rect(-30, 770, 1060, 200):soften(60), {angle=0.02})
print("reflections softened")

--@ chunk 121
darkbark = pile{{"raw umber",5},{"lead white",2},{"pale smalt",1.5}, medium=0.04}
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(darkbark, 1.0)
  b:stroke(t.pts, {pressure={0.45, 0.85, 0.95}, orient="across", shake=0.6})
end
midbark = pile{{"raw umber",3.5},{"lead white",3.5},{"pale smalt",1.5}, medium=0.05}
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.5}
  b:load(midbark, 0.9)
  b:stroke(t.pts, {pressure={0.3, 0.6, 0.85}, orient="across", shake=1.0})
end
print("wood brought back")

--@ chunk 122
reflm3 = pile{{"raw umber",4},{"pale smalt",3},{"bone black",1}, medium=0.1}
rb = brush{kind="filbert", width=11, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(reflm3, 1.0)
  rb:stroke({{x, y - 6}, {x, y + 48}, {x, y + 104}}, {pressure={0.6, 0.45, 0.1}, orient="across", shake=2.2})
end
print("reflections brought back")

--@ chunk 123
coolgray = pile{{"raw umber",4},{"pale smalt",3},{"lead white",2.5}, medium=0.04}
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(coolgray, 1.0)
  b:stroke(t.pts, {pressure={0.45, 0.85, 0.95}, orient="across", shake=0.6})
end
-- the feet: dark, full width, reaching down into the reflection
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.9, stiffness=0.6}
  b:load(darkbark, 1.0)
  b:stroke({{x, y - 20}, {x, y + 4}, {x, y + 30}}, {pressure={0.55, 0.9, 0.35}, orient="across", shake=0.8})
end
print("cool trunks, dark feet")

--@ chunk 124
refl4 = pile{{"lead white",3},{"pale smalt",3},{"raw umber",2}, medium=0.1}
rb = brush{kind="filbert", width=12, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(refl4, 1.0)
  rb:stroke({{x, y + 24}, {x, y + 70}, {x, y + 118}}, {pressure={0.5, 0.35, 0.08}, orient="across", shake=2.4})
end
print("reflections lightened")

--@ chunk 125
topveil = pile{{"lead white",8},{"pale smalt",2.5}, medium=0.22}
work(rect(-30, 548, 1060, 76):soften(38), {hand="glaze", pile=topveil, tool="filbert 26", coverage=1.3, pressure={0.3, 0.2}, angle=0.02})
print("trunk tops lost in the vapour")

--@ chunk 126
gr = brush{kind="round", width=2.2, point=0.85, stiffness=0.5}
grdark = pile{{"raw umber",4},{"bone black",2}, medium=0.04}
tufts = {
 {60, 1272, 22, -1.5}, {92, 1281, 17, -1.35}, {118, 1288, 25, -1.6}, {150, 1287, 15, -1.4},
 {176, 1290, 20, -1.5}, {212, 1301, 24, -1.55}, {244, 1305, 16, -1.3}, {286, 1305, 21, -1.5},
 {318, 1300, 14, -1.45}, {352, 1296, 19, -1.55}, {398, 1290, 23, -1.35}, {432, 1282, 15, -1.5},
 {470, 1272, 20, -1.6}, {512, 1260, 17, -1.4}, {556, 1250, 22, -1.5}, {600, 1240, 14, -1.45},
 {646, 1234, 19, -1.55}, {700, 1229, 16, -1.35}, {744, 1232, 21, -1.5}, {790, 1236, 15, -1.45},
 {836, 1238, 18, -1.6}, {884, 1244, 22, -1.4}, {930, 1252, 16, -1.5}, {972, 1258, 20, -1.55}}
for _, t in ipairs(tufts) do
  for _, d in ipairs({-0.28, 0.0, 0.3}) do
    gr:load(grdark, 0.75)
    gr:touch(t[1], t[2], {pressure=0.45, drag={t[3], t[4] + d}})
  end
end
print("grass along the bank")

--@ chunk 127
land3 = land2:grow(12)
earth6 = pile{{"raw umber",8},{"bone black",3}, medium=0.02}
work(land3, {hand="body", pile=earth6, tool="filbert 16", coverage=2.2, pressure={0.6, 0.5},
             clip=true, edge="soft", length={60, 120}, angle=0.08, fill=true})
blend(land3, {angle=0.1})
print("the bank made solid to its edge")

--@ chunk 128
-- the fallen birch, laid clear on the bank
mlog = ribbon({{-30, 1322}, {60, 1315}, {160, 1307}, {250, 1300}, {330, 1289}, {390, 1276}, {446, 1256}}, 17):soften(2)
work(mlog, {hand="body", pile=barkop, tool="filbert 12", coverage=2.6, pressure={0.8, 0.85},
            clip=true, edge="firm", length={50, 100}, angle=0.06, fill=true})
mshadow = ribbon({{-30, 1336}, {120, 1327}, {270, 1316}, {360, 1303}, {436, 1281}}, 12):soften(3)
work(mshadow, {hand="body", pile=earth6, tool="filbert 10", coverage=2.0, pressure={0.7, 0.5},
               clip=true, edge="soft", length={40, 90}, angle=0.06, fill=true})
print("log laid")

--@ chunk 129
logtone = pile{{"raw umber",3},{"lead white",3},{"pale smalt",2}, medium=0.14}
work(mlog:grow(3), {hand="glaze", pile=logtone, tool="filbert 20", coverage=1.2, pressure={0.3, 0.22}, clip=true, angle=0.06})
-- dark markings on the bark
lm = brush{kind="round", width=2.6, point=0.5, stiffness=0.5}
marks2 = {{52, 1315, 8}, {110, 1310, 6}, {168, 1304, 9}, {226, 1298, 6}, {286, 1292, 8}, {340, 1283, 6}, {392, 1270, 7}, {120, 1322, 5}, {240, 1310, 6}}
for _, m in ipairs(marks2) do
  lm:load(barkdk, 0.7)
  lm:touch(m[1], m[2], {pressure=0.45, drag={m[3], 0.08}, twist=0.4})
end
print("log toned and marked")

--@ chunk 130
-- a thin bright line of mist along the shore
shoreline = pile{{"lead white",7},{"pale smalt",3}, medium=0.2}
work((land3:grow(26) - land3:grow(-4)):soften(10), {hand="glaze", pile=shoreline, tool="filbert 20",
     coverage=0.9, pressure={0.28, 0.2}, clip=true, angle=0.06})
print("mist along the shore")

--@ chunk 131
work(land3, {hand="body", pile=earth6, tool="filbert 14", coverage=1.8, pressure={0.6, 0.5},
             clip=true, edge="firm", length={50, 100}, angle=0.08, fill=true})
print("bank back to its edge")

--@ chunk 132
-- mist on the shore, in patches only
patches = {{190, 1292, 120, 16}, {520, 1252, 140, 18}, {880, 1252, 130, 15}}
for _, p in ipairs(patches) do
  work(ellipse(p[1], p[2], p[3], p[4]):soften(p[4] * 1.5), {hand="glaze", pile=shoreline, tool="filbert 20", coverage=0.9, pressure={0.28, 0.2}, clip=true})
end
print("mist in patches on the shore")

--@ chunk 133
mfig = poly({{757, 1195}, {767, 1195}, {770, 1207}, {769, 1215}, {775, 1249}, {749, 1249}, {755, 1215}, {754, 1207}})
work(mfig, {hand="body", pile=figdark, tool="filbert 8", coverage=2.8, pressure={0.9, 0.88},
            clip=true, edge="soft", length={10, 26}, fill=true})
ft = brush{kind="filbert", width=4.5, stiffness=0.7}
ft:load(figdark, 0.95)
ft:stroke({{757, 1248}, {756, 1262}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
ft:stroke({{767, 1248}, {768, 1262}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
-- the light on her left, from the sky
rl = brush{kind="filbert", width=2.6, stiffness=0.6}
rl:load(pile{{"lead white",6},{"pale smalt",3}, medium=0.06}, 0.9)
rl:stroke({{753, 1216}, {751, 1230}, {750, 1244}}, {pressure={0.7, 0.6, 0.35}, orient="across", shake=0.4})
rl:stroke({{755, 1209}, {754, 1215}}, {pressure={0.6, 0.5}, orient="across", shake=0.4})
print("the figure, painted again")

--@ chunk 134
-- the log tightened and toned
mlog2 = ribbon({{-30, 1321}, {60, 1314}, {160, 1306}, {250, 1299}, {330, 1288}, {390, 1275}, {446, 1255}}, 13):soften(2)
work(mlog2, {hand="body", pile=logtone, tool="filbert 10", coverage=2.4, pressure={0.8, 0.85},
             clip=true, edge="firm", length={40, 90}, angle=0.06, fill=true})
-- the shadow beneath it, and the dark of the bank at the lower left
msh2 = ribbon({{-30, 1334}, {120, 1326}, {270, 1315}, {360, 1302}, {436, 1280}}, 13):soften(3)
work(msh2, {hand="body", pile=earth6, tool="filbert 10", coverage=2.2, pressure={0.7, 0.5},
            clip=true, edge="soft", length={40, 90}, angle=0.06, fill=true})
work(rect(-30, 1300, 400, 100):soften(25), {hand="body", pile=earth6, tool="filbert 14", coverage=1.8,
     pressure={0.6, 0.5}, clip=true, edge="soft", length={60, 110}, angle=0.06, fill=true})
print("log and lower bank")

--@ chunk 135
work(rect(-30, 1285, 120, 90):soften(22), {hand="body", pile=earth6, tool="filbert 12", coverage=2.0,
     pressure={0.6, 0.5}, clip=true, edge="soft", length={40, 80}, angle=0.1, fill=true})
print("lower left cleaned")

--@ chunk 136
-- a thin glaze over the whole canvas to bind the passages together
whole = rect(-30, -30, 1060, 1450)
unify = pile{{"lead white",4},{"pale smalt",3},{"raw umber",1.5}, medium=0.32}
work(whole, {hand="glaze", pile=unify, tool="filbert 26", coverage=0.6, pressure={0.26, 0.18}, angle=0.03})
print("unifying glaze")

--@ chunk 137
cwA = pile{{"pale smalt",4},{"cobalt blue",1},{"lead white",3},{"raw umber",2}, medium=0.12}
cwB = pile{{"pale smalt",3.5},{"cobalt blue",1.5},{"lead white",2.5},{"raw umber",3}, medium=0.1}
work(water * rect(-30, 700, 1060, 240):soften(90), {hand="glaze", pile=cwA, tool="filbert 26", coverage=1.3, pressure={0.32, 0.24}, clip=true, angle=0.02})
work(water * rect(-30, 900, 1060, 400):soften(95), {hand="glaze", pile=cwB, tool="filbert 26", coverage=1.5, pressure={0.34, 0.26}, clip=true, angle=0.02})
blend(water * rect(-30, 680, 1060, 620):soften(80), {angle=0.02})
print("water cooled toward evening blue")

--@ chunk 138
-- the wood, once more, and left alone after this
coolgray = pile{{"raw umber",4},{"pale smalt",3},{"lead white",2.5}, medium=0.04}
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(coolgray, 1.0)
  b:stroke(t.pts, {pressure={0.4, 0.85, 0.95}, orient="across", shake=0.6})
  x, y = t.pts[5][1], t.pts[5][2]
  b:load(darkbark, 1.0)
  b:stroke({{x, y - 18}, {x, y + 2}, {x, y + 26}}, {pressure={0.5, 0.9, 0.35}, orient="across", shake=0.8})
end
midbark = pile{{"raw umber",3.5},{"lead white",3.5},{"pale smalt",1.5}, medium=0.05}
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.5}
  b:load(midbark, 0.9)
  b:stroke(t.pts, {pressure={0.3, 0.6, 0.85}, orient="across", shake=1.0})
end
print("wood restated")

--@ chunk 139
refl4 = pile{{"lead white",3},{"pale smalt",3},{"raw umber",2}, medium=0.1}
rb = brush{kind="filbert", width=12, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(refl4, 1.0)
  rb:stroke({{x, y + 20}, {x, y + 68}, {x, y + 116}}, {pressure={0.5, 0.35, 0.08}, orient="across", shake=2.4})
end
-- and the white streaks on the near trunks toned down
nearveil = pile{{"raw umber",3},{"pale smalt",2},{"lead white",1.5}, medium=0.1}
work(mA:shrink(3) + mB:shrink(2), {hand="glaze", pile=nearveil, tool="filbert 20", coverage=0.8, pressure={0.26, 0.18}, clip=true, angle=1.45})
print("reflections and near trunks toned")

--@ chunk 140
-- the reflections were reading white: they must be darker than the water
refl5 = pile{{"raw umber",5},{"pale smalt",2.5},{"bone black",1}, medium=0.06}
rb = brush{kind="filbert", width=12, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(refl5, 1.0)
  rb:stroke({{x, y + 16}, {x, y + 62}, {x, y + 112}}, {pressure={0.6, 0.45, 0.1}, orient="across", shake=2.4})
end
print("reflections darkened")

--@ chunk 141
work(mA, {hand="body", pile=near2, tool="filbert 26", coverage=1.4, pressure={0.7, 0.6},
          clip=true, edge="soft", length={60, 110}, angle=1.45, fill=true})
work(mB, {hand="body", pile=near2, tool="filbert 20", coverage=1.4, pressure={0.7, 0.6},
          clip=true, edge="soft", length={50, 100}, angle=1.45, fill=true})
print("near trunks darkened")

--@ chunk 142
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.65}
  b:load(darkbark, 1.0)
  b:stroke({{x, y - 26}, {x, y - 4}, {x, y + 18}}, {pressure={0.75, 0.95, 0.9}, orient="across", shake=0.6})
end
print("feet made dark")

--@ chunk 143
soften2 = pile{{"lead white",3},{"pale smalt",3},{"cobalt blue",0.6},{"raw umber",2}, medium=0.14}
work(water * rect(-30, 740, 1060, 230):soften(70), {hand="glaze", pile=soften2, tool="filbert 26", coverage=0.9, pressure={0.28, 0.2}, clip=true, angle=0.02})
blend(water * rect(-30, 740, 1060, 230):soften(60), {angle=0.02})
print("the feet and reflections softened into the water")

--@ chunk 144
-- the light is from the right: warm the far right, cool the near left
warm = pile{{"lead white",6},{"yellow ochre",2},{"red earth",0.8}, medium=0.3}
work(ellipse(880, 480, 520, 420):soften(180), {hand="glaze", pile=warm, tool="filbert 26", coverage=0.9, pressure={0.28, 0.2}, clip=true, angle=0.04})
cool = pile{{"pale smalt",4},{"smalt",1.5},{"lead white",2}, medium=0.28}
work(ellipse(160, 1150, 560, 460):soften(200), {hand="glaze", pile=cool, tool="filbert 26", coverage=0.9, pressure={0.28, 0.2}, clip=true, angle=0.04})
print("warm light right, cool shade left")

--@ chunk 145
mute2 = pile{{"pale smalt",4},{"lead white",4}, medium=0.24}
work(ellipse(880, 460, 540, 420):soften(170), {hand="glaze", pile=mute2, tool="filbert 26", coverage=1.3, pressure={0.3, 0.22}, clip=true, angle=0.04})
work(skyab * rect(-30, 300, 1060, 340):soften(90), {hand="glaze", pile=mute2, tool="filbert 26", coverage=0.8, pressure={0.26, 0.18}, clip=true, angle=0.04})
print("the warmth muted")

--@ chunk 146
-- she steps up to the water's edge, where she can be seen against it
mfig2 = poly({{757, 1150}, {767, 1150}, {770, 1162}, {769, 1170}, {776, 1207}, {748, 1207}, {754, 1170}, {753, 1162}})
work(mfig2, {hand="body", pile=figdark, tool="filbert 8", coverage=3.0, pressure={0.9, 0.88},
             clip=true, edge="soft", length={10, 26}, fill=true})
ft = brush{kind="filbert", width=4.5, stiffness=0.7}
ft:load(figdark, 0.95)
ft:stroke({{757, 1206}, {756, 1222}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
ft:stroke({{768, 1206}, {769, 1222}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
rl = brush{kind="filbert", width=2.6, stiffness=0.6}
rl:load(pile{{"lead white",6},{"pale smalt",3}, medium=0.06}, 0.9)
rl:stroke({{752, 1172}, {750, 1186}, {749, 1202}}, {pressure={0.7, 0.6, 0.35}, orient="across", shake=0.4})
rl:stroke({{755, 1165}, {754, 1171}}, {pressure={0.6, 0.5}, orient="across", shake=0.4})
print("the figure, at the water's edge")

--@ chunk 147
ft:load(figdark, 0.95)
ft:stroke({{756, 1220}, {756, 1236}}, {pressure={0.9, 0.9}, orient="across", shake=0.3})
ft:stroke({{769, 1220}, {769, 1236}}, {pressure={0.9, 0.9}, orient="across", shake=0.3})
work(ellipse(758, 1258, 26, 20):soften(12), {hand="body", pile=earth6, tool="filbert 10", coverage=2.0,
     pressure={0.7, 0.6}, clip=true, edge="soft", length={20, 40}, fill=true})
print("feet on the ground, old mark covered")

--@ chunk 148
-- more in the upper sky, and a little weight at the top
cl3 = pile{{"smalt",4},{"pale smalt",3},{"lead white",2}, medium=0.3}
u1 = ellipse(240, 128, 300, 26):soften(34)
u2 = ellipse(600, 96, 260, 20):soften(30)
u3 = ellipse(900, 150, 220, 24):soften(32)
u4 = ellipse(420, 214, 300, 16):soften(26)
work((u1 + u2 + u3 + u4) * skyab, {hand="glaze", pile=cl3, tool="filbert 26", pressure={0.28, 0.18}, coverage=0.9})
deepsky = pile{{"smalt",5},{"pale smalt",3},{"lead white",1.5}, medium=0.25}
work(rect(-30, -30, 1060, 120):soften(70), {hand="glaze", pile=deepsky, tool="filbert 26", coverage=1.1, pressure={0.3, 0.2}, clip=true, angle=0.05})
print("upper sky filled")

--@ chunk 149
-- weight at the near edge of the bank
nearbank = pile{{"raw umber",8},{"bone black",4}, medium=0.02}
work(rect(-30, 1340, 1060, 90):soften(35), {hand="body", pile=nearbank, tool="filbert 16", coverage=1.6,
     pressure={0.6, 0.5}, clip=true, edge="soft", length={70, 130}, angle=0.06, fill=true})
print("the near edge darkened")

--@ chunk 150
mistfront = pile{{"lead white",8},{"pale smalt",2.5}, medium=0.22}
work(rect(-30, 596, 240, 130):soften(46), {hand="glaze", pile=mistfront, tool="filbert 26", coverage=1.2, pressure={0.28, 0.2}, angle=0.02})
work(rect(-30, 556, 240, 76):soften(40), {hand="glaze", pile=mistfront, tool="filbert 26", coverage=0.8, pressure={0.24, 0.16}, angle=0.02})
print("mist drawn across the near trunks")

--@ chunk 151
de = brush{kind="filbert", width=8, stiffness=0.9}
for _, e in ipairs(defA) do
  for i = 1, #e - 1 do
    de:load(near2, 1.0)
    de:stroke({e[i], e[i + 1]}, {pressure={0.95, 0.95}, orient="across", shake=0.15})
  end
end
for _, e in ipairs(defB) do
  for i = 1, #e - 1 do
    de:load(near2, 1.0)
    de:stroke({e[i], e[i + 1]}, {pressure={0.9, 0.9}, orient="across", shake=0.2})
  end
end
print("the near edges drawn firm")

--@ chunk 152
boat = pile{{"raw umber",5},{"bone black",2},{"lead white",1.5}, medium=0.03}
bl = brush{kind="filbert", width=7, stiffness=0.6}
bl:load(boat, 1.0)
bl:stroke({{378, 872}, {410, 879}, {444, 875}}, {pressure={0.35, 0.9, 0.35}, orient="across", shake=0.5})
bl:stroke({{382, 877}, {410, 884}, {440, 879}}, {pressure={0.4, 0.85, 0.4}, orient="across", shake=0.5})
rim = brush{kind="filbert", width=2.6, stiffness=0.6}
rim:load(pile{{"lead white",6},{"pale smalt",2}, medium=0.06}, 0.8)
rim:stroke({{382, 871}, {410, 877}, {440, 872}}, {pressure={0.3, 0.75, 0.3}, orient="across", shake=0.6})
ref = brush{kind="filbert", width=9, stiffness=0.45}
ref:load(boat, 0.9)
ref:stroke({{398, 892}, {412, 916}, {424, 938}}, {pressure={0.5, 0.3, 0.06}, orient="across", shake=2.2})
print("a small boat on the water")

--@ chunk 153
watfill = pile{{"lead white",3},{"pale smalt",3},{"cobalt blue",0.6},{"raw umber",2}, medium=0.12}
work(rect(380, 880, 70, 70):soften(18), {hand="glaze", pile=watfill, tool="filbert 20", coverage=1.3, pressure={0.3, 0.22}, clip=true, angle=0.02})
ref:load(boat, 0.85)
ref:stroke({{410, 890}, {412, 912}, {413, 932}}, {pressure={0.5, 0.3, 0.06}, orient="across", shake=2.4})
print("the boat's reflection set straight")

--@ chunk 154
work(rect(378, 886, 80, 70):soften(16), {hand="body", pile=watfill, tool="filbert 12", coverage=2.2,
     pressure={0.45, 0.36}, clip=true, edge="lost", length={40, 80}, angle=0.02, fill=true})
ref:load(boat, 0.85)
ref:stroke({{410, 894}, {411, 914}, {412, 930}}, {pressure={0.45, 0.28, 0.06}, orient="across", shake=2.4})
print("the mark under the boat cleared")

--@ chunk 155
work(rect(360, 850, 120, 110):soften(26), {hand="glaze", pile=watfill, tool="filbert 22", coverage=1.2, pressure={0.3, 0.22}, clip=true, angle=0.02})
blend(rect(370, 880, 100, 70):soften(24), {angle=0.02})
print("the water smoothed about the boat")

--@ chunk 156
de2 = brush{kind="filbert", width=23, stiffness=0.85}
de2:load(near2, 1.0)
de2:stroke({{132, 1296}, {126, 1340}, {120, 1389}}, {pressure={0.9, 0.92, 0.9}, orient="across", shake=0.3})
print("the second near trunk carried to the frame")

--@ chunk 157
-- the fallen birch taken down a little in value
logdown = pile{{"raw umber",4},{"lead white",2.5},{"pale smalt",2}, medium=0.12}
work(mlog2:grow(2), {hand="glaze", pile=logdown, tool="filbert 20", coverage=1.1, pressure={0.3, 0.22}, clip=true, angle=0.06})
print("log toned")

--@ chunk 158
print(wait(60 * 8))
print("sky", drying(500, 200), "mist", drying(500, 640), "water", drying(500, 900), "trunks", drying(250, 700), "bank", drying(500, 1300), "figure", drying(760, 1180))

--@ chunk 159
nearwood = {
 {pts={{238, 556}, {232, 620}, {228, 700}, {226, 786}, {226, 838}}, w=8},
 {pts={{402, 548}, {408, 616}, {414, 700}, {416, 780}, {416, 846}}, w=9},
 {pts={{612, 560}, {608, 622}, {604, 700}, {602, 782}, {602, 840}}, w=8}}
deepwood = pile{{"raw umber",6},{"lead white",1.8},{"pale smalt",1.5}, medium=0.04}
for _, t in ipairs(nearwood) do
  b = brush{kind="filbert", width=t.w * 1.9, stiffness=0.65}
  b:load(deepwood, 1.0)
  b:stroke(t.pts, {pressure={0.35, 0.8, 0.95}, orient="across", shake=0.7})
  x, y = t.pts[5][1], t.pts[5][2]
  b:load(refl5, 1.0)
  b:stroke({{x, y - 30}, {x, y + 6}, {x, y + 40}}, {pressure={0.4, 0.85, 0.7}, orient="across", shake=0.8})
  rb:load(refl5, 1.0)
  rb:stroke({{x, y + 34}, {x, y + 84}, {x, y + 132}}, {pressure={0.5, 0.3, 0.06}, orient="across", shake=2.4})
end
print("three nearer trees added to the wood")

--@ chunk 160
deepb2 = pile{{"smalt",3},{"raw umber",3},{"pale smalt",2}, medium=0.14}
for _, b in ipairs({{220, 1010, 320, 20}, {700, 1046, 300, 22}, {420, 1150, 340, 24}, {880, 1180, 240, 20}, {180, 1230, 300, 24}}) do
  work(ellipse(b[1], b[2], b[3], b[4]):soften(b[4] * 1.6), {hand="glaze", pile=deepb2, tool="filbert 26", coverage=1.0, pressure={0.3, 0.22}, clip=true})
end
glint2 = pile{{"lead white",8},{"pale smalt",2},{"yellow ochre",1}, medium=0.22}
for _, b in ipairs({{340, 1060, 240, 8}, {820, 1120, 220, 8}, {520, 1220, 260, 9}, {120, 1150, 200, 7}}) do
  work(ellipse(b[1], b[2], b[3], b[4]):soften(b[4] * 1.6), {hand="glaze", pile=glint2, tool="filbert 26", coverage=1.0, pressure={0.3, 0.2}, clip=true})
end
print("the near water given its bands")

--@ chunk 161
work(rect(180, 800, 480, 180):soften(60), {hand="glaze", pile=watfill, tool="filbert 24", coverage=1.1, pressure={0.28, 0.2}, clip=true, angle=0.02})
blend(rect(180, 800, 480, 180):soften(50), {angle=0.02})
print("the nearer trees softened into the water")

--@ chunk 162
refl5 = pile{{"raw umber",5},{"pale smalt",2.5},{"bone black",1}, medium=0.06}
rb = brush{kind="filbert", width=11, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(refl5, 1.0)
  rb:stroke({{x, y + 18}, {x, y + 62}, {x, y + 108}}, {pressure={0.5, 0.36, 0.08}, orient="across", shake=2.4})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(refl5, 1.0)
  rb:stroke({{x, y + 26}, {x, y + 76}, {x, y + 128}}, {pressure={0.55, 0.4, 0.1}, orient="across", shake=2.4})
end
print("the reflections restated")

--@ chunk 163
print(wait(60 * 24 * 6))
print("sky", drying(500, 200), "water", drying(500, 950), "bank", drying(500, 1300), "wood", drying(330, 700), "figure", drying(762, 1180))

--@ chunk 164
-- the three nearer trees, softened back into the vapour
nm = mask(function(x, y) return 0 end)
for _, t in ipairs(nearwood) do nm = nm + ribbon(t.pts, t.w * 1.1) end
misttone = pile{{"lead white",6},{"pale smalt",4},{"raw umber",1.5}, medium=0.3}
work(nm:grow(6), {hand="glaze", pile=misttone, tool="filbert 24", coverage=1.1, pressure={0.3, 0.22}, clip=true, angle=0.04})
print("the nearer trees softened")

--@ chunk 165
-- the reflections and the dark bands, eased back into the water
work(water * rect(-30, 800, 1060, 170):soften(60), {hand="glaze", pile=watfill, tool="filbert 26", coverage=1.2, pressure={0.3, 0.22}, clip=true, angle=0.02})
work(water * rect(-30, 980, 1060, 300):soften(80), {hand="glaze", pile=watfill, tool="filbert 26", coverage=1.1, pressure={0.3, 0.22}, clip=true, angle=0.02})
print("water eased")

--@ chunk 166
stipple(nm:grow(6), {pile=misttone, width=3.0, coverage=1.6, pressure={0.5, 0.3}, clip=true, feather=0.4, dips={40, 0.9, 0.4}})
print("the nearer trees evened")

--@ chunk 167
work(water * rect(-30, 800, 1060, 150):soften(60), {hand="glaze", pile=watfill, tool="filbert 26", coverage=0.9, pressure={0.28, 0.2}, clip=true, angle=0.03})
print("reflections softened further")

--@ chunk 168
midtree = pile{{"raw umber",5},{"lead white",3},{"pale smalt",1.5}, medium=0.03}
for _, t in ipairs(nearwood) do
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.65}
  b:load(midtree, 1.0)
  b:stroke(t.pts, {pressure={0.35, 0.85, 0.95}, orient="across", shake=0.8})
end
print("the nearer trees restated in a mid tone")

--@ chunk 169
stillw = pile{{"pale smalt",4},{"cobalt blue",0.8},{"lead white",3},{"raw umber",2.5}, medium=0.22}
work(water * rect(-30, 820, 1060, 460):soften(60), {hand="glaze", pile=stillw, tool="filbert 26", coverage=1.8, pressure={0.34, 0.26}, clip=true, angle=0.02})
blend(water * rect(-30, 820, 1060, 460):soften(50), {angle=0.02})
print("the near water laid quiet")

--@ chunk 170
midw = pile{{"pale smalt",3},{"cobalt blue",1},{"lead white",2},{"raw umber",4}, medium=0.16}
work(water * rect(-30, 900, 1060, 420):soften(80), {hand="glaze", pile=midw, tool="filbert 26", coverage=1.5, pressure={0.34, 0.26}, clip=true, angle=0.02})
work(water * rect(-30, 820, 1060, 160):soften(70), {hand="glaze", pile=stillw, tool="filbert 26", coverage=0.8, pressure={0.32, 0.24}, clip=true, angle=0.02})
blend(water * rect(-30, 800, 1060, 520):soften(60), {angle=0.02})
print("the near water brought to a middle value")

--@ chunk 171
darkfilm = pile{{"lead white",2},{"pale smalt",2.5},{"cobalt blue",0.8},{"raw umber",5}, medium=0.02}
work(water, {hand="body", pile=darkfilm, tool="filbert 20", coverage=2.4, pressure={0.5, 0.4},
            clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
print("the water laid dark")

--@ chunk 172
mistw = pile{{"lead white",6},{"pale smalt",3.5}, medium=0.3}
work(water * rect(-30, 600, 1060, 200):soften(80), {hand="glaze", pile=mistw, tool="filbert 26", coverage=1.8, pressure={0.32, 0.24}, clip=true, angle=0.02})
blend(water * rect(-30, 600, 1060, 220):soften(70), {angle=0.02})
print("the far water made misty again")

--@ chunk 173
-- the wood and its reflections once more, over the dark water
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(coolgray, 1.0)
  b:stroke({{x, t.pts[4][2]}, {x, y - 14}}, {pressure={0.7, 0.95}, orient="across", shake=0.6})
  b:load(darkbark, 1.0)
  b:stroke({{x, y - 18}, {x, y + 4}, {x, y + 26}}, {pressure={0.5, 0.9, 0.45}, orient="across", shake=0.7})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(midtree, 1.0)
  b:stroke({{x, t.pts[4][2]}, {x, y - 16}}, {pressure={0.7, 0.95}, orient="across", shake=0.6})
  b:load(darkbark, 1.0)
  b:stroke({{x, y - 20}, {x, y + 2}, {x, y + 24}}, {pressure={0.5, 0.9, 0.45}, orient="across", shake=0.7})
end
print("the wood's feet over the dark water")

--@ chunk 174
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(refl5, 1.0)
  rb:stroke({{x, y + 20}, {x, y + 64}, {x, y + 110}}, {pressure={0.5, 0.36, 0.08}, orient="across", shake=2.4})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb:load(refl5, 1.0)
  rb:stroke({{x, y + 28}, {x, y + 78}, {x, y + 130}}, {pressure={0.55, 0.4, 0.1}, orient="across", shake=2.4})
end
print("the reflections over the dark water")

--@ chunk 175
-- the wood, drawn again from the vapour to the water
pale1 = pile{{"lead white",6},{"pale smalt",3},{"raw umber",2}, medium=0.05}
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.7, stiffness=0.6}
  b:load(pale1, 1.0)
  b:stroke(t.pts, {pressure={0.4, 0.8, 0.9}, orient="across", shake=0.7})
end
mid1 = pile{{"lead white",3.5},{"pale smalt",2.5},{"raw umber",4}, medium=0.04}
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.5}
  b:load(mid1, 0.85)
  b:stroke(t.pts, {pressure={0.3, 0.6, 0.8}, orient="across", shake=1.1})
end
for _, t in ipairs(nearwood) do
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.65}
  b:load(mid1, 1.0)
  b:stroke(t.pts, {pressure={0.4, 0.85, 0.95}, orient="across", shake=0.8})
end
print("the whole wood redrawn")

--@ chunk 176
-- and its reflections: soft, tapering, directly under each trunk
reflp = pile{{"raw umber",5},{"pale smalt",3}, medium=0.12}
rb2 = brush{kind="filbert", width=12, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb2:load(reflp, 1.0)
  rb2:stroke({{x, y - 10}, {x, y + 40}, {x, y + 96}}, {pressure={0.45, 0.3, 0.05}, orient="across", shake=2.6})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb2:load(reflp, 1.0)
  rb2:stroke({{x, y - 12}, {x, y + 46}, {x, y + 108}}, {pressure={0.5, 0.34, 0.06}, orient="across", shake=2.6})
end
print("reflections redrawn")

--@ chunk 177
-- trunks with a base wider than their crown, as birches are
for _, t in ipairs(grove) do
  x = t.pts[5][1]
  topy = t.pts[1][2]
  midy = t.pts[3][2]
  boty = t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.1, stiffness=0.55}
  b:load(pale1, 1.0)
  b:stroke({{t.pts[1][1], topy + 6}, {t.pts[2][1], midy}}, {pressure={0.5, 0.75}, orient="across", shake=0.9})
  b = brush{kind="filbert", width=t.w * 1.7, stiffness=0.6}
  b:load(pale1, 1.0)
  b:stroke({{t.pts[3][1], midy}, {x, boty - 8}, {x, boty + 10}}, {pressure={0.7, 0.9, 0.85}, orient="across", shake=0.6})
end
print("trunks given a foot")

--@ chunk 178
-- reflections: the same tone a little deeper, continuous with the foot
reflq = pile{{"raw umber",4},{"pale smalt",4},{"lead white",1.5}, medium=0.14}
rb3 = brush{kind="filbert", width=14, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb3:load(reflq, 1.0)
  rb3:stroke({{x, y + 2}, {x, y + 44}, {x, y + 92}}, {pressure={0.55, 0.38, 0.06}, orient="across", shake=2.6})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb3:load(reflq, 1.0)
  rb3:stroke({{x, y + 2}, {x, y + 50}, {x, y + 104}}, {pressure={0.6, 0.42, 0.08}, orient="across", shake=2.6})
end
print("reflections lightened")

--@ chunk 179
-- the near water back to a middle, cooler value
nearw = pile{{"lead white",2},{"pale smalt",2},{"cobalt blue",1},{"raw umber",5}, medium=0.02}
work(water * rect(-30, 880, 1060, 430):soften(95), {hand="body", pile=nearw, tool="filbert 20", coverage=2.2,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
-- the bank, back to its edge
work(land3, {hand="body", pile=earth6, tool="filbert 14", coverage=2.2, pressure={0.6, 0.5},
             clip=true, edge="firm", length={55, 110}, angle=0.08, fill=true})
print("water and bank restored")

--@ chunk 180
-- she is back on the shore
mfig2 = poly({{757, 1150}, {767, 1150}, {770, 1162}, {769, 1170}, {776, 1207}, {748, 1207}, {754, 1170}, {753, 1162}})
work(mfig2, {hand="body", pile=figdark, tool="filbert 8", coverage=3.0, pressure={0.9, 0.88},
             clip=true, edge="soft", length={10, 26}, fill=true})
ft = brush{kind="filbert", width=4.5, stiffness=0.7}
ft:load(figdark, 0.95)
ft:stroke({{757, 1206}, {756, 1224}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
ft:stroke({{768, 1206}, {769, 1224}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
print("figure restored")

--@ chunk 181
coolv = pile{{"pale smalt",4},{"cobalt blue",1.5},{"lead white",3}, medium=0.12}
work(water, {hand="glaze", pile=coolv, tool="filbert 26", coverage=1.3, pressure={0.32, 0.24}, clip=true, angle=0.02})
print("the water cooled")

--@ chunk 182
-- pale birches give pale reflections
palerefl = pile{{"lead white",6},{"pale smalt",3}, medium=0.1}
rb4 = brush{kind="filbert", width=13, stiffness=0.45}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb4:load(palerefl, 1.0)
  rb4:stroke({{x, y + 2}, {x, y + 44}, {x, y + 92}}, {pressure={0.6, 0.4, 0.06}, orient="across", shake=2.6})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb4:load(palerefl, 1.0)
  rb4:stroke({{x, y + 2}, {x, y + 50}, {x, y + 104}}, {pressure={0.65, 0.44, 0.08}, orient="across", shake=2.6})
end
print("reflections pale again")

--@ chunk 183
film = pile{{"lead white",2},{"pale smalt",2},{"cobalt blue",1},{"raw umber",4.5}, medium=0.02}
work(water, {hand="body", pile=film, tool="filbert 22", coverage=2.3, pressure={0.5, 0.4},
             clip=true, edge="lost", length={130, 230}, angle=0.0, fill=true, dips={2, 0.95, 0.3}})
print("the water laid once, quietly")

--@ chunk 184
farw = pile{{"lead white",5},{"pale smalt",3},{"raw umber",1}, medium=0.16}
work(water * rect(-30, 600, 1060, 230):soften(90), {hand="glaze", pile=farw, tool="filbert 26", coverage=1.7, pressure={0.32, 0.24}, clip=true, angle=0.0})
print("the far water graded light")

--@ chunk 185
-- the wood's feet and their reflections, laid on the new water
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(pale1, 1.0)
  b:stroke({{x, y - 26}, {x, y - 4}, {x, y + 12}}, {pressure={0.8, 0.95, 0.9}, orient="across", shake=0.6})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(mid1, 1.0)
  b:stroke({{x, y - 28}, {x, y - 4}, {x, y + 12}}, {pressure={0.8, 0.95, 0.9}, orient="across", shake=0.6})
end
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb4:load(palerefl, 1.0)
  rb4:stroke({{x, y + 8}, {x, y + 50}, {x, y + 98}}, {pressure={0.6, 0.4, 0.06}, orient="across", shake=2.6})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb4:load(palerefl, 1.0)
  rb4:stroke({{x, y + 8}, {x, y + 56}, {x, y + 110}}, {pressure={0.65, 0.44, 0.08}, orient="across", shake=2.6})
end
print("wood and reflections on the new water")

--@ chunk 186
-- the trunks below the horizon, solid to the water
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.7, stiffness=0.6}
  b:load(pale1, 1.0)
  b:stroke({t.pts[3], t.pts[4], t.pts[5]}, {pressure={0.85, 0.9, 0.85}, orient="across", shake=0.6})
end
for _, t in ipairs(nearwood) do
  b = brush{kind="filbert", width=t.w * 1.8, stiffness=0.6}
  b:load(mid1, 1.0)
  b:stroke({t.pts[3], t.pts[4], t.pts[5]}, {pressure={0.85, 0.9, 0.85}, orient="across", shake=0.6})
end
for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.0, stiffness=0.5}
  b:load(mid1, 0.85)
  b:stroke({t.pts[2], t.pts[3], t.pts[4]}, {pressure={0.8, 0.85, 0.8}, orient="across", shake=0.8})
end
print("trunks into the water")

--@ chunk 187
-- reflections: unbroken, straight down, fading as they go
palerefl = pile{{"lead white",6},{"pale smalt",3}, medium=0.12}
rb5 = brush{kind="filbert", width=11, stiffness=0.55}
for _, t in ipairs(grove) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb5:load(palerefl, 1.0)
  rb5:stroke({{x, y + 2}, {x, y + 50}, {x, y + 104}}, {pressure={0.75, 0.5, 0.08}, orient="across", shake=1.2})
end
for _, t in ipairs(nearwood) do
  x, y = t.pts[5][1], t.pts[5][2]
  rb5:load(palerefl, 1.0)
  rb5:stroke({{x, y + 2}, {x, y + 58}, {x, y + 118}}, {pressure={0.8, 0.55, 0.1}, orient="across", shake=1.2})
end
print("reflections unbroken")

--@ chunk 188
-- the wood drawn back into the vapour, not standing out of it
mistknock = pile{{"lead white",6},{"pale smalt",4},{"raw umber",1}, medium=0.26}
work(rect(-30, 540, 1060, 420):soften(70), {hand="glaze", pile=mistknock, tool="filbert 26", coverage=1.1, pressure={0.3, 0.22}, angle=0.0})
print("the wood drawn into the mist")

--@ chunk 189
boat = pile{{"raw umber",5},{"bone black",2},{"lead white",1.5}, medium=0.03}
bl = brush{kind="filbert", width=7, stiffness=0.6}
bl:load(boat, 1.0)
bl:stroke({{322, 894}, {354, 901}, {388, 897}}, {pressure={0.35, 0.9, 0.35}, orient="across", shake=0.5})
bl:stroke({{326, 899}, {354, 906}, {384, 901}}, {pressure={0.4, 0.85, 0.4}, orient="across", shake=0.5})
rim = brush{kind="filbert", width=2.6, stiffness=0.6}
rim:load(pile{{"lead white",6},{"pale smalt",3}, medium=0.06}, 0.85)
rim:stroke({{326, 893}, {354, 899}, {384, 895}}, {pressure={0.35, 0.8, 0.35}, orient="across", shake=0.6})
brf = brush{kind="filbert", width=9, stiffness=0.45}
brf:load(boat, 0.9)
brf:stroke({{352, 912}, {353, 936}, {354, 956}}, {pressure={0.45, 0.3, 0.05}, orient="across", shake=2.2})
print("boat laid")

--@ chunk 190
-- two stakes and a clump of reeds in the near water
sk = brush{kind="filbert", width=7, stiffness=0.6}
sk:load(boat, 1.0)
sk:stroke({{176, 1108}, {177, 1146}}, {pressure={0.9, 0.95}, orient="across", shake=0.3})
sk:stroke({{196, 1122}, {197, 1158}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
sk2 = brush{kind="filbert", width=8, stiffness=0.45}
sk2:load(boat, 0.85)
sk2:stroke({{177, 1146}, {178, 1186}}, {pressure={0.45, 0.06}, orient="across", shake=2.4})
sk2:stroke({{197, 1158}, {198, 1196}}, {pressure={0.4, 0.05}, orient="across", shake=2.4})
rd = brush{kind="round", width=2.6, point=0.85, stiffness=0.5}
rd:load(pile{{"raw umber",4},{"bone black",2}, medium=0.04}, 0.85)
for _, r in ipairs({{120, 1180, 34, -1.5}, {132, 1186, 28, -1.42}, {144, 1190, 38, -1.55},
                    {158, 1192, 30, -1.35}, {172, 1190, 36, -1.5}, {188, 1186, 26, -1.45},
                    {206, 1182, 33, -1.55}, {222, 1178, 29, -1.4}}) do
  rd:touch(r[1], r[2], {pressure=0.5, drag={r[3], r[4]}})
end
print("stakes and reeds")

--@ chunk 191
mfig2 = poly({{757, 1150}, {767, 1150}, {770, 1162}, {769, 1170}, {776, 1207}, {748, 1207}, {754, 1170}, {753, 1162}})
work(mfig2, {hand="body", pile=figdark, tool="filbert 8", coverage=3.0, pressure={0.9, 0.88},
             clip=true, edge="soft", length={10, 26}, fill=true})
ft = brush{kind="filbert", width=4.5, stiffness=0.7}
ft:load(figdark, 0.95)
ft:stroke({{757, 1206}, {756, 1226}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
ft:stroke({{768, 1206}, {769, 1226}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
rl = brush{kind="filbert", width=2.4, stiffness=0.6}
rl:load(pile{{"lead white",6},{"pale smalt",3}, medium=0.06}, 0.85)
rl:stroke({{752, 1172}, {750, 1186}, {749, 1202}}, {pressure={0.7, 0.55, 0.3}, orient="across", shake=0.4})
print("the figure, for the last time")

--@ chunk 192
print(wait(60 * 10))
print("water", drying(500, 1000), "wood", drying(330, 700), "figure", drying(762, 1180), "sky", drying(500, 200), "bank", drying(500, 1300))

--@ chunk 193
-- what is where: sample the masks I still have
print("water area", water:area())
for _, y in ipairs({560, 600, 640, 700, 800, 900, 1000, 1100, 1200, 1280, 1330}) do
  local row = {}
  for _, x in ipairs({50, 250, 500, 750, 950}) do
    row[#row + 1] = string.format("%d:%.2f", x, water:at(x, y))
  end
  print("y=" .. y, table.concat(row, " "))
end
print("skyab area", skyab:area())
for _, y in ipairs({100, 300, 500, 580, 600}) do
  print("sky y=" .. y, string.format("%.2f %.2f", skyab:at(500, y), skyab:at(950, y)))
end
print("grove n", #grove, "far n", #far, "nearwood n", #nearwood)
print("grove[1]", grove[1].pts[1][1], grove[1].pts[1][2], "foot", grove[1].pts[5][1], grove[1].pts[5][2], "w", grove[1].w)
print("far[1]", far[1].pts[1][1], far[1].pts[1][2], "foot", far[1].pts[#far[1].pts][1], far[1].pts[#far[1].pts][2], "w", far[1].w)
print("mA area", mA:area(), "mB", mB:area(), "land3", land3:area())
print("drying: water", drying(500, 1000), "wood", drying(330, 700), "bank", drying(500, 1330), "sky", drying(600, 200), "fig", drying(762, 1180))

--@ chunk 194
-- 1. the water, laid quiet again: three bands, from the misty far water to the deep near water
farq = pile{{"lead white", 4}, {"pale smalt", 3.5}, {"raw umber", 1}, medium=0.2}
midq = pile{{"lead white", 3}, {"pale smalt", 3}, {"cobalt blue", 0.5}, {"raw umber", 2.5}, medium=0.14}
nearq = pile{{"lead white", 2.5}, {"pale smalt", 3}, {"cobalt blue", 1}, {"raw umber", 3}, medium=0.12}

work(water * rect(-30, 596, 1060, 200):soften(70), {hand="glaze", pile=farq, tool="filbert 26",
     coverage=1.1, pressure={0.3, 0.22}, clip=true, angle=0.02})
work(water * rect(-30, 740, 1060, 300):soften(80), {hand="glaze", pile=midq, tool="filbert 26",
     coverage=1.7, pressure={0.34, 0.26}, clip=true, angle=0.02})
work(water * rect(-30, 960, 1060, 340):soften(90), {hand="glaze", pile=nearq, tool="filbert 26",
     coverage=1.8, pressure={0.36, 0.28}, clip=true, angle=0.02})
print("the water laid quiet, in three bands")

--@ chunk 195
-- 2. a flat, even film over the whole water: the dabs knocked down and the value taken down with them
flatw = pile{{"lead white", 3}, {"pale smalt", 3}, {"cobalt blue", 0.8}, {"raw umber", 3}, medium=0.05}
work(water, {hand="body", pile=flatw, tool="filbert 14", coverage=1.8, pressure={0.5, 0.4},
     clip=true, edge="lost", length={90, 170}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
blend(water, {angle=0.02})
print("the water laid flat")

--@ chunk 196
-- 3. the value of the water built: light under the far shore, deeper and cooler toward the feet of the viewer
mistfar = pile{{"lead white", 5}, {"pale smalt", 3.5}, medium=0.16}
midw2 = pile{{"lead white", 3.2}, {"pale smalt", 3}, {"cobalt blue", 0.7}, {"raw umber", 2.6}, medium=0.1}
deepw = pile{{"lead white", 2.2}, {"pale smalt", 3}, {"cobalt blue", 1.2}, {"raw umber", 4}, medium=0.05}

work(water * rect(-30, 610, 1060, 150):soften(60), {hand="glaze", pile=mistfar, tool="filbert 26",
     coverage=1.0, pressure={0.3, 0.22}, clip=true, angle=0.02})
work(water * rect(-30, 760, 1060, 280):soften(90), {hand="body", pile=midw2, tool="filbert 14", coverage=1.0,
     pressure={0.46, 0.36}, clip=true, edge="lost", length={90, 170}, angle=0.02, fill=true})
work(water * rect(-30, 1000, 1060, 320):soften(110), {hand="body", pile=deepw, tool="filbert 14", coverage=1.5,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={90, 170}, angle=0.02, fill=true})
blend(water * rect(-30, 600, 1060, 740):soften(70), {angle=0.02})
print("the water given its depth")

--@ chunk 197
-- 4. more weight in the water: the near half taken down to a cool mid-dark
middeep = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"cobalt blue", 0.8}, {"raw umber", 3.6}, medium=0.05}
neardeep = pile{{"lead white", 1.6}, {"pale smalt", 2.6}, {"cobalt blue", 1.4}, {"raw umber", 5}, medium=0.02}
work(water * rect(-30, 740, 1060, 320):soften(90), {hand="body", pile=middeep, tool="filbert 14", coverage=1.3,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={90, 170}, angle=0.02, fill=true})
work(water * rect(-30, 980, 1060, 340):soften(110), {hand="body", pile=neardeep, tool="filbert 14", coverage=1.7,
     pressure={0.52, 0.42}, clip=true, edge="lost", length={90, 170}, angle=0.02, fill=true})
blend(water * rect(-30, 700, 1060, 640):soften(70), {angle=0.02})
print("the water given weight")

--@ chunk 198
-- 5. the water's striations: long, thin, horizontal marks, pale where the sky lies in it and dark where the wood does
rlp = pile{{"lead white", 6}, {"pale smalt", 3.5}, medium=0.12}
rld = pile{{"raw umber", 4}, {"pale smalt", 2}, medium=0.1}
rip = brush{kind="filbert", width=20, stiffness=0.5}
rpd = brush{kind="filbert", width=16, stiffness=0.55}

pales = {
 {{130, 656}, {300, 654}}, {{420, 662}, {640, 660}}, {{760, 656}, {960, 658}},
 {{200, 676}, {430, 674}}, {{560, 680}, {790, 678}}, {{860, 672}, {1010, 674}},
 {{90, 700}, {280, 698}}, {{380, 704}, {610, 702}}, {{700, 698}, {930, 700}},
 {{150, 726}, {360, 724}}, {{470, 730}, {700, 728}}, {{790, 724}, {980, 726}},
 {{250, 754}, {480, 752}}, {{600, 758}, {830, 756}}, {{110, 762}, {300, 760}},
 {{380, 788}, {600, 786}}, {{700, 792}, {920, 790}}, {{180, 800}, {400, 798}},
 {{520, 820}, {740, 818}}, {{860, 812}, {1020, 814}}, {{300, 840}, {520, 838}},
 {{640, 852}, {880, 850}}, {{120, 866}, {340, 864}}, {{430, 884}, {670, 882}},
 {{780, 892}, {1000, 890}}, {{200, 910}, {440, 908}}, {{560, 930}, {800, 928}},
 {{300, 962}, {560, 960}}, {{700, 972}, {940, 970}}, {{420, 1004}, {680, 1002}},
 {{120, 1030}, {380, 1028}}, {{560, 1052}, {820, 1050}}, {{240, 1090}, {520, 1088}},
 {{700, 1116}, {980, 1114}}, {{380, 1150}, {660, 1148}}, {{120, 1186}, {400, 1184}},
 {{620, 1216}, {900, 1214}}, {{300, 1250}, {580, 1248}}, {{760, 1258}, {1000, 1256}}}
darks = {
 {{240, 690}, {470, 688}}, {{640, 742}, {860, 740}}, {{100, 782}, {300, 780}},
 {{480, 806}, {700, 804}}, {{820, 848}, {1010, 846}}, {{260, 872}, {480, 870}},
 {{600, 946}, {840, 944}}, {{180, 986}, {400, 984}}, {{480, 1020}, {720, 1018}},
 {{860, 1064}, {1020, 1062}}, {{240, 1122}, {500, 1120}}, {{640, 1178}, {900, 1176}},
 {{360, 1212}, {620, 1210}}, {{840, 1238}, {1010, 1236}}}
for _, s in ipairs(pales) do
  rip:load(rlp, 0.8)
  rip:stroke({s[1], s[2]}, {pressure={0.3, 0.12}, orient="across", shake=0.5, swell={1, 1.15, 1}})
end
for _, s in ipairs(darks) do
  rpd:load(rld, 0.8)
  rpd:stroke({s[1], s[2]}, {pressure={0.3, 0.12}, orient="across", shake=0.6, swell={1, 1.15, 1}})
end
print("the water striated")

--@ chunk 199
-- 6. those marks read as driftwood: the water film laid over them again, quieter
work(water, {hand="body", pile=neardeep, tool="filbert 16", coverage=1.0, pressure={0.46, 0.36},
     clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
work(water * rect(-30, 620, 1060, 420):soften(90), {hand="body", pile=middeep, tool="filbert 16", coverage=1.0,
     pressure={0.46, 0.36}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
work(water * rect(-30, 600, 1060, 180):soften(70), {hand="glaze", pile=mistfar, tool="filbert 26", coverage=0.9,
     pressure={0.3, 0.22}, clip=true, angle=0.02})
blend(water * rect(-30, 600, 1060, 740):soften(70), {angle=0.02})
print("the driftwood covered")

--@ chunk 200
-- 7. the wood again: far ghosts first, then the grove, then the three nearer trees
ghost = pile{{"lead white", 6}, {"pale smalt", 4}, {"raw umber", 1.5}, medium=0.26}
birch = pile{{"lead white", 5}, {"pale smalt", 3.5}, {"raw umber", 2.2}, medium=0.08}
birch2 = pile{{"lead white", 3.5}, {"pale smalt", 3}, {"raw umber", 3.5}, medium=0.04}
foot = pile{{"raw umber", 5}, {"bone black", 1}, {"pale smalt", 1}, medium=0.03}

for _, t in ipairs(far) do
  b = brush{kind="filbert", width=t.w * 2.2, stiffness=0.45}
  b:load(ghost, 0.8)
  b:stroke(t.pts, {pressure={0.25, 0.5, 0.7}, orient="across", shake=1.2})
end
for _, t in ipairs(grove) do
  b = brush{kind="filbert", width=t.w * 1.5, stiffness=0.55}
  for i = 1, 3 do
    b:load(birch, 0.9)
    b:stroke({t.pts[i], t.pts[i + 1]}, {pressure={0.7, 0.85}, orient="across", shake=0.7})
  end
  x, y = t.pts[5][1], t.pts[5][2]
  b = brush{kind="filbert", width=t.w * 1.9, stiffness=0.6}
  b:load(birch, 0.9)
  b:stroke({t.pts[4], t.pts[5]}, {pressure={0.85, 0.9}, orient="across", shake=0.5})
  b:load(foot, 0.9)
  b:stroke({{x, y - 26}, {x, y - 4}, {x, y + 8}}, {pressure={0.3, 0.7, 0.35}, orient="across", shake=0.8})
end
for _, t in ipairs(nearwood) do
  b = brush{kind="filbert", width=t.w * 1.7, stiffness=0.6}
  for i = 1, 4 do
    b:load(birch2, 1.0)
    b:stroke({t.pts[i], t.pts[i + 1]}, {pressure={0.8, 0.9}, orient="across", shake=0.6})
  end
  x, y = t.pts[5][1], t.pts[5][2]
  b:load(foot, 1.0)
  b:stroke({{x, y - 30}, {x, y - 6}, {x, y + 10}}, {pressure={0.4, 0.8, 0.4}, orient="across", shake=0.7})
end
print("the wood drawn again, pale and standing")

--@ chunk 201
print("grove:")
for i, t in ipairs(grove) do
  print(i, "w", t.w, "top", t.pts[1][1], t.pts[1][2], "mid", t.pts[3][1], t.pts[3][2], "foot", t.pts[5][1], t.pts[5][2])
end
print("far:")
for i, t in ipairs(far) do
  print(i, "w", t.w, "top", t.pts[1][1], t.pts[1][2], "foot", t.pts[#t.pts][1], t.pts[#t.pts][2])
end
print("nearwood:")
for i, t in ipairs(nearwood) do
  print(i, "w", t.w, "top", t.pts[1][1], t.pts[1][2], "foot", t.pts[5][1], t.pts[5][2])
end

--@ chunk 202
-- 8. the wood and the old shore covered: the water film again, the vapour over the horizon
work(water * rect(-30, 640, 1060, 320):soften(70), {hand="body", pile=middeep, tool="filbert 16", coverage=1.5,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
blend(water * rect(-30, 620, 1060, 380):soften(70), {angle=0.02})
mistb = pile{{"lead white", 5}, {"pale smalt", 2.5}, {"yellow ochre", 0.8}, medium=0.2}
work(skyab * rect(-30, 552, 1060, 120):soften(46), {hand="glaze", pile=mistb, tool="filbert 26",
     coverage=0.8, pressure={0.28, 0.2}, clip=true, angle=0.03})
print("the wood and the shore covered")

--@ chunk 203
-- 9. all the old marks in the water buried under one opaque film, and the bands of value laid again on a clean ground
work(water, {hand="body", pile=flatw, tool="filbert 16", coverage=2.0, pressure={0.52, 0.42},
     clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
work(water * rect(-30, 980, 1060, 340):soften(110), {hand="body", pile=neardeep, tool="filbert 16", coverage=1.4,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={90, 170}, angle=0.02, fill=true})
work(water * rect(-30, 770, 1060, 260):soften(90), {hand="body", pile=middeep, tool="filbert 16", coverage=1.0,
     pressure={0.48, 0.38}, clip=true, edge="lost", length={90, 170}, angle=0.02, fill=true})
work(water * rect(-30, 600, 1060, 190):soften(70), {hand="glaze", pile=mistfar, tool="filbert 26", coverage=1.1,
     pressure={0.3, 0.22}, clip=true, angle=0.02})
blend(water, {angle=0.02})
print("the water cleaned and regraded")

--@ chunk 204
-- 10. the wood drawn as wood: thin birches, each with its own lean and height, the nearer ones with a few branches
ghost2 = pile{{"lead white", 6}, {"pale smalt", 4}, {"raw umber", 1.5}, medium=0.3}
barkp = pile{{"lead white", 4}, {"pale smalt", 3.5}, {"raw umber", 2.8}, medium=0.06}
barkn = pile{{"lead white", 3}, {"pale smalt", 3}, {"raw umber", 3.8}, medium=0.04}
brn = brush{kind="round", width=2.4, point=0.7, stiffness=0.5}

farw = {{186, 604, 190, 770}, {238, 592, 241, 762}, {288, 606, 292, 774}, {344, 590, 348, 766},
        {400, 608, 402, 776}, {456, 598, 458, 768}, {506, 610, 508, 772}, {562, 596, 560, 766},
        {618, 608, 613, 772}, {674, 592, 666, 764}, {732, 606, 724, 770}, {806, 598, 798, 768},
        {898, 612, 890, 772}}
for _, f in ipairs(farw) do
  b = brush{kind="filbert", width=5, stiffness=0.4}
  b:load(ghost2, 0.8)
  b:stroke({{f[1] + 2, 626}, {f[1] + 1, 690}, {f[2], f[3]}}, {pressure={0.2, 0.5, 0.7}, orient="across", shake=1.4})
end

-- {foot x, foot y, width, top y, lean at the top}
grovew = {{139, 792, 7, 596, 8}, {211, 800, 6, 628, -6}, {250, 786, 6.5, 588, 10}, {330, 808, 9, 604, -12},
          {360, 792, 5.5, 638, 5}, {437, 802, 7.5, 592, -9}, {469, 794, 6, 634, 7}, {540, 806, 8.5, 600, -14},
          {573, 792, 6.5, 622, 4}, {650, 796, 7, 612, -7}, {690, 794, 6, 640, 8}, {768, 798, 5.5, 618, -5},
          {850, 792, 5, 646, 6}}
for _, g in ipairs(grovew) do
  fx, fy, w, ty, ln = g[1], g[2], g[3], g[4], g[5]
  my = (ty + fy) * 0.5
  mx = fx + ln * 0.35
  b = brush{kind="filbert", width=w, stiffness=0.55}
  b:load(barkp, 0.9)
  b:stroke({{fx + ln, ty}, {mx, my}}, {pressure={0.15, 0.75}, orient="across", shake=0.8, ramps={0.12, 0.2}})
  b:load(barkp, 0.9)
  b:stroke({{mx, my}, {fx, fy - 6}}, {pressure={0.7, 0.95}, orient="across", shake=0.5})
end

-- the three nearer trees, darker, with fine bare branches
nearw = {{226, 838, 8, 566, -14}, {416, 846, 9, 548, 16}, {602, 840, 8, 574, -10}}
branches = {
  {{212, 566}, {196, 540}, {184, 524}}, {{212, 566}, {228, 542}, {240, 528}},
  {{220, 590}, {204, 572}, {192, 560}}, {{432, 548}, {452, 526}, {466, 512}},
  {{432, 548}, {416, 528}, {406, 514}}, {{424, 574}, {444, 558}, {460, 550}},
  {{592, 574}, {574, 552}, {562, 540}}, {{592, 574}, {610, 556}, {624, 546}},
  {{596, 600}, {612, 584}, {626, 576}}}
for _, n in ipairs(nearw) do
  fx, fy, w, ty, ln = n[1], n[2], n[3], n[4], n[5]
  my = (ty + fy) * 0.5
  mx = fx + ln * 0.35
  b = brush{kind="filbert", width=w * 1.15, stiffness=0.6}
  b:load(barkn, 1.0)
  b:stroke({{fx + ln, ty}, {mx, my}}, {pressure={0.15, 0.8}, orient="across", shake=0.7, ramps={0.12, 0.2}})
  b:load(barkn, 1.0)
  b:stroke({{mx, my}, {fx, fy - 8}}, {pressure={0.75, 0.95}, orient="across", shake=0.5})
end
for _, br in ipairs(branches) do
  brn:load(barkn, 0.9)
  brn:stroke(br, {pressure={0.8, 0.6, 0.15}, orient="across", shake=0.4, ramps={0.05, 0.5}})
end
print("the wood drawn: thin birches, the nearest with branches")

--@ chunk 205
-- 11. clean slate: the whole middle band covered again, the vapour above the shore and the water below it
work(water * rect(-30, 620, 1060, 320):soften(60), {hand="body", pile=middeep, tool="filbert 16", coverage=1.8,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
work(water * rect(-30, 600, 1060, 200):soften(70), {hand="glaze", pile=mistfar, tool="filbert 26", coverage=1.0,
     pressure={0.3, 0.22}, clip=true, angle=0.02})
blend(water * rect(-30, 600, 1060, 360):soften(60), {angle=0.02})
work(skyab * rect(-30, 540, 1060, 110):soften(40), {hand="glaze", pile=mistb, tool="filbert 26",
     coverage=0.9, pressure={0.28, 0.2}, clip=true, angle=0.03})
print("the band cleaned")

--@ chunk 206
-- 12. the sky band above the shore covered opaquely: it is all vapour there anyway
horiz = pile{{"lead white", 5}, {"pale smalt", 3}, {"yellow ochre", 0.7}, {"raw umber", 1.2}, medium=0.05}
work(rect(-30, 524, 1060, 110):soften(26), {hand="body", pile=horiz, tool="filbert 16", coverage=1.8,
     pressure={0.5, 0.4}, clip=true, edge="soft", length={110, 200}, angle=0.03, fill=true})
blend(rect(-30, 520, 1060, 120):soften(26), {angle=0.03})
print("the horizon band covered")

--@ chunk 207
-- 13. the last of the old marks buried, and the far shore drawn as a low soft line of wood in the haze
work(rect(-30, 460, 1060, 110):soften(30), {hand="body", pile=horiz, tool="filbert 16", coverage=1.3,
     pressure={0.46, 0.36}, clip=true, edge="lost", length={110, 200}, angle=0.03, fill=true})
shore = pile{{"raw umber", 4}, {"pale smalt", 2.5}, {"lead white", 1.5}, medium=0.16}
shb = brush{kind="filbert", width=9, stiffness=0.4}
ridge = {{-30, 618}, {90, 612}, {190, 616}, {290, 609}, {380, 614}, {470, 608}, {560, 613},
         {650, 607}, {740, 612}, {830, 609}, {920, 614}, {1010, 610}}
for i = 1, #ridge - 1 do
  shb:load(shore, 0.85)
  shb:stroke({ridge[i], ridge[i + 1]}, {pressure={0.5, 0.35}, orient="across"})
end
for _, c in ipairs({{150, 612, 16, 10}, {210, 610, 12, 8}, {340, 608, 20, 12}, {420, 611, 14, 9},
                    {500, 606, 18, 11}, {600, 609, 13, 8}, {700, 605, 17, 10}, {790, 608, 12, 8},
                    {880, 607, 15, 9}, {960, 611, 11, 7}}) do
  shb:load(shore, 0.8)
  shb:touch(c[1], c[2] - c[4] * 0.4, {pressure=0.5, drag={c[3], -c[4]}})
end
print("the far shore drawn")

--@ chunk 208
-- 14. the shore sausages buried; the far wood redrawn as a low grey band of wood in the haze, with no clean edges
work(rect(-30, 586, 1060, 50):soften(14), {hand="body", pile=horiz, tool="filbert 16", coverage=1.8,
     pressure={0.5, 0.4}, clip=true, edge="soft", length={80, 160}, angle=0.03, fill=true})
blend(rect(-30, 584, 1060, 54):soften(14), {angle=0.03})
haze = pile{{"lead white", 4}, {"pale smalt", 3}, {"raw umber", 2}, medium=0.3}
hb = brush{kind="filbert", width=13, stiffness=0.4}
farwood = {{-30, 616, 150}, {130, 611, 120}, {270, 614, 100}, {390, 609, 140}, {550, 613, 110},
           {680, 608, 130}, {830, 612, 120}, {960, 610, 80}}
for _, f in ipairs(farwood) do
  hb:load(haze, 0.8)
  hb:stroke({{f[1], f[2] + 2}, {f[1] + f[3] * 0.5, f[2] - 1}, {f[1] + f[3], f[2] + 1}},
            {pressure={0.35, 0.5, 0.3}, orient="across"})
end
hb:load(haze, 0.7)
for _, t in ipairs({{60, 606, 7}, {88, 603, 5}, {168, 604, 8}, {205, 600, 6}, {232, 605, 7},
                    {310, 601, 6}, {345, 599, 8}, {420, 603, 7}, {466, 600, 5}, {500, 604, 7},
                    {584, 601, 6}, {620, 598, 8}, {716, 600, 6}, {752, 603, 7}, {866, 601, 7},
                    {905, 599, 5}, {990, 603, 6}}) do
  hb:touch(t[1], t[2], {pressure=0.4, drag={1, -t[3]}})
end
print("the far wood in the haze")

--@ chunk 209
-- 15. the grove, drawn without a hand that wanders: thin trunks, thin at the top, standing in the water
barkp = pile{{"lead white", 3.5}, {"pale smalt", 3.5}, {"raw umber", 3.2}, medium=0.05}
barkn = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"raw umber", 4.2}, medium=0.03}
shadowb = pile{{"raw umber", 4}, {"pale smalt", 2}, {"lead white", 0.8}, medium=0.05}

grovew = {{139, 792, 7, 596, 8}, {211, 800, 6, 628, -6}, {250, 786, 6.5, 588, 10}, {330, 808, 9, 604, -12},
          {360, 792, 5.5, 638, 5}, {437, 802, 7.5, 592, -9}, {469, 794, 6, 634, 7}, {540, 806, 8.5, 600, -14},
          {573, 792, 6.5, 622, 4}, {650, 796, 7, 612, -7}, {690, 794, 6, 640, 8}, {768, 798, 5.5, 618, -5},
          {850, 792, 5, 646, 6}}
for _, g in ipairs(grovew) do
  fx, fy, w, ty, ln = g[1], g[2], g[3], g[4], g[5]
  b = brush{kind="filbert", width=w, stiffness=0.55}
  b:load(barkp, 0.95)
  b:stroke({{fx + ln, ty}, {fx + ln * 0.5, ty + (fy - ty) * 0.3}, {fx + ln * 0.2, (ty + fy) * 0.5}},
           {pressure={0.2, 0.6, 0.8}, orient="across", ramps={0.18, 0.3}})
  b:load(barkp, 0.95)
  b:stroke({{fx + ln * 0.2, (ty + fy) * 0.5}, {fx, fy - 10}}, {pressure={0.8, 0.95}, orient="across"})
end
-- a shadow side on some of them, so they are round and not cut out
for _, s in ipairs({{139, 792, 3.5, 620}, {330, 808, 4.5, 630}, {437, 802, 3.5, 618}, {540, 806, 4, 628},
                    {650, 796, 3.5, 640}, {250, 786, 3, 614}}) do
  b = brush{kind="filbert", width=2.4, stiffness=0.5}
  b:load(shadowb, 0.7)
  b:stroke({{s[1] + s[3], s[4]}, {s[1] + s[3] * 0.6, s[4] + 70}, {s[1] + s[3] * 0.3, s[2] - 8}},
           {pressure={0.25, 0.5, 0.3}, orient="across"})
end
print("the grove standing")

--@ chunk 210
-- 16. the spindle trunks covered; a clean vapour above the shore and clean water below it
work(rect(-30, 552, 1060, 76):soften(20), {hand="body", pile=horiz, tool="filbert 16", coverage=1.8,
     pressure={0.5, 0.4}, clip=true, edge="soft", length={80, 160}, angle=0.03, fill=true})
work(water * rect(-30, 618, 1060, 270):soften(24), {hand="body", pile=middeep, tool="filbert 16", coverage=1.8,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
blend(water * rect(-30, 616, 1060, 274):soften(24), {angle=0.02})
print("the band cleaned again")

--@ chunk 211
-- 17. the wood: ghosts behind, a grove in the middle, three nearer trees with their own weight
ghostg = pile{{"lead white", 5}, {"pale smalt", 4}, {"raw umber", 2}, medium=0.2}
barkp = pile{{"lead white", 3.5}, {"pale smalt", 3.5}, {"raw umber", 3.2}, medium=0.05}
barkn = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"raw umber", 4.2}, medium=0.03}
litb = pile{{"lead white", 5}, {"pale smalt", 2.5}, medium=0.1}
darkb = pile{{"raw umber", 4}, {"pale smalt", 2}, {"lead white", 0.8}, medium=0.05}
lenticel = pile{{"raw umber", 3}, {"bone black", 1.5}, medium=0.03}

ghosts = {{176, 780, 4.5, 5, 648, 3}, {262, 790, 4, 4.5, 662, -4}, {350, 776, 4.5, 5, 640, 5},
          {470, 786, 4, 4.5, 656, -3}, {585, 774, 4.5, 5, 646, 6}, {700, 792, 4, 4.5, 668, -5},
          {815, 778, 4.5, 5, 636, 4}, {905, 788, 4, 4.5, 660, -4}}
mids = {{148, 800, 6.5, 7, 596, 7}, {218, 812, 7, 7.5, 640, -8}, {300, 796, 6, 6.5, 570, 5},
        {412, 820, 7.5, 8, 606, -6}, {494, 802, 6.5, 7, 634, 9}, {556, 824, 7, 7.5, 584, -11},
        {640, 808, 6.5, 7, 622, 6}, {742, 818, 6, 6.5, 596, -7}, {820, 800, 5.5, 6, 646, 8}}
mains = {{240, 852, 9, 10.5, 556, -13}, {430, 860, 10, 11.5, 540, 15}, {610, 848, 9, 10.5, 568, -9}}

for _, t in ipairs(ghosts) do
  fx, fy, wt, wf, ty, ln = t[1], t[2], t[3], t[4], t[5], t[6]
  b = brush{kind="filbert", width=wt, stiffness=0.5}
  b:load(ghostg, 0.85)
  b:stroke({{fx + ln, ty}, {fx + ln * 0.4, (ty + fy) * 0.55}, {fx, fy}}, {pressure={0.4, 0.6, 0.7}, orient="across"})
end
for _, t in ipairs(mids) do
  fx, fy, wt, wf, ty, ln = t[1], t[2], t[3], t[4], t[5], t[6]
  for i = 1, 3 do
    y0 = ty + (fy - ty) * (i - 1) * 0.36
    y1 = ty + (fy - ty) * i * 0.36
    x0 = fx + ln * (1 - (y0 - ty) / (fy - ty))
    x1 = fx + ln * (1 - (y1 - ty) / (fy - ty))
    b = brush{kind="filbert", width=wt + (wf - wt) * ((i - 1) * 0.36), stiffness=0.55}
    b:load(barkp, 0.95)
    b:stroke({{x0, y0}, {x1, y1 + 3}}, {pressure={0.62, 0.72}, orient="across"})
  end
end
for _, t in ipairs(mains) do
  fx, fy, wt, wf, ty, ln = t[1], t[2], t[3], t[4], t[5], t[6]
  for i = 1, 4 do
    y0 = ty + (fy - ty) * (i - 1) * 0.28
    y1 = ty + (fy - ty) * i * 0.28
    x0 = fx + ln * (1 - (y0 - ty) / (fy - ty))
    x1 = fx + ln * (1 - (y1 - ty) / (fy - ty))
    b = brush{kind="filbert", width=wt + (wf - wt) * ((i - 1) * 0.28), stiffness=0.6}
    b:load(barkn, 1.0)
    b:stroke({{x0, y0}, {x1, y1 + 3}}, {pressure={0.66, 0.76}, orient="across"})
  end
end
print("the wood drawn")

--@ chunk 212
-- 18. the wood as a mass: a veil of vertical strokes where the grove stands, which also softens what is under it
massg = pile{{"lead white", 4}, {"pale smalt", 4}, {"raw umber", 2.5}, medium=0.14}
woodz = ellipse(510, 716, 420, 190):soften(74)
work(woodz, {hand="glaze", pile=massg, tool="filbert 22", coverage=2.0, pressure={0.34, 0.26},
     clip=true, angle=1.5708, fill=true, length={60, 140}})
stipple(woodz, {pile=massg, width=3.2, coverage=1.2, pressure={0.45, 0.28}, clip=true, feather=0.4, dips={40, 0.9, 0.4}})
print("the wood as a mass of vapour and stems")

--@ chunk 213
-- 19. the cottage cheese smoothed out: the veil laid with the brush alone and blended
massg = pile{{"lead white", 4}, {"pale smalt", 4}, {"raw umber", 2.8}, medium=0.16}
work(woodz, {hand="glaze", pile=massg, tool="filbert 24", coverage=1.6, pressure={0.32, 0.24},
     clip=true, angle=1.5708, length={80, 170}})
blend(woodz, {angle=1.5708})
print("the veil smoothed")

--@ chunk 214
-- 20. three trunks, each in a single stroke, to see what a single stroke gives
barkn = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"raw umber", 4.2}, medium=0.03}
tb = brush{kind="filbert", width=10, stiffness=0.6}
tb:load(barkn, 1.0)
tb:stroke({{227, 556}, {233, 700}, {240, 852}}, {pressure={0.62, 0.72, 0.78}, orient="across", ramps={0.05, 0.05}})
tb = brush{kind="filbert", width=11, stiffness=0.6}
tb:load(barkn, 1.0)
tb:stroke({{445, 540}, {438, 700}, {430, 860}}, {pressure={0.62, 0.72, 0.78}, orient="across", ramps={0.05, 0.05}})
tb = brush{kind="filbert", width=10, stiffness=0.6}
tb:load(barkn, 1.0)
tb:stroke({{601, 568}, {606, 700}, {610, 848}}, {pressure={0.62, 0.72, 0.78}, orient="across", ramps={0.05, 0.05}})
print("three trunks, single strokes")

--@ chunk 215
-- 21. the rest of the wood, every trunk in one stroke: the ghosts first, then the grove
ghostg = pile{{"lead white", 5}, {"pale smalt", 4}, {"raw umber", 2.2}, medium=0.18}
barkp = pile{{"lead white", 3.5}, {"pale smalt", 3.5}, {"raw umber", 3.2}, medium=0.05}
barkn = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"raw umber", 4.2}, medium=0.03}
ghosts = {{176, 780, 5, 648, 3}, {262, 790, 4.5, 662, -4}, {350, 776, 5, 640, 5},
          {470, 786, 4.5, 656, -3}, {585, 774, 5, 646, 6}, {700, 792, 4.5, 668, -5},
          {815, 778, 5, 636, 4}, {905, 788, 4.5, 660, -4}}
mids = {{148, 800, 7, 596, 7}, {218, 812, 7.5, 640, -8}, {300, 796, 6.5, 570, 5},
        {412, 820, 8, 606, -6}, {494, 802, 7, 634, 9}, {556, 824, 7.5, 584, -11},
        {640, 808, 7, 622, 6}, {742, 818, 6.5, 596, -7}, {820, 800, 6, 646, 8}}
for _, t in ipairs(ghosts) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w, stiffness=0.5}
  b:load(ghostg, 0.85)
  b:stroke({{fx + ln, ty}, {fx + ln * 0.4, (ty + fy) * 0.55}, {fx, fy}}, {pressure={0.5, 0.62, 0.66}, orient="across", ramps={0.08, 0.08}})
end
for _, t in ipairs(mids) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w, stiffness=0.55}
  b:load(barkp, 0.95)
  b:stroke({{fx + ln, ty}, {fx + ln * 0.4, (ty + fy) * 0.55}, {fx, fy}}, {pressure={0.5, 0.66, 0.72}, orient="across", ramps={0.06, 0.06}})
end
print("the wood drawn")

--@ chunk 216
-- 22. the water in front of the grove given its weight again, and the vapour drawn over the tops of the wood
work(water * rect(-30, 880, 1060, 420):soften(110), {hand="body", pile=neardeep, tool="filbert 16", coverage=1.5,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
work(water * rect(-30, 780, 1060, 180):soften(80), {hand="body", pile=middeep, tool="filbert 16", coverage=0.9,
     pressure={0.48, 0.38}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
blend(water * rect(-30, 760, 1060, 560):soften(80), {angle=0.02})
work(skyab * rect(-30, 516, 1060, 200):soften(70), {hand="glaze", pile=massg, tool="filbert 24",
     coverage=1.3, pressure={0.3, 0.22}, clip=true, angle=0.03})
print("the water weighted, the tops veiled")

--@ chunk 217
-- 23. the grove given presence: the nearer trunks darker, dark sides, and drowned brush along their feet
barkn = pile{{"lead white", 2.2}, {"pale smalt", 2.8}, {"raw umber", 4.6}, medium=0.02}
barkd = pile{{"raw umber", 4}, {"pale smalt", 2.2}, {"lead white", 1}, medium=0.02}
brushdark = pile{{"raw umber", 4.5}, {"bone black", 2}, medium=0.02}
mains = {{227, 852, 556, 11, -13}, {445, 860, 540, 12, 15}, {601, 848, 568, 11, -9}}
for _, t in ipairs(mains) do
  b = brush{kind="filbert", width=t[4], stiffness=0.6}
  b:load(barkn, 1.0)
  b:stroke({{t[1] + t[5], t[3] + 60}, {t[1] + t[5] * 0.5, (t[3] + t[2]) * 0.55}, {t[1], t[2]}},
           {pressure={0.72, 0.78, 0.82}, orient="across", ramps={0.06, 0.06}})
  b = brush{kind="filbert", width=2.6, stiffness=0.5}
  b:load(barkd, 0.8)
  b:stroke({{t[1] + t[5] * 0.5 - t[4] * 0.32, t[3] + 90}, {t[1] - t[4] * 0.28, t[2] - 30}},
           {pressure={0.35, 0.6}, orient="across"})
end
mids = {{148, 800, 596, 7, 7}, {218, 812, 640, 7.5, -8}, {300, 796, 570, 6.5, 5},
        {412, 820, 606, 8, -6}, {494, 802, 634, 7, 9}, {556, 824, 584, 7.5, -11},
        {640, 808, 622, 7, 6}, {742, 818, 596, 6.5, -7}, {820, 800, 646, 6, 8}}
for _, t in ipairs(mids) do
  fx, fy, ty, w, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w * 0.6, stiffness=0.55}
  b:load(barkn, 0.9)
  b:stroke({{fx + ln, ty + 40}, {fx + ln * 0.4, (ty + fy) * 0.6}, {fx, fy}}, {pressure={0.4, 0.55, 0.6}, orient="across", ramps={0.1, 0.1}})
end
-- drowned brush along the feet of the wood
bt = brush{kind="filbert", width=4, stiffness=0.45}
for _, s in ipairs({{132, 796, 26}, {186, 786, 20}, {236, 802, 30}, {262, 788, 18}, {312, 794, 24},
                    {372, 800, 22}, {424, 818, 28}, {468, 802, 18}, {506, 800, 24}, {566, 822, 26},
                    {614, 846, 22}, {652, 806, 26}, {706, 800, 18}, {752, 816, 24}, {800, 806, 20},
                    {832, 798, 22}, {868, 790, 18}}) do
  bt:load(brushdark, 0.75)
  bt:touch(s[1], s[2], {pressure=0.55, drag={s[3], -2}})
end
print("the grove given presence")

--@ chunk 218
-- 24. the drowned brush along the feet of the wood: clumps of short strokes, not dashes on the water
brushdark = pile{{"raw umber", 4.5}, {"bone black", 2}, medium=0.02}
bt = brush{kind="filbert", width=3.4, stiffness=0.4}
clumps = {{136, 798}, {190, 788}, {240, 804}, {266, 790}, {316, 796}, {376, 802},
          {428, 820}, {472, 804}, {510, 802}, {570, 824}, {614, 848}, {656, 808},
          {710, 802}, {756, 818}, {804, 808}, {836, 800}, {872, 792}}
angles = {-2.0, -1.72, -1.4, -1.1, -2.3, -0.9}
for _, c in ipairs(clumps) do
  for i = 1, #angles do
    bt:load(brushdark, 0.8)
    bt:touch(c[1] + (i - 3) * 2.6, c[2] - 2, {pressure=0.5, drag={11 + (i % 3) * 3, angles[i]}})
  end
end
print("the brush along the feet of the wood")

--@ chunk 219
-- 25. the brush redrawn at a size that reads: a soft dark wisp with a few flicks of stem out of it
brushdark = pile{{"raw umber", 4.5}, {"bone black", 2}, medium=0.02}
bw = brush{kind="filbert", width=7, stiffness=0.35}
bf = brush{kind="filbert", width=2.8, stiffness=0.45}
clumps = {{136, 798, 46}, {190, 788, 32}, {240, 804, 52}, {266, 790, 30}, {316, 796, 40},
          {376, 802, 34}, {428, 820, 48}, {472, 804, 30}, {510, 802, 42}, {570, 824, 44},
          {614, 848, 34}, {656, 808, 44}, {710, 802, 30}, {756, 818, 40}, {804, 808, 32},
          {836, 800, 36}, {872, 792, 30}}
for _, c in ipairs(clumps) do
  bw:load(brushdark, 0.7)
  bw:stroke({{c[1] - c[3] * 0.5, c[2]}, {c[1] + c[3] * 0.5, c[2] + 2}}, {pressure={0.3, 0.5}, orient="across", ramps={0.25, 0.25}})
  for k = 1, 3 do
    bf:load(brushdark, 0.8)
    bf:touch(c[1] - 9 + k * 9, c[2] - 1, {pressure=0.55, drag={5, -11 - k * 3}})
  end
end
print("the brush again, bigger")

--@ chunk 220
-- 26. the lily pads painted out; the water where the wood stands laid clean, and the feet of the trunks put back
work(water * rect(-30, 770, 1060, 110):soften(34), {hand="body", pile=middeep, tool="filbert 14", coverage=1.8,
     pressure={0.48, 0.38}, clip=true, edge="lost", length={100, 180}, angle=0.02, fill=true})
blend(water * rect(-30, 766, 1060, 118):soften(34), {angle=0.02})
barkn = pile{{"lead white", 2.2}, {"pale smalt", 2.8}, {"raw umber", 4.6}, medium=0.02}
feet = {{240, 852, 11}, {445, 860, 12}, {601, 848, 11}, {148, 800, 7}, {218, 812, 7.5}, {300, 796, 6.5},
        {412, 820, 8}, {494, 802, 7}, {556, 824, 7.5}, {640, 808, 7}, {742, 818, 6.5}, {820, 800, 6}}
for _, f in ipairs(feet) do
  b = brush{kind="filbert", width=f[3], stiffness=0.6}
  b:load(barkn, 1.0)
  b:stroke({{f[1], f[2] - 34}, {f[1], f[2] + 6}}, {pressure={0.74, 0.8}, orient="across"})
end
print("the pads gone, the feet back")

--@ chunk 221
-- 27. the last of the pads knocked back, and the trunks carried down to the water in one stroke each
work(water * rect(-30, 760, 1060, 130):soften(30), {hand="glaze", pile=middeep, tool="filbert 20",
     coverage=0.8, pressure={0.34, 0.26}, clip=true, angle=0.02})
barkn = pile{{"lead white", 2.2}, {"pale smalt", 2.8}, {"raw umber", 4.6}, medium=0.02}
trunks = {{148, 800, 7, 7}, {218, 812, 7.5, -8}, {300, 796, 6.5, 5}, {412, 820, 8, -6},
          {494, 802, 7, 9}, {556, 824, 7.5, -11}, {640, 808, 7, 6}, {742, 818, 6.5, -7},
          {820, 800, 6, 8}, {240, 852, 11, -13}, {445, 860, 12, 15}, {601, 848, 11, -9}}
for _, t in ipairs(trunks) do
  fx, fy, w, ln = t[1], t[2], t[3], t[4]
  y0 = fy - 110
  b = brush{kind="filbert", width=w, stiffness=0.6}
  b:load(barkn, 1.0)
  b:stroke({{fx + ln * 0.7, y0}, {fx + ln * 0.3, y0 + 55}, {fx, fy + 6}},
           {pressure={0.68, 0.76, 0.8}, orient="across", ramps={0.08, 0.06}})
end
print("the trunks carried down to the water")

--@ chunk 222
-- 28. the whole wood zone blended: every edge softened until the stems are part of the vapour
blend(woodz, {angle=1.5708})
print("the wood zone blended")

--@ chunk 223
-- 29. the stems cooled and unified, a few dark accents put back into them, and the reflections laid in the water
coolz = pile{{"lead white", 4}, {"pale smalt", 3.5}, {"cobalt blue", 0.6}, {"raw umber", 2.6}, medium=0.16}
work(water * rect(-30, 740, 1060, 200):soften(60), {hand="glaze", pile=coolz, tool="filbert 24",
     coverage=0.7, pressure={0.32, 0.24}, clip=true, angle=0.02})
acc = pile{{"raw umber", 3.4}, {"pale smalt", 2.2}, {"lead white", 0.6}, medium=0.08}
ab = brush{kind="filbert", width=4, stiffness=0.4}
for _, a in ipairs({{240, 800, 852}, {445, 806, 860}, {601, 796, 848}, {412, 764, 820}, {556, 768, 824}}) do
  ab:load(acc, 0.8)
  ab:stroke({{a[1] - 4, a[2]}, {a[1] - 3.4, (a[2] + a[3]) * 0.5}, {a[1] - 3, a[3]}},
            {pressure={0.3, 0.5, 0.35}, orient="across", ramps={0.2, 0.2}})
end
print("the stems cooled, the accents in")

--@ chunk 224
-- 30. the dark accents knocked back, and the reflections laid: narrow, cool, broken by the ripple lines
work(water * rect(-30, 780, 1060, 130):soften(40), {hand="glaze", pile=coolz, tool="filbert 24",
     coverage=0.55, pressure={0.3, 0.22}, clip=true, angle=0.02})
reflp = pile{{"lead white", 2}, {"pale smalt", 3}, {"raw umber", 3.6}, medium=0.12}
rb6 = brush{kind="filbert", width=8, stiffness=0.4}
mirror = {{240, 852, 96, 10}, {445, 860, 104, 11}, {601, 848, 88, 9}, {148, 800, 70, 7},
          {218, 812, 76, 7}, {300, 796, 64, 6}, {412, 820, 84, 8}, {494, 802, 68, 7},
          {556, 824, 80, 7}, {640, 808, 72, 7}, {742, 818, 66, 6}, {820, 800, 60, 6}}
for _, m in ipairs(mirror) do
  x, y, len, w = m[1], m[2], m[3], m[4]
  b = brush{kind="filbert", width=w, stiffness=0.4}
  b:load(reflp, 0.85)
  b:stroke({{x, y + 4}, {x + 0.6, y + len * 0.36}}, {pressure={0.5, 0.36}, orient="across", ramps={0.15, 0.2}})
  b:load(reflp, 0.85)
  b:stroke({{x + 0.3, y + len * 0.46}, {x, y + len * 0.76}}, {pressure={0.34, 0.2}, orient="across", ramps={0.2, 0.25}})
  b:load(reflp, 0.8)
  b:stroke({{x - 0.3, y + len * 0.84}, {x - 0.8, y + len}}, {pressure={0.2, 0.05}, orient="across", ramps={0.3, 0.4}})
end
print("the reflections laid")

--@ chunk 225
-- 31. the decision: the water in front of the wood taken down to a cool dark, so the pale stems and the vapour read against it
deepd = pile{{"lead white", 1.8}, {"pale smalt", 2.6}, {"cobalt blue", 1.4}, {"raw umber", 4.6}, medium=0.02}
midd = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"cobalt blue", 0.9}, {"raw umber", 3.6}, medium=0.04}
work(water * rect(-30, 866, 1060, 460):soften(90), {hand="body", pile=deepd, tool="filbert 16", coverage=1.6,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
work(water * rect(-30, 756, 1060, 130):soften(40), {hand="body", pile=midd, tool="filbert 14", coverage=1.3,
     pressure={0.48, 0.38}, clip=true, edge="lost", length={100, 180}, angle=0.02, fill=true})
blend(water * rect(-30, 750, 1060, 590):soften(60), {angle=0.02})
print("the water taken down")

--@ chunk 226
-- 32. the water cooled away from the mauve, and the stems carried down into it again
coolwater = pile{{"lead white", 1}, {"pale smalt", 3}, {"cobalt blue", 2.2}, {"raw umber", 2.6}, medium=0.12}
work(water * rect(-30, 620, 1060, 700):soften(120), {hand="glaze", pile=coolwater, tool="filbert 26",
     coverage=0.9, pressure={0.32, 0.24}, clip=true, angle=0.02})
barkn = pile{{"lead white", 2.2}, {"pale smalt", 2.8}, {"raw umber", 4.6}, medium=0.02}
trunks = {{148, 800, 7, 7}, {218, 812, 7.5, -8}, {300, 796, 6.5, 5}, {412, 820, 8, -6},
          {494, 802, 7, 9}, {556, 824, 7.5, -11}, {640, 808, 7, 6}, {742, 818, 6.5, -7},
          {820, 800, 6, 8}, {240, 852, 11, -13}, {445, 860, 12, 15}, {601, 848, 11, -9}}
for _, t in ipairs(trunks) do
  fx, fy, w, ln = t[1], t[2], t[3], t[4]
  b = brush{kind="filbert", width=w, stiffness=0.6}
  b:load(barkn, 1.0)
  b:stroke({{fx + ln * 0.6, fy - 96}, {fx + ln * 0.25, fy - 46}, {fx, fy + 4}},
           {pressure={0.62, 0.72, 0.78}, orient="across", ramps={0.14, 0.1}})
end
print("the water cooled, the stems down again")

--@ chunk 227
-- 33. cobalt was far too strong; the water laid again as a plain cool grey with only a trace of blue in it
coolfilm = pile{{"lead white", 2.2}, {"pale smalt", 3.4}, {"cobalt blue", 0.35}, {"raw umber", 3.2}, medium=0.04}
work(water, {hand="body", pile=coolfilm, tool="filbert 16", coverage=1.8, pressure={0.5, 0.4},
     clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
work(water * rect(-30, 980, 1060, 360):soften(110), {hand="body", pile=pile{{"lead white", 1.6}, {"pale smalt", 3}, {"cobalt blue", 0.4}, {"raw umber", 4.2}, medium=0.02},
     tool="filbert 16", coverage=1.2, pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
blend(water * rect(-30, 620, 1060, 720):soften(80), {angle=0.02})
print("the water a plain cool grey")

--@ chunk 228
-- 34. a cooler ground for the water: white and pale smalt carry it, and the depth put in with a thin blue-grey glaze
coolfilm2 = pile{{"lead white", 3.5}, {"pale smalt", 4}, {"raw umber", 2.2}, medium=0.03}
work(water, {hand="body", pile=coolfilm2, tool="filbert 16", coverage=1.7, pressure={0.5, 0.4},
     clip=true, edge="lost", length={110, 200}, angle=0.02, fill=true})
deepglaze = pile{{"pale smalt", 4}, {"cobalt blue", 1}, {"raw umber", 1.6}, medium=0.45}
work(water * rect(-30, 940, 1060, 420):soften(120), {hand="glaze", pile=deepglaze, tool="filbert 26",
     coverage=0.85, pressure={0.32, 0.24}, clip=true, angle=0.02})
work(water * rect(-30, 700, 1060, 260):soften(90), {hand="glaze", pile=pile{{"pale smalt", 4}, {"lead white", 2}, {"raw umber", 1.2}, medium=0.4},
     tool="filbert 26", coverage=0.8, pressure={0.3, 0.22}, clip=true, angle=0.02})
blend(water * rect(-30, 620, 1060, 720):soften(80), {angle=0.02})
print("the water cool and deep")

--@ chunk 229
-- 35. one laid film and one blend: the water smooth again, and this time a cool grey of the right depth
waterd = pile{{"lead white", 2}, {"pale smalt", 3}, {"cobalt blue", 0.3}, {"raw umber", 4}, {"bone black", 0.3}, medium=0.02}
work(water, {hand="body", pile=waterd, tool="filbert 16", coverage=2.0, pressure={0.52, 0.42},
     clip=true, edge="lost", length={130, 230}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
blend(water, {angle=0.02})
work(water * rect(-30, 1020, 1060, 320):soften(120), {hand="glaze", pile=pile{{"pale smalt", 4}, {"cobalt blue", 0.9}, {"raw umber", 2}, medium=0.5},
     tool="filbert 26", coverage=0.7, pressure={0.3, 0.22}, clip=true, angle=0.02})
print("the water smooth and cool")

--@ chunk 230
-- 36. no umber at all in the water: white, pale smalt, a little blue and a little black, so nothing goes brown
watern = pile{{"lead white", 3}, {"pale smalt", 3.4}, {"cobalt blue", 0.6}, {"bone black", 0.8}, medium=0.02}
work(water, {hand="body", pile=watern, tool="filbert 16", coverage=2.1, pressure={0.52, 0.42},
     clip=true, edge="lost", length={130, 230}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
blend(water, {angle=0.02})
work(water * rect(-30, 1040, 1060, 300):soften(130), {hand="glaze", pile=pile{{"pale smalt", 4}, {"cobalt blue", 1.2}, {"bone black", 1.2}, medium=0.5},
     tool="filbert 26", coverage=0.7, pressure={0.3, 0.22}, clip=true, angle=0.02})
print("the water a neutral grey")

--@ chunk 231
-- 37. the near water made deep without blue dashes, then the stems into it and their reflections under them
frontd = pile{{"lead white", 2.4}, {"pale smalt", 3.4}, {"bone black", 0.9}, medium=0.03}
work(water * rect(-30, 1020, 1060, 330):soften(110), {hand="body", pile=frontd, tool="filbert 16", coverage=1.3,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={130, 230}, angle=0.02, fill=true})
blend(water * rect(-30, 1000, 1060, 360):soften(110), {angle=0.02})

barkn = pile{{"lead white", 2.2}, {"pale smalt", 2.8}, {"raw umber", 4.6}, medium=0.02}
trunks = {{148, 800, 7, 7}, {218, 812, 7.5, -8}, {300, 796, 6.5, 5}, {412, 820, 8, -6},
          {494, 802, 7, 9}, {556, 824, 7.5, -11}, {640, 808, 7, 6}, {742, 818, 6.5, -7},
          {820, 800, 6, 8}, {240, 852, 11, -13}, {445, 860, 12, 15}, {601, 848, 11, -9}}
for _, t in ipairs(trunks) do
  fx, fy, w, ln = t[1], t[2], t[3], t[4]
  b = brush{kind="filbert", width=w, stiffness=0.6}
  b:load(barkn, 1.0)
  b:stroke({{fx + ln, fy - 150}, {fx + ln * 0.4, fy - 70}, {fx, fy + 4}},
           {pressure={0.3, 0.66, 0.78}, orient="across", ramps={0.16, 0.1}})
end
print("the stems into the water")

--@ chunk 232
-- 38. the whole wood drawn in one pass and left alone: stems from the vapour down into the water, in a cool grey
stemc = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.03}
stemn = pile{{"lead white", 2.2}, {"pale smalt", 2.8}, {"bone black", 0.9}, medium=0.02}
-- {foot x, foot y, width, top y, lean}
stems = {{148, 800, 7, 604, 7}, {218, 812, 7.5, 646, -8}, {300, 796, 6.5, 578, 5},
         {412, 820, 8, 614, -6}, {494, 802, 7, 640, 9}, {556, 824, 7.5, 592, -11},
         {640, 808, 7, 628, 6}, {742, 818, 6.5, 604, -7}, {820, 800, 6, 652, 8},
         {240, 852, 11, 566, -13}, {445, 860, 12, 552, 15}, {601, 848, 11, 578, -9},
         {352, 790, 5.5, 664, 4}, {686, 798, 5.5, 672, -5}, {906, 796, 5, 668, 5}}
for _, t in ipairs(stems) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  pileuse = (w > 8) and stemn or stemc
  b = brush{kind="filbert", width=w, stiffness=0.55}
  b:load(pileuse, 1.0)
  b:stroke({{fx + ln * 1.2, ty}, {fx + ln * 0.6, (ty + fy) * 0.55}, {fx, fy + 4}},
           {pressure={0.22, 0.62, 0.72}, orient="across", ramps={0.22, 0.12}})
end
print("the wood drawn in one pass")

--@ chunk 233
-- 38. the water cooled with a transparent blue-grey glaze, evenly laid and blended
coolgl = pile{{"pale smalt", 5}, {"bone black", 1.4}, {"cobalt blue", 0.5}, medium=0.55}
work(water * rect(-30, 700, 1060, 640):soften(100), {hand="glaze", pile=coolgl, tool="filbert 26",
     coverage=1.4, pressure={0.3, 0.24}, clip=true, angle=0.02})
blend(water * rect(-30, 640, 1060, 720):soften(80), {angle=0.02})
print("the water cooled")

--@ chunk 234
-- 39. the middle finished in one pass: stems, their reflections, a few glints, and the vapour over their tops
stemd = pile{{"lead white", 2.2}, {"pale smalt", 2.6}, {"bone black", 0.8}, medium=0.03}
stemm = pile{{"lead white", 2.8}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.03}
stems = {{148, 800, 7, 604, 7}, {218, 812, 7.5, 646, -8}, {300, 796, 6.5, 578, 5},
         {412, 820, 8, 614, -6}, {494, 802, 7, 640, 9}, {556, 824, 7.5, 592, -11},
         {640, 808, 7, 628, 6}, {742, 818, 6.5, 604, -7}, {820, 800, 6, 652, 8},
         {240, 852, 11, 566, -13}, {445, 860, 12, 552, 15}, {601, 848, 11, 578, -9},
         {352, 790, 5.5, 664, 4}, {686, 798, 5.5, 672, -5}, {906, 796, 5, 668, 5}}
for _, t in ipairs(stems) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w, stiffness=0.55}
  b:load(stemm, 0.95)
  b:stroke({{fx + ln * 1.2, ty}, {fx + ln * 0.7, (ty + fy) * 0.6}}, {pressure={0.2, 0.5}, orient="across", ramps={0.25, 0.15}})
  b:load(stemd, 1.0)
  b:stroke({{fx + ln * 0.6, (ty + fy) * 0.62}, {fx, fy + 4}}, {pressure={0.5, 0.75}, orient="across", ramps={0.15, 0.1}})
end
reflc = pile{{"lead white", 3.4}, {"pale smalt", 3.4}, {"bone black", 0.3}, medium=0.14}
for _, t in ipairs(stems) do
  fx, fy, w = t[1], t[2], t[3]
  len = w * 9
  b = brush{kind="filbert", width=w * 0.9, stiffness=0.4}
  b:load(reflc, 0.9)
  b:stroke({{fx, fy + 6}, {fx + 0.4, fy + len * 0.42}}, {pressure={0.45, 0.3}, orient="across", ramps={0.2, 0.25}})
  b:load(reflc, 0.85)
  b:stroke({{fx - 0.3, fy + len * 0.54}, {fx - 0.6, fy + len * 0.86}}, {pressure={0.26, 0.08}, orient="across", ramps={0.3, 0.4}})
end
glint = pile{{"lead white", 7}, {"pale smalt", 2.5}, medium=0.1}
gb = brush{kind="filbert", width=18, stiffness=0.4}
for _, g in ipairs({{120, 700, 170}, {430, 726, 150}, {700, 690, 190}, {220, 780, 210}, {560, 800, 180},
                    {860, 770, 160}, {300, 880, 200}, {640, 900, 170}, {140, 940, 150}, {460, 960, 190},
                    {760, 1020, 210}, {300, 1080, 180}, {620, 1140, 200}, {180, 1180, 160}, {520, 1230, 180}}) do
  gb:load(glint, 0.8)
  gb:stroke({{g[1], g[2]}, {g[1] + g[3], g[2] + 1}}, {pressure={0.28, 0.14}, orient="across"})
end
work(skyab * rect(-30, 540, 1060, 140):soften(50), {hand="glaze", pile=pile{{"lead white", 5}, {"pale smalt", 3.5}, medium=0.24},
     tool="filbert 26", coverage=0.9, pressure={0.3, 0.22}, clip=true, angle=0.03})
print("the middle finished")

--@ chunk 235
-- 40. the leaf-shaped reflections and the white sticks glazed back, and the reflections laid as soft streaks instead
waterg = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.14}
work(water * rect(-30, 840, 1060, 460):soften(70), {hand="glaze", pile=waterg, tool="filbert 24",
     coverage=0.9, pressure={0.3, 0.24}, clip=true, angle=0.02})
reflc = pile{{"lead white", 3.4}, {"pale smalt", 3.4}, {"bone black", 0.3}, medium=0.2}
for _, t in ipairs(stems) do
  fx, fy, w = t[1], t[2], t[3]
  len = w * 11
  work(ellipse(fx, fy + len * 0.45, w * 0.8, len * 0.45):soften(w * 0.9), {hand="glaze", pile=reflc,
       tool="filbert 24", coverage=0.85, pressure={0.3, 0.22}, clip=true})
end
print("the reflections as soft streaks")

--@ chunk 236
-- 41. every bright mark in the water buried under an opaque film, the stems put back, the reflections laid a whisper
watern = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.02}
work(water * rect(-30, 836, 1060, 500):soften(60), {hand="body", pile=watern, tool="filbert 16", coverage=2.0,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={130, 230}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
blend(water * rect(-30, 830, 1060, 510):soften(60), {angle=0.02})
stemd = pile{{"lead white", 2.2}, {"pale smalt", 2.6}, {"bone black", 0.8}, medium=0.03}
for _, t in ipairs(stems) do
  fx, fy, w = t[1], t[2], t[3]
  b = brush{kind="filbert", width=w, stiffness=0.55}
  b:load(stemd, 1.0)
  b:stroke({{fx, fy - 46}, {fx, fy + 4}}, {pressure={0.6, 0.75}, orient="across", ramps={0.15, 0.1}})
end
reflc = pile{{"lead white", 3.2}, {"pale smalt", 3.2}, {"bone black", 0.35}, medium=0.3}
for _, t in ipairs(stems) do
  fx, fy, w = t[1], t[2], t[3]
  len = w * 10
  b = brush{kind="filbert", width=w * 0.8, stiffness=0.4}
  b:load(reflc, 0.8)
  b:stroke({{fx, fy + 4}, {fx, fy + len * 0.55}}, {pressure={0.3, 0.12}, orient="across", ramps={0.3, 0.3}})
end
blend(water * rect(-30, 884, 1060, 440):soften(70), {angle=0.02})
print("the water clean, the reflections a whisper")

--@ chunk 237
-- 42. the water laid clean once more, the stems drawn again, and the reflections only a breath of light
watern = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.02}
work(water * rect(-30, 690, 1060, 650):soften(80), {hand="body", pile=watern, tool="filbert 16", coverage=2.0,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={130, 230}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
blend(water * rect(-30, 686, 1060, 660):soften(80), {angle=0.02})
stemm = pile{{"lead white", 2.8}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.03}
stemd = pile{{"lead white", 2.2}, {"pale smalt", 2.6}, {"bone black", 0.8}, medium=0.03}
for _, t in ipairs(stems) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w, stiffness=0.55}
  b:load(stemm, 0.95)
  b:stroke({{fx + ln * 1.2, ty}, {fx + ln * 0.7, (ty + fy) * 0.62}}, {pressure={0.2, 0.55}, orient="across", ramps={0.25, 0.15}})
  b:load(stemd, 1.0)
  b:stroke({{fx + ln * 0.55, (ty + fy) * 0.64}, {fx, fy + 4}}, {pressure={0.55, 0.78}, orient="across", ramps={0.15, 0.1}})
  breath = pile{{"lead white", 3.1}, {"pale smalt", 3.2}, {"bone black", 0.4}, medium=0.55}
  work(ellipse(fx, fy + w * 5, w * 2.2, w * 5):soften(w * 3), {hand="glaze", pile=breath, tool="filbert 24",
       coverage=0.35, pressure={0.28, 0.2}, clip=true})
end
print("the wood and its reflections, left alone now")

--@ chunk 238
-- 43. the near birches laid solid: one opaque film each, no sky showing through the bars
near2 = pile{{"raw umber", 5}, {"bone black", 4}, medium=0.01}
work(mA, {hand="body", pile=near2, tool="filbert 20", coverage=3.0, pressure={0.92, 0.95},
          clip=true, edge="firm", length={40, 80}, angle=1.45, fill=true, dips={1, 1.0, 0.1}})
work(mB, {hand="body", pile=near2, tool="filbert 18", coverage=3.0, pressure={0.9, 0.94},
          clip=true, edge="firm", length={40, 80}, angle=1.45, fill=true, dips={1, 1.0, 0.1}})
print("the near trunks solid")

--@ chunk 239
-- 44. the near birches modelled: light down the right of each, and the marks of the bark
sheen = pile{{"lead white", 4}, {"pale smalt", 2.5}, medium=0.35}
sheenb = brush{kind="filbert", width=22, stiffness=0.35}
for _, t in ipairs({{96, 40, 100, 560}, {92, 60, 90, 570}}) do
  sheenb:load(sheen, 0.9)
  sheenb:stroke({{t[1], t[2]}, {t[1] + 1, t[3]}, {t[1], t[4]}}, {pressure={0.2, 0.4, 0.15}, orient="across", ramps={0.3, 0.3}})
end
for _, t in ipairs({{218, 40, 100, 570}, {214, 80, 90, 580}}) do
  sheenb:load(sheen, 0.9)
  sheenb:stroke({{t[1], t[2]}, {t[1] + 1, t[3]}, {t[1], t[4]}}, {pressure={0.2, 0.4, 0.15}, orient="across", ramps={0.3, 0.3}})
end
barkpale = pile{{"lead white", 4.5}, {"pale smalt", 3}, {"raw umber", 0.8}, medium=0.25}
bb = brush{kind="filbert", width=5, stiffness=0.4}
marks = {{58, 120, 22}, {84, 168, 26}, {62, 232, 18}, {90, 300, 24}, {56, 356, 20}, {86, 420, 28},
         {60, 486, 22}, {92, 540, 24}, {186, 96, 24}, {212, 150, 20}, {182, 214, 26}, {208, 286, 22},
         {180, 350, 24}, {206, 414, 18}, {184, 470, 26}, {210, 528, 22}, {66, 40, 24}, {196, 30, 22}}
for _, m in ipairs(marks) do
  bb:load(barkpale, 0.8)
  bb:touch(m[1], m[2], {pressure=0.4, drag={m[3], 1}})
end
print("the near trunks modelled")

--@ chunk 240
-- 45. the modelling blended: the sheen and the bark marks softened into the trunk, the edges into the air
blend(mA, {angle=1.45})
blend(mB, {angle=1.45})
print("the near trunks blended")

--@ chunk 241
-- 46. the sheen knocked back: the dark of the bark laid thin over both trunks so the light is only a breath on the edge
darkthin = pile{{"raw umber", 5}, {"bone black", 4}, medium=0.12}
work(mA, {hand="glaze", pile=darkthin, tool="filbert 20", coverage=1.1, pressure={0.3, 0.24}, clip=true, angle=1.45})
work(mB, {hand="glaze", pile=darkthin, tool="filbert 18", coverage=1.1, pressure={0.3, 0.24}, clip=true, angle=1.45})
blend(mA, {angle=1.45})
blend(mB, {angle=1.45})
print("the sheen taken back")

--@ chunk 242
-- 47. the near bank made solid and quiet: one dark earth, its edge running level along the water
earthd = pile{{"raw umber", 5}, {"bone black", 3}, medium=0.02}
work(rect(-30, 1256, 1060, 160):soften(30), {hand="body", pile=earthd, tool="filbert 16", coverage=1.7,
     pressure={0.52, 0.42}, clip=true, edge="soft", length={100, 180}, angle=0.05, fill=true, dips={2, 0.95, 0.3}})
bank = rect(-30, 1256, 1060, 160):soften(18)
blend(bank, {angle=0.05})
print("the bank solid")

--@ chunk 243
-- 48. the smoke above the bank cleared, and the bank laid again to a shoreline I choose
work(rect(-30, 1226, 1060, 100):soften(30), {hand="body", pile=watern, tool="filbert 16", coverage=1.7,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={120, 200}, angle=0.02, fill=true})
blend(rect(-30, 1220, 1060, 110):soften(30), {angle=0.02})
earthd = pile{{"raw umber", 5}, {"bone black", 3}, medium=0.02}
shorepts = {{-30, 1300}, {150, 1306}, {330, 1310}, {520, 1306}, {700, 1300}, {860, 1296}, {1030, 1292}}
bankm = below(shorepts):blur(7)
work(bankm, {hand="body", pile=earthd, tool="filbert 16", coverage=1.8, pressure={0.52, 0.42},
     clip=true, edge="lost", length={100, 180}, angle=0.05, fill=true, dips={2, 0.95, 0.3}})
blend(bankm, {angle=0.05})
print("the bank to its shoreline")

--@ chunk 244
-- 49. the debris on the water cleared, and the fallen birch laid on the bank along the shore
work(rect(-30, 1236, 1060, 70):soften(18), {hand="body", pile=watern, tool="filbert 14", coverage=1.6,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={110, 190}, angle=0.02, fill=true})
blend(rect(-30, 1232, 1060, 78):soften(18), {angle=0.02})
logpath = {{24, 1348}, {160, 1338}, {300, 1324}, {430, 1306}, {520, 1288}}
logs = brush{kind="filbert", width=15, stiffness=0.5}
for i = 1, #logpath - 1 do
  logs:load(pile{{"raw umber", 4}, {"lead white", 2}, {"pale smalt", 1.2}, medium=0.05}, 0.95)
  logs:stroke({logpath[i], logpath[i + 1]}, {pressure={0.6, 0.5}, orient="across"})
end
logs = brush{kind="filbert", width=11, stiffness=0.45}
for i = 1, #logpath - 1 do
  logs:load(pile{{"raw umber", 4.5}, {"bone black", 2.5}, medium=0.03}, 0.9)
  logs:stroke({{logpath[i][1] + 2, logpath[i][2] + 9}, {logpath[i + 1][1] + 2, logpath[i + 1][2] + 8}},
              {pressure={0.4, 0.3}, orient="across"})
end
logs = brush{kind="filbert", width=5, stiffness=0.5}
for i = 1, #logpath - 1 do
  logs:load(pile{{"lead white", 4}, {"pale smalt", 2.5}, medium=0.2}, 0.85)
  logs:stroke({{logpath[i][1], logpath[i][2] - 5}, {logpath[i + 1][1], logpath[i + 1][2] - 5}},
              {pressure={0.4, 0.3}, orient="across"})
end
print("the fallen birch on the bank")

--@ chunk 245
-- 50. the log gone from a plank to a wet fallen birch: darker, tapered, its light barely there
logpath2 = {{20, 1344}, {150, 1336}, {290, 1322}, {420, 1304}, {516, 1286}}
logd = pile{{"raw umber", 4.6}, {"lead white", 1.2}, {"pale smalt", 1}, medium=0.03}
lg = brush{kind="filbert", width=19, stiffness=0.45}
for i = 1, #logpath2 - 1 do
  lg:load(logd, 1.0)
  lg:stroke({logpath2[i], logpath2[i + 1]}, {pressure={0.62, 0.55}, orient="across"})
end
lg = brush{kind="filbert", width=12, stiffness=0.45}
for i = 1, #logpath2 - 1 do
  lg:load(pile{{"raw umber", 4.8}, {"bone black", 2.6}, medium=0.02}, 0.95)
  lg:stroke({{logpath2[i][1] + 3, logpath2[i][2] + 11}, {logpath2[i + 1][1] + 3, logpath2[i + 1][2] + 9}},
            {pressure={0.45, 0.32}, orient="across"})
end
lg = brush{kind="filbert", width=4, stiffness=0.5}
for i = 1, #logpath2 - 1 do
  lg:load(pile{{"lead white", 3}, {"pale smalt", 2.5}, {"raw umber", 1.5}, medium=0.3}, 0.8)
  lg:stroke({{logpath2[i][1] + 4, logpath2[i][2] - 4}, {logpath2[i + 1][1] + 4, logpath2[i + 1][2] - 4}},
            {pressure={0.3, 0.2}, orient="across"})
end
print("the log a fallen birch")

--@ chunk 246
-- 51. the plank painted out; a darker, lumpier birch laid in three pieces with stubs of branch
earthd = pile{{"raw umber", 5}, {"bone black", 3}, medium=0.02}
cb = brush{kind="filbert", width=30, stiffness=0.4}
for i = 1, #logpath2 - 1 do
  cb:load(earthd, 1.0)
  cb:stroke({{logpath2[i][1], logpath2[i][2] + 4}, {logpath2[i + 1][1], logpath2[i + 1][2] + 3}},
            {pressure={0.6, 0.55}, orient="across"})
end
logdark = pile{{"raw umber", 4.8}, {"lead white", 0.8}, {"pale smalt", 0.9}, medium=0.02}
pieces = {{{18, 1344}, {110, 1338}}, {{124, 1336}, {250, 1326}, {330, 1316}}, {{350, 1314}, {450, 1298}, {520, 1284}}}
for i, p in ipairs(pieces) do
  b = brush{kind="filbert", width=22 - i * 4, stiffness=0.4}
  b:load(logdark, 1.0)
  b:stroke(p, {pressure={0.66, 0.62, 0.6}, orient="across", ramps={0.12, 0.12}})
end
b = brush{kind="filbert", width=7, stiffness=0.4}
b:load(logdark, 1.0)
b:stroke({{214, 1326}, {206, 1300}}, {pressure={0.6, 0.3}, orient="across"})
b:stroke({{382, 1308}, {396, 1284}}, {pressure={0.55, 0.25}, orient="across"})
-- the faint light along its upper side
b = brush{kind="filbert", width=3.5, stiffness=0.5}
for i, p in ipairs(pieces) do
  b:load(pile{{"lead white", 2.4}, {"pale smalt", 2.2}, {"raw umber", 2.4}, medium=0.45}, 0.75)
  b:stroke({p[1], p[#p]}, {pressure={0.25, 0.2}, orient="across"})
end
print("the birch broken and dark on the bank")

--@ chunk 247
-- 52. the beam and the stubs taken away; the birch laid as one soft dark form, and mist along the shore above it
earthd = pile{{"raw umber", 5}, {"bone black", 3}, medium=0.02}
cb = brush{kind="filbert", width=26, stiffness=0.4}
cb:load(earthd, 1.0)
cb:stroke({{190, 1320}, {400, 1300}}, {pressure={0.6, 0.55}, orient="across"})
cb:load(earthd, 1.0)
cb:stroke({{206, 1310}, {208, 1296}}, {pressure={0.6, 0.5}, orient="across"})
cb:load(earthd, 1.0)
cb:stroke({{384, 1298}, {396, 1282}}, {pressure={0.6, 0.5}, orient="across"})
logdark = pile{{"raw umber", 4.4}, {"lead white", 1.2}, {"pale smalt", 1.2}, medium=0.04}
logm = ribbon({{14, 1342}, {150, 1332}, {300, 1318}, {430, 1300}, {520, 1284}}, 18):soften(4)
work(logm, {hand="body", pile=logdark, tool="filbert 12", coverage=2.2, pressure={0.55, 0.5},
     clip=true, edge="soft", length={50, 100}, angle=0.06, fill=true})
blend(logm, {angle=0.06})
print("the birch soft and dark")

--@ chunk 248
-- 53. the pale ridge painted out; the birch laid dark, with its shadow under it and mist above it
earthd = pile{{"raw umber", 5}, {"bone black", 3}, medium=0.02}
cb = brush{kind="filbert", width=28, stiffness=0.4}
for i = 1, #logpath2 - 1 do
  cb:load(earthd, 1.0)
  cb:stroke({{logpath2[i][1], logpath2[i][2] + 4}, {logpath2[i + 1][1], logpath2[i + 1][2] + 2}},
            {pressure={0.6, 0.55}, orient="across"})
end
cb:load(earthd, 1.0)
cb:stroke({{420, 1298}, {540, 1284}}, {pressure={0.6, 0.5}, orient="across"})
shad = pile{{"raw umber", 5}, {"bone black", 3.5}, medium=0.02}
work(ribbon({{30, 1352}, {160, 1342}, {310, 1328}, {440, 1310}, {530, 1296}}, 10):soften(5),
     {hand="body", pile=shad, tool="filbert 10", coverage=2.0, pressure={0.5, 0.42}, clip=true, edge="soft", length={50, 90}, fill=true})
logdark = pile{{"raw umber", 4.8}, {"lead white", 0.9}, {"pale smalt", 1}, medium=0.03}
work(ribbon({{14, 1338}, {150, 1328}, {300, 1314}, {430, 1296}, {524, 1280}}, 13):soften(4),
     {hand="body", pile=logdark, tool="filbert 10", coverage=2.0, pressure={0.5, 0.44}, clip=true, edge="soft", length={50, 90}, fill=true})
print("the birch dark on the bank")

--@ chunk 249
-- 54. the whole foreground cleared to one dark earth, and the birch drawn on it once, cleanly
work(rect(-30, 1268, 1060, 150):soften(22), {hand="body", pile=earthd, tool="filbert 16", coverage=2.6,
     pressure={0.55, 0.45}, clip=true, edge="lost", length={100, 180}, angle=0.05, fill=true, dips={1, 1.0, 0.1}})
blend(rect(-30, 1264, 1060, 160):soften(22), {angle=0.05})
logdark = pile{{"raw umber", 5}, {"lead white", 0.8}, {"pale smalt", 1}, medium=0.02}
logm = ribbon({{10, 1338}, {150, 1328}, {300, 1314}, {430, 1296}, {528, 1278}}, 14):soften(3)
work(logm, {hand="body", pile=logdark, tool="filbert 10", coverage=2.8, pressure={0.5, 0.44},
     clip=true, edge="firm", length={40, 80}, fill=true})
print("the birch drawn on clean ground")

--@ chunk 250
-- 55. the birch taken down to sit in the bank, smoothed, and mist laid along the shore above it
work(logm, {hand="glaze", pile=earthd, tool="filbert 20", coverage=1.4, pressure={0.3, 0.24}, clip=true, angle=0.06})
blend(logm, {angle=0.06})
shorepts = {{-30, 1300}, {150, 1306}, {330, 1310}, {520, 1306}, {700, 1300}, {860, 1296}, {1030, 1292}}
mistline = ribbon(shorepts, 22):soften(9)
work(mistline, {hand="glaze", pile=pile{{"lead white", 5.5}, {"pale smalt", 3.5}, medium=0.3},
     tool="filbert 24", coverage=1.0, pressure={0.3, 0.22}, clip=true, angle=0.04})
print("the birch down, mist on the shore")

--@ chunk 251
-- 56. she is put back: a dark figure at the water's edge, seen from behind against the pale water
figdark = pile{{"raw umber", 3.4}, {"bone black", 3}, medium=0.01}
mfig = poly({{741, 1206}, {751, 1206}, {753, 1218}, {756, 1242}, {760, 1274}, {732, 1274}, {736, 1242}, {739, 1218}})
mfig = mfig + ellipse(746, 1212, 5.4, 6):blur(0.6)
work(mfig, {hand="body", pile=figdark, tool="filbert 6", coverage=2.6, pressure={0.9, 0.86},
            clip=true, edge="soft", length={12, 26}, fill=true})
fb = brush{kind="filbert", width=4.5, stiffness=0.7}
fb:load(figdark, 0.95)
fb:stroke({{740, 1273}, {739, 1286}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
fb:stroke({{752, 1273}, {753, 1286}}, {pressure={0.85, 0.9}, orient="across", shake=0.3})
print("the figure, drawn")

--@ chunk 252
-- 57. the figure smoothed, and a breath of light on her right shoulder
blend(mfig, {angle=1.57})
rl = brush{kind="filbert", width=2.6, stiffness=0.6}
rl:load(pile{{"lead white", 4}, {"pale smalt", 2.5}, medium=0.25}, 0.85)
rl:stroke({{752, 1216}, {755, 1240}, {758, 1266}}, {pressure={0.4, 0.45, 0.2}, orient="across", ramps={0.2, 0.3}})
rl:stroke({{750, 1208}, {752, 1214}}, {pressure={0.35, 0.3}, orient="across"})
print("the figure smoothed")

--@ chunk 253
-- 58. the rim light taken off her back: it read as a cane
db = brush{kind="filbert", width=7, stiffness=0.6}
db:load(figdark, 1.0)
db:stroke({{752, 1212}, {756, 1242}, {759, 1270}}, {pressure={0.8, 0.85, 0.7}, orient="across", ramps={0.12, 0.15}})
db:load(figdark, 1.0)
db:stroke({{750, 1206}, {753, 1216}}, {pressure={0.7, 0.6}, orient="across"})
blend(mfig, {angle=1.57})
print("her back in shadow again")

--@ chunk 254
-- 59. the wandering scratches in the sky buried, and cloud laid back over them
skyp = pile{{"lead white", 4}, {"pale smalt", 3.5}, {"raw umber", 0.8}, medium=0.06}
patch = rect(390, 170, 260, 280):soften(44)
work(patch, {hand="body", pile=skyp, tool="filbert 14", coverage=1.8, pressure={0.48, 0.4},
     clip=true, edge="soft", length={80, 150}, angle=0.05, fill=true})
cloudw = pile{{"lead white", 5}, {"yellow ochre", 1.2}, {"red earth", 0.3}, medium=0.16}
for _, c in ipairs({{430, 230, 190, 22}, {470, 290, 210, 26}, {420, 350, 170, 20}, {500, 400, 150, 18}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):soften(c[4]), {hand="glaze", pile=cloudw, tool="filbert 24",
       coverage=0.9, pressure={0.3, 0.22}, clip=true, angle=0.04})
end
cloudc = pile{{"pale smalt", 4}, {"lead white", 2}, medium=0.2}
for _, c in ipairs({{560, 200, 180, 20}, {520, 340, 200, 24}, {600, 260, 150, 18}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):soften(c[4]), {hand="glaze", pile=cloudc, tool="filbert 24",
       coverage=0.8, pressure={0.3, 0.22}, clip=true, angle=0.04})
end
print("the sky cleaned")

--@ chunk 255
-- 60. the rectangle's edges dissolved: a much wider, much softer passage, and the sky blended
work(skyab * rect(280, 40, 560, 520):soften(130), {hand="body", pile=skyp, tool="filbert 16", coverage=1.5,
     pressure={0.46, 0.38}, clip=true, edge="lost", length={100, 180}, angle=0.05, fill=true})
blend(skyab * rect(150, 20, 900, 540):soften(100), {angle=0.05})
print("the sky dissolved and blended")

--@ chunk 256
-- 61. the blender dragged the dark of the trunks across the sky; the sky laid again over the smear
skyzone = (rect(120, -20, 540, 600):soften(90)) * (-mA:grow(10)) * (-mB:grow(10))
skyp2 = pile{{"lead white", 4}, {"pale smalt", 3.4}, {"raw umber", 0.6}, medium=0.05}
work(skyzone, {hand="body", pile=skyp2, tool="filbert 14", coverage=1.9, pressure={0.48, 0.4},
     clip=true, edge="lost", length={90, 170}, angle=0.05, fill=true})
print("the sky laid over the smear")

--@ chunk 257
-- 62. the cloud bands laid back across the whole sky, warm and cool, so the flattened part has its air again
warmc = pile{{"lead white", 5}, {"yellow ochre", 1.4}, {"red earth", 0.4}, medium=0.18}
coolc = pile{{"pale smalt", 4}, {"lead white", 2}, medium=0.2}
darkc = pile{{"pale smalt", 5}, {"smalt", 1}, {"raw umber", 1}, medium=0.3}
for _, c in ipairs({{300, 90, 240, 16}, {520, 130, 260, 20}, {760, 90, 220, 14}, {300, 210, 280, 18},
                    {640, 250, 240, 16}, {380, 320, 260, 20}, {720, 350, 220, 16}, {300, 430, 240, 18},
                    {560, 470, 260, 16}, {330, 520, 220, 14}, {700, 500, 240, 14}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):soften(c[4] * 1.1), {hand="glaze", pile=warmc, tool="filbert 24",
       coverage=0.8, pressure={0.3, 0.22}, clip=true, angle=0.04})
end
for _, c in ipairs({{380, 150, 260, 18}, {700, 200, 240, 16}, {320, 270, 250, 16}, {620, 390, 260, 18},
                    {360, 470, 240, 14}, {780, 300, 200, 14}, {300, 60, 200, 12}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):soften(c[4] * 1.1), {hand="glaze", pile=coolc, tool="filbert 24",
       coverage=0.7, pressure={0.3, 0.22}, clip=true, angle=0.04})
end
for _, c in ipairs({{420, 40, 220, 12}, {700, 60, 200, 10}, {340, 180, 200, 10}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):soften(c[4] * 1.2), {hand="glaze", pile=darkc, tool="filbert 24",
       coverage=0.6, pressure={0.28, 0.2}, clip=true, angle=0.04})
end
print("the cloud bands back")

--@ chunk 258
-- 63. the sooty fringe the blender left beside the trunks painted out, cloud laid back over it
fringe = rect(80, -20, 320, 660):soften(26) * (-mA:grow(38)) * (-mB:grow(38))
work(fringe, {hand="body", pile=skyp2, tool="filbert 12", coverage=2.0, pressure={0.5, 0.4},
     clip=true, edge="soft", length={60, 120}, angle=0.05, fill=true})
for _, c in ipairs({{300, 120, 200, 14}, {320, 260, 210, 16}, {290, 380, 190, 14}, {330, 470, 200, 12}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):soften(c[4]), {hand="glaze", pile=warmc, tool="filbert 24",
       coverage=0.7, pressure={0.3, 0.22}, clip=true, angle=0.04})
end
for _, c in ipairs({{310, 190, 200, 14}, {330, 330, 190, 14}, {300, 430, 180, 12}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):soften(c[4]), {hand="glaze", pile=coolc, tool="filbert 24",
       coverage=0.7, pressure={0.3, 0.22}, clip=true, angle=0.04})
end
print("the fringe out, the cloud back")

--@ chunk 259
-- 64. the near water deepened so the figure has something to stand against, and the far shore cooled off the yellow
deepglass = pile{{"pale smalt", 4}, {"cobalt blue", 0.6}, {"bone black", 0.9}, medium=0.55}
work(water * rect(-30, 1060, 1060, 290):soften(110), {hand="glaze", pile=deepglass, tool="filbert 26",
     coverage=0.85, pressure={0.3, 0.24}, clip=true, angle=0.02})
work(water * rect(-30, 930, 1060, 180):soften(110), {hand="glaze", pile=deepglass, tool="filbert 26",
     coverage=0.45, pressure={0.28, 0.22}, clip=true, angle=0.02})
work(rect(-30, 584, 1060, 70):soften(24), {hand="glaze", pile=pile{{"pale smalt", 4.5}, {"lead white", 2}, medium=0.5},
     tool="filbert 26", coverage=0.5, pressure={0.3, 0.22}, clip=true, angle=0.03})
print("the near water deepened, the shore cooled")

--@ chunk 260
-- 65. the blue dashes evened into the water with a neutral glaze, and the surface blended where nothing stands on it
neut = pile{{"pale smalt", 4}, {"lead white", 2.4}, {"bone black", 0.5}, medium=0.45}
work(water * rect(-30, 930, 1060, 390):soften(90), {hand="glaze", pile=neut, tool="filbert 26",
     coverage=1.5, pressure={0.32, 0.26}, clip=true, angle=0.02})
blend(water * rect(-30, 930, 1060, 355):soften(70) * (-mfig:grow(10)), {angle=0.02})
print("the near water evened")

--@ chunk 261
-- 66. the foreground water laid once more, opaque and even, and the figure put back on it
fgw = pile{{"lead white", 2.4}, {"pale smalt", 3}, {"bone black", 0.6}, medium=0.02}
work(water * rect(-30, 920, 1060, 400):soften(80), {hand="body", pile=fgw, tool="filbert 14", coverage=1.9,
     pressure={0.5, 0.4}, clip=true, edge="lost", length={120, 200}, angle=0.02, fill=true, dips={2, 0.95, 0.3}})
print("the foreground water laid even")

--@ chunk 262
-- 67. the dabs in the foreground evened out: a broad glaze hand pass, which lays an even film and leaves no cushions
work(water * rect(-30, 900, 1060, 430):soften(70), {hand="glaze", pile=fgw, tool="filbert 26",
     coverage=2.0, pressure={0.34, 0.28}, clip=true, angle=0.02, length={150, 280}})
print("the foreground water evened with the broad hand")

--@ chunk 263
-- 68. the lumpy foreground at the left laid flat: one broad even film, no dabbing, no filling
flatf = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.06}
work(water * rect(-20, 1020, 540, 360):soften(40), {hand="glaze", pile=flatf, tool="filbert 26",
     coverage=2.4, pressure={0.36, 0.3}, clip=true, angle=0.02, length={180, 320}})
print("the left foreground laid flat")

--@ chunk 264
-- 69. the marbling buried under an opaque coat, then smoothed with a broad glaze over it
opaque = pile{{"lead white", 2.6}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.0}
work(water * rect(-20, 1000, 560, 380):soften(36), {hand="body", pile=opaque, tool="filbert 14", coverage=2.2,
     pressure={0.58, 0.5}, clip=true, edge="lost", length={110, 190}, angle=0.02, fill=true, dips={1, 1.0, 0.1}})
work(water * rect(-20, 1000, 560, 380):soften(36), {hand="glaze", pile=opaque, tool="filbert 26",
     coverage=1.2, pressure={0.34, 0.28}, clip=true, angle=0.02, length={180, 320}})
print("the left foreground opaque and smooth")

--@ chunk 265
-- 70. mist across the feet of the near birches, their reflections broken under it, and a few long ripples
work(water * rect(-20, 1100, 600, 150):soften(60), {hand="glaze", pile=pile{{"lead white", 5}, {"pale smalt", 3.5}, medium=0.4},
     tool="filbert 26", coverage=0.8, pressure={0.3, 0.24}, clip=true, angle=0.02})
refnear = pile{{"pale smalt", 3}, {"bone black", 1.6}, {"raw umber", 2}, medium=0.1}
nb = brush{kind="filbert", width=26, stiffness=0.4}
for _, r in ipairs({{72, 1206, 84, 46}, {152, 1216, 76, 40}}) do
  nb:load(refnear, 0.85)
  nb:stroke({{r[1], r[2]}, {r[1] + 1, r[2] + r[3] * 0.4}}, {pressure={0.45, 0.3}, orient="across", ramps={0.25, 0.25}})
  nb:load(refnear, 0.8)
  nb:stroke({{r[1] - 1, r[2] + r[3] * 0.55}, {r[1] - 2, r[2] + r[3] * 0.85}}, {pressure={0.25, 0.06}, orient="across", ramps={0.3, 0.35}})
end
ripple = pile{{"lead white", 4}, {"pale smalt", 3}, medium=0.22}
rb7 = brush{kind="filbert", width=24, stiffness=0.4}
for _, r in ipairs({{240, 1180, 190}, {260, 1230, 150}, {420, 1150, 170}, {560, 1210, 140}, {640, 1170, 160}}) do
  rb7:load(ripple, 0.8)
  rb7:stroke({{r[1], r[2]}, {r[1] + r[3], r[2] + 1}}, {pressure={0.3, 0.12}, orient="across"})
end
print("the near feet in the mist")

--@ chunk 266
-- 71. the two blobs that were standing in for reflections painted out; the near water left quiet
spot = (rect(20, 1160, 220, 130):soften(30)) * water
work(spot, {hand="body", pile=opaque, tool="filbert 14", coverage=2.2, pressure={0.55, 0.48},
     clip=true, edge="lost", length={90, 160}, angle=0.02, fill=true, dips={1, 1.0, 0.1}})
work(spot, {hand="glaze", pile=opaque, tool="filbert 26", coverage=1.1, pressure={0.32, 0.26}, clip=true, angle=0.02})
print("the near water quiet again")

--@ chunk 267
-- 72. the thin glaze over the whole picture to bind the passages into one air
bind = pile{{"lead white", 4}, {"pale smalt", 3}, {"raw umber", 1.5}, medium=0.35}
work(rect(-20, -20, 1040, 1430), {hand="glaze", pile=bind, tool="filbert 26", coverage=0.55,
     pressure={0.3, 0.22}, angle=0.04, length={200, 340}})
print("the binding glaze")

--@ chunk 268
-- 73. the grove stated again after the binding glaze: the stems given their step of value against the vapour
stemm = pile{{"lead white", 2.8}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.03}
stemd = pile{{"lead white", 2}, {"pale smalt", 2.6}, {"bone black", 0.9}, medium=0.03}
for _, t in ipairs(stems) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w, stiffness=0.55}
  b:load(stemm, 0.95)
  b:stroke({{fx + ln * 1.2, ty}, {fx + ln * 0.7, (ty + fy) * 0.62}}, {pressure={0.18, 0.5}, orient="across", ramps={0.28, 0.15}})
  b:load(stemd, 1.0)
  b:stroke({{fx + ln * 0.55, (ty + fy) * 0.64}, {fx, fy + 4}}, {pressure={0.5, 0.74}, orient="across", ramps={0.16, 0.1}})
end
print("the grove restated")

--@ chunk 269
-- 74. the stems given flat sides and square feet, and a few twigs to break the blade look
stemd = pile{{"lead white", 2}, {"pale smalt", 2.6}, {"bone black", 0.9}, medium=0.03}
for _, t in ipairs(stems) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w * 1.25, stiffness=0.6}
  b:load(stemd, 1.0)
  b:stroke({{fx + ln * 0.5, (ty + fy) * 0.66}, {fx, fy - 6}}, {pressure={0.72, 0.76}, orient="across", ramps={0.03, 0.03}})
end
twig = pile{{"raw umber", 3}, {"pale smalt", 2.4}, {"lead white", 1}, medium=0.06}
tb2 = brush{kind="filbert", width=3.4, stiffness=0.45}
for _, t in ipairs({{300, 660, 1}, {412, 690, -1}, {556, 660, 1}, {742, 680, -1}, {240, 640, 1}, {640, 700, -1}}) do
  tb2:load(twig, 0.85)
  tb2:stroke({{t[1], t[2]}, {t[1] + 16 * t[3], t[2] - 26}}, {pressure={0.55, 0.2}, orient="across", ramps={0.1, 0.3}})
  tb2:load(twig, 0.85)
  tb2:stroke({{t[1] + 3, t[2] + 14}, {t[1] + 22 * t[3], t[2] - 6}}, {pressure={0.45, 0.12}, orient="across", ramps={0.1, 0.35}})
end
print("the stems flat-footed, with twigs")

--@ chunk 270
-- 75. the little stray twigs that read as flies veiled away, and the tops of the wood drawn further into the vapour
mistv = pile{{"lead white", 5.5}, {"pale smalt", 3.5}, medium=0.34}
work(rect(-30, 604, 1060, 120):soften(26), {hand="glaze", pile=mistv, tool="filbert 26", coverage=1.0,
     pressure={0.3, 0.22}, clip=true, angle=0.03})
print("the vapour over the tops of the wood")

--@ chunk 271
-- 76. each stray twig covered with a small opaque patch of vapour
mistop = pile{{"lead white", 5}, {"pale smalt", 3.5}, {"raw umber", 0.5}, medium=0.05}
for _, c in ipairs({{308, 647}, {314, 671}, {404, 677}, {425, 701}, {564, 647}, {570, 671},
                    {734, 667}, {755, 691}, {248, 627}, {254, 651}, {632, 687}, {653, 711}}) do
  work(ellipse(c[1], c[2], 24, 15):soften(9), {hand="body", pile=mistop, tool="filbert 8", coverage=2.0,
       pressure={0.5, 0.45}, clip=true, edge="soft", length={20, 40}, fill=true})
end
print("the twigs gone")

--@ chunk 272
-- 77. the old figure painted out and a larger one drawn: long skirt, no stick legs, standing at the edge
work(rect(706, 1158, 84, 140):soften(18) * water, {hand="body", pile=fgw, tool="filbert 12", coverage=2.2,
     pressure={0.55, 0.48}, clip=true, edge="lost", length={80, 150}, angle=0.02, fill=true, dips={1, 1.0, 0.1}})
work(rect(706, 1158, 84, 140):soften(18) * water, {hand="glaze", pile=fgw, tool="filbert 24", coverage=1.0,
     pressure={0.32, 0.26}, clip=true, angle=0.02})
figdark = pile{{"raw umber", 3.4}, {"bone black", 3}, medium=0.01}
mfig = poly({{740, 1200}, {752, 1200}, {754, 1214}, {757, 1248}, {760, 1284}, {732, 1284}, {735, 1248}, {738, 1214}})
mfig = mfig + ellipse(746, 1194, 6, 7):blur(0.7)
work(mfig, {hand="body", pile=figdark, tool="filbert 6", coverage=2.6, pressure={0.9, 0.86},
            clip=true, edge="soft", length={12, 26}, fill=true})
blend(mfig, {angle=1.57})
fb = brush{kind="filbert", width=4, stiffness=0.7}
fb:load(figdark, 0.95)
fb:stroke({{739, 1283}, {738, 1290}}, {pressure={0.8, 0.85}, orient="across"})
fb:stroke({{753, 1283}, {754, 1290}}, {pressure={0.8, 0.85}, orient="across"})
print("the figure, larger and whole")

--@ chunk 273
-- 78. her shape rebuilt: a head on a neck, shoulders, a long skirt that flares, seen from behind
figdark = pile{{"raw umber", 3.4}, {"bone black", 3}, medium=0.01}
headm = ellipse(746, 1193, 5.4, 6.4):blur(0.6)
shawl = poly({{740.5, 1196}, {751.5, 1196}, {755, 1210}, {737, 1210}})
torso = poly({{739.5, 1206}, {752.5, 1206}, {753.5, 1234}, {738.5, 1234}})
skirt = poly({{739, 1230}, {753, 1230}, {758.5, 1285}, {733.5, 1285}})
mfig = headm + shawl + torso + skirt
work(mfig, {hand="body", pile=figdark, tool="filbert 6", coverage=2.4, pressure={0.88, 0.84},
            clip=true, edge="soft", length={10, 22}, fill=true})
print("her shape rebuilt")

--@ chunk 274
-- 79. her, drawn with the brush this time: skirt, body, shawled head, a little larger
work(rect(706, 1156, 84, 146):soften(18) * water, {hand="body", pile=fgw, tool="filbert 12", coverage=2.2,
     pressure={0.55, 0.48}, clip=true, edge="lost", length={80, 150}, angle=0.02, fill=true, dips={1, 1.0, 0.1}})
work(rect(706, 1156, 84, 146):soften(18) * water, {hand="glaze", pile=fgw, tool="filbert 24", coverage=1.0,
     pressure={0.32, 0.26}, clip=true, angle=0.02})
figdark = pile{{"raw umber", 3.2}, {"bone black", 3}, medium=0.01}
sk = brush{kind="filbert", width=26, stiffness=0.55}
sk:load(figdark, 1.0)
sk:stroke({{746, 1222}, {746, 1256}, {746, 1290}}, {pressure=0.6, orient="across", swell={1, 1.1, 1.4}, ramps={0.06, 0.06}})
tor = brush{kind="filbert", width=15, stiffness=0.6}
tor:load(figdark, 1.0)
tor:stroke({{746, 1200}, {746, 1228}}, {pressure={0.72, 0.66}, orient="across", ramps={0.08, 0.08}})
sh2 = brush{kind="filbert", width=17, stiffness=0.5}
sh2:load(figdark, 1.0)
sh2:stroke({{746, 1198}, {746, 1212}}, {pressure={0.7, 0.5}, orient="across", swell={0.8, 1.25}, ramps={0.1, 0.15}})
hd2 = brush{kind="filbert", width=11, stiffness=0.6}
hd2:load(figdark, 1.0)
hd2:stroke({{746, 1186}, {746, 1196}}, {pressure={0.7, 0.8}, orient="across", ramps={0.15, 0.15}})
print("her drawn with the brush")

--@ chunk 275
-- 80. her neck filled and her waist joined, so she is one shape and not three
nk = brush{kind="filbert", width=9, stiffness=0.6}
nk:load(figdark, 0.95)
nk:stroke({{746, 1193}, {746, 1204}}, {pressure={0.75, 0.7}, orient="across", ramps={0.12, 0.12}})
wa = brush{kind="filbert", width=19, stiffness=0.55}
wa:load(figdark, 1.0)
wa:stroke({{746, 1216}, {746, 1234}}, {pressure={0.66, 0.62}, orient="across", ramps={0.1, 0.1}})
print("her made whole")

--@ chunk 276
-- 81. the popcorn where the twigs were covered toned back into the vapour along the shore
mistband = pile{{"lead white", 5}, {"pale smalt", 3.6}, {"raw umber", 1.1}, medium=0.25}
work(rect(-30, 612, 1060, 100):soften(22), {hand="glaze", pile=mistband, tool="filbert 26", coverage=1.5,
     pressure={0.32, 0.26}, clip=true, angle=0.03})
print("the shore mist evened")

--@ chunk 277
-- 82. the cotton wool along the shore covered opaquely, patch by patch, then the band glazed to one tone
mistop2 = pile{{"lead white", 5}, {"pale smalt", 3.6}, {"raw umber", 0.8}, medium=0.04}
for _, c in ipairs({{280, 660}, {450, 665}, {590, 660}, {710, 655}, {790, 660}, {350, 682}, {520, 686}, {660, 682}}) do
  work(ellipse(c[1], c[2], 46, 30):soften(14), {hand="body", pile=mistop2, tool="filbert 12", coverage=2.2,
       pressure={0.5, 0.46}, clip=true, edge="soft", length={40, 80}, fill=true})
end
work(rect(-30, 612, 1060, 106):soften(20), {hand="glaze", pile=mistband, tool="filbert 26", coverage=1.1,
     pressure={0.32, 0.26}, clip=true, angle=0.03})
print("the shore band made one tone again")

--@ chunk 278
-- 83. the whole mist band laid as one even film, and the tops of the wood drawn into it again
work(rect(-30, 604, 1060, 134):soften(18), {hand="glaze", pile=mistop2, tool="filbert 26", coverage=2.6,
     pressure={0.34, 0.28}, clip=true, angle=0.03, length={200, 340}})
stemm = pile{{"lead white", 2.8}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.03}
stemd = pile{{"lead white", 2}, {"pale smalt", 2.6}, {"bone black", 0.9}, medium=0.03}
for _, t in ipairs(stems) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w * 1.1, stiffness=0.55}
  b:load(stemm, 0.9)
  b:stroke({{fx + ln * 1.3, ty}, {fx + ln * 0.7, 700}, {fx + ln * 0.4, 750}},
           {pressure={0.15, 0.42, 0.55}, orient="across", ramps={0.3, 0.15}})
  b = brush{kind="filbert", width=w * 1.25, stiffness=0.6}
  b:load(stemd, 1.0)
  b:stroke({{fx + ln * 0.4, 752}, {fx, fy + 2}}, {pressure={0.68, 0.76}, orient="across", ramps={0.06, 0.05}})
end
work(rect(-30, 610, 1060, 110):soften(24), {hand="glaze", pile=mistband, tool="filbert 26", coverage=0.9,
     pressure={0.3, 0.24}, clip=true, angle=0.03})
print("the mist band and the wood in it")

--@ chunk 279
-- 84. the wood given its weight again, and the reflections in the water under it, a breath lighter than the water
stemd2 = pile{{"lead white", 1.7}, {"pale smalt", 2.4}, {"bone black", 1.1}, medium=0.03}
refl = pile{{"lead white", 3.6}, {"pale smalt", 3.2}, {"bone black", 0.3}, medium=0.3}
for _, t in ipairs(stems) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w * 1.3, stiffness=0.6}
  b:load(stemd2, 1.0)
  b:stroke({{fx + ln * 0.45, 720}, {fx, fy + 2}}, {pressure={0.6, 0.74}, orient="across", ramps={0.14, 0.06}})
  len = w * 9
  b = brush{kind="filbert", width=w * 1.1, stiffness=0.4}
  b:load(refl, 0.85)
  b:stroke({{fx, fy + 6}, {fx, fy + len * 0.45}}, {pressure={0.4, 0.22}, orient="across", ramps={0.28, 0.28}})
  b:load(refl, 0.8)
  b:stroke({{fx - 0.4, fy + len * 0.58}, {fx - 0.8, fy + len * 0.9}}, {pressure={0.2, 0.05}, orient="across", ramps={0.32, 0.4}})
end
print("the wood and its reflections")

--@ chunk 280
-- 85. the pointed tops of the wood taken back into the vapour, and the white almonds of reflection covered
work(rect(-30, 606, 1060, 130):soften(24), {hand="glaze", pile=mistband, tool="filbert 26", coverage=1.3,
     pressure={0.32, 0.26}, clip=true, angle=0.03})
work(water * rect(-30, 840, 1060, 300):soften(60), {hand="glaze", pile=waterg, tool="filbert 26", coverage=1.1,
     pressure={0.32, 0.26}, clip=true, angle=0.02})
breath = pile{{"lead white", 3.6}, {"pale smalt", 3.2}, {"bone black", 0.35}, medium=0.6}
for _, t in ipairs(stems) do
  fx, fy, w = t[1], t[2], t[3]
  work(ellipse(fx, fy + w * 4.6, w * 1.9, w * 4.4):soften(w * 2.2), {hand="glaze", pile=breath, tool="filbert 24",
       coverage=0.4, pressure={0.28, 0.22}, clip=true})
end
print("the tops in the vapour, the reflections a breath")

--@ chunk 281
-- 86. the near birches reflected in the foreground water, broken by the ripple lines
nrefl = pile{{"raw umber", 3}, {"bone black", 2}, {"pale smalt", 1.6}, medium=0.14}
nb2 = brush{kind="filbert", width=38, stiffness=0.4}
for _, r in ipairs({{72, 1198, 62, 34}, {152, 1206, 58, 30}}) do
  nb2:load(nrefl, 0.85)
  nb2:stroke({{r[1], r[2]}, {r[1] + 1, r[2] + r[3] * 0.4}}, {pressure={0.5, 0.36}, orient="across", ramps={0.2, 0.2}})
  nb2:load(nrefl, 0.8)
  nb2:stroke({{r[1] - 1, r[2] + r[3] * 0.55}, {r[1] - 2, r[2] + r[3] * 0.9}}, {pressure={0.28, 0.06}, orient="across", ramps={0.3, 0.35}})
end
-- the ripple lines drawn across them, in the tone of the water
across = pile{{"lead white", 3}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.5}
ab2 = brush{kind="filbert", width=16, stiffness=0.35}
for _, a in ipairs({{30, 1228, 120}, {150, 1252, 110}, {40, 1276, 130}, {160, 1290, 100}}) do
  ab2:load(across, 0.85)
  ab2:stroke({{a[1], a[2]}, {a[1] + a[3], a[2] + 1}}, {pressure={0.4, 0.18}, orient="across"})
end
print("the near birches reflected")

--@ chunk 282
-- 87. the four dark blobs painted out; the near birches instead run down into the dark of the bank
spot2 = (rect(10, 1150, 250, 130):soften(26)) * water
work(spot2, {hand="body", pile=opaque, tool="filbert 14", coverage=2.2, pressure={0.55, 0.48},
     clip=true, edge="lost", length={90, 160}, angle=0.02, fill=true, dips={1, 1.0, 0.1}})
work(spot2, {hand="glaze", pile=opaque, tool="filbert 24", coverage=1.0, pressure={0.32, 0.26}, clip=true, angle=0.02})
shadowdeep = pile{{"raw umber", 3.4}, {"bone black", 2.6}, {"pale smalt", 1.2}, medium=0.25}
work(ellipse(112, 1180, 120, 110):soften(70), {hand="glaze", pile=shadowdeep, tool="filbert 26",
     coverage=0.7, pressure={0.3, 0.22}, clip=true})
print("the near feet lost in the dark")

--@ chunk 283
-- 88. the water given its middle distance: a soft dark band under the wood, a breath of light on it, and a few long ripples
work(water * rect(-30, 750, 1060, 170):soften(80), {hand="glaze", pile=neut, tool="filbert 26",
     coverage=0.55, pressure={0.3, 0.24}, clip=true, angle=0.02})
work(water * rect(-30, 850, 1060, 140):soften(90), {hand="glaze", pile=pile{{"lead white", 5}, {"pale smalt", 4}, medium=0.6},
     tool="filbert 26", coverage=0.5, pressure={0.28, 0.22}, clip=true, angle=0.02})
rip2 = pile{{"lead white", 4}, {"pale smalt", 3.4}, medium=0.45}
rip3 = pile{{"pale smalt", 3}, {"bone black", 1.2}, {"raw umber", 1.4}, medium=0.4}
rb8 = brush{kind="filbert", width=40, stiffness=0.35}
for _, r in ipairs({{120, 800, 320}, {520, 826, 300}, {180, 880, 340}, {640, 900, 280}, {260, 960, 300},
                    {560, 1000, 320}, {120, 1060, 300}, {420, 1110, 340}, {180, 1170, 320}, {520, 1220, 300}}) do
  rb8:load(rip2, 0.8)
  rb8:stroke({{r[1], r[2]}, {r[1] + r[3], r[2] + 2}}, {pressure={0.26, 0.1}, orient="across"})
end
for _, r in ipairs({{380, 860, 300}, {160, 940, 260}, {620, 970, 280}, {300, 1040, 320}, {760, 1150, 280}, {420, 1200, 300}}) do
  rb8:load(rip3, 0.8)
  rb8:stroke({{r[1], r[2]}, {r[1] + r[3], r[2] + 2}}, {pressure={0.24, 0.08}, orient="across"})
end
print("the water's middle distance given")

--@ chunk 284
-- 89. the ripples that read as floating sticks knocked back into the tone of the water
work(water * rect(-30, 780, 1060, 500):soften(90), {hand="glaze", pile=waterg, tool="filbert 26",
     coverage=1.0, pressure={0.32, 0.26}, clip=true, angle=0.02})
print("the ripples softened")

--@ chunk 285
-- 90. her put back, the glazes having taken her: skirt, body, shawled head, drawn on the quiet water
figdark = pile{{"raw umber", 3.2}, {"bone black", 3}, medium=0.01}
sk:load(figdark, 1.0)
sk:stroke({{746, 1220}, {746, 1254}, {746, 1290}}, {pressure=0.6, orient="across", swell={1, 1.1, 1.4}, ramps={0.06, 0.06}})
tor:load(figdark, 1.0)
tor:stroke({{746, 1200}, {746, 1228}}, {pressure={0.72, 0.66}, orient="across", ramps={0.08, 0.08}})
sh2:load(figdark, 1.0)
sh2:stroke({{746, 1198}, {746, 1212}}, {pressure={0.7, 0.5}, orient="across", swell={0.8, 1.25}, ramps={0.1, 0.15}})
hd2:load(figdark, 1.0)
hd2:stroke({{746, 1186}, {746, 1197}}, {pressure={0.7, 0.8}, orient="across", ramps={0.15, 0.15}})
nk:load(figdark, 0.95)
nk:stroke({{746, 1193}, {746, 1204}}, {pressure={0.75, 0.7}, orient="across", ramps={0.12, 0.12}})
print("her drawn again")

--@ chunk 286
-- 91. four more stems to make the wood a wood, and long reflections under the three nearest trees
more = {{180, 806, 5.5, 636, 5}, {266, 812, 5, 656, -6}, {520, 826, 5.5, 648, 6}, {872, 806, 5, 664, -4}}
stemm = pile{{"lead white", 2.8}, {"pale smalt", 3}, {"bone black", 0.5}, medium=0.03}
stemd2 = pile{{"lead white", 1.7}, {"pale smalt", 2.4}, {"bone black", 1.1}, medium=0.03}
for _, t in ipairs(more) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w * 1.1, stiffness=0.55}
  b:load(stemm, 0.9)
  b:stroke({{fx + ln * 1.2, ty}, {fx + ln * 0.6, 730}, {fx + ln * 0.3, 770}},
           {pressure={0.15, 0.45, 0.58}, orient="across", ramps={0.3, 0.15}})
  b = brush{kind="filbert", width=w * 1.2, stiffness=0.6}
  b:load(stemd2, 1.0)
  b:stroke({{fx + ln * 0.3, 772}, {fx, fy + 2}}, {pressure={0.65, 0.74}, orient="across", ramps={0.1, 0.06}})
end
refl2 = pile{{"lead white", 3.8}, {"pale smalt", 3.4}, {"bone black", 0.35}, medium=0.5}
for _, t in ipairs({{240, 852, 11}, {445, 860, 12}, {601, 848, 11}}) do
  fx, fy, w = t[1], t[2], t[3]
  b = brush{kind="filbert", width=w * 2.2, stiffness=0.4}
  b:load(refl2, 0.8)
  b:stroke({{fx, fy + 6}, {fx - 1, fy + 70}, {fx - 2, fy + 140}}, {pressure={0.3, 0.16, 0.03},
           orient="across", ramps={0.3, 0.35}, swell={1, 0.8, 0.6}})
end
print("the wood made a wood")

--@ chunk 287
-- 92. the needle points taken off the tops: each stem redrawn with a blunt start and a soft blur above it
stemw = pile{{"lead white", 3}, {"pale smalt", 2.6}, {"raw umber", 1.6}, medium=0.04}
stemwd = pile{{"lead white", 2}, {"pale smalt", 2.2}, {"raw umber", 2.8}, medium=0.03}
softtop = pile{{"lead white", 4}, {"pale smalt", 3}, {"raw umber", 1.6}, medium=0.3}
allstems = {}
for _, t in ipairs(stems) do allstems[#allstems + 1] = t end
for _, t in ipairs(more) do allstems[#allstems + 1] = t end
for _, t in ipairs(allstems) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w * 1.2, stiffness=0.6}
  b:load(stemwd, 1.0)
  b:stroke({{fx + ln * 0.5, (ty + fy) * 0.62}, {fx, fy + 2}}, {pressure={0.68, 0.76}, orient="across", ramps={0.05, 0.05}})
  b = brush{kind="filbert", width=w * 1.2, stiffness=0.6}
  b:load(stemw, 0.95)
  b:stroke({{fx + ln * 1.1, ty + 46}, {fx + ln * 0.5, (ty + fy) * 0.62}}, {pressure={0.6, 0.62}, orient="across", ramps={0.05, 0.05}})
  b = brush{kind="filbert", width=w * 3.2, stiffness=0.35}
  b:load(softtop, 0.8)
  b:stroke({{fx + ln * 1.15, ty + 30}, {fx + ln * 1.1, ty - 20}}, {pressure={0.3, 0.06}, orient="across", ramps={0.3, 0.4}})
end
print("the tops soft")

--@ chunk 288
-- 93. the water deepened just above the bank, so the bottom of the picture is weighted and the mist line reads
deepf = pile{{"pale smalt", 3.4}, {"bone black", 1.4}, {"raw umber", 1.6}, medium=0.35}
work(water * rect(-30, 1170, 1060, 130):soften(70), {hand="glaze", pile=deepf, tool="filbert 26",
     coverage=0.75, pressure={0.32, 0.26}, clip=true, angle=0.02})
print("the near water weighted")

--@ chunk 289
-- 94. the last thin glaze over everything but her, to bind the passages into one evening
final = pile{{"lead white", 4}, {"pale smalt", 3}, {"raw umber", 1.4}, medium=0.4}
finmask = rect(-20, -20, 1040, 1430) * (-(poly({{720, 1170}, {780, 1170}, {780, 1300}, {720, 1300}}):grow(8)))
work(finmask, {hand="glaze", pile=final, tool="filbert 26", coverage=0.35, pressure={0.28, 0.22}, angle=0.04, length={200, 340}})
print(wait(60 * 6))
print("water", drying(500, 1000), "wood", drying(445, 800), "sky", drying(600, 200), "bank", drying(500, 1330))

--@ chunk 290
-- 95. the light of the vapour laid down the water as a path, and three more stems to break the even spacing
path = pile{{"lead white", 4.5}, {"pale smalt", 3.4}, medium=0.5}
for _, p in ipairs({{470, 780, 60, 90}, {480, 900, 80, 110}, {500, 1040, 100, 120}, {520, 1170, 120, 100}}) do
  work(ellipse(p[1], p[2], p[3], p[4]):soften(p[3] * 1.6), {hand="glaze", pile=path, tool="filbert 26",
       coverage=0.42, pressure={0.3, 0.24}, clip=true})
end
stemw = pile{{"lead white", 3}, {"pale smalt", 2.6}, {"raw umber", 1.6}, medium=0.04}
stemwd = pile{{"lead white", 2}, {"pale smalt", 2.2}, {"raw umber", 2.8}, medium=0.03}
for _, t in ipairs({{372, 818, 6, 622, 6}, {700, 816, 6.5, 610, -8}, {560, 838, 7.5, 596, 9}}) do
  fx, fy, w, ty, ln = t[1], t[2], t[3], t[4], t[5]
  b = brush{kind="filbert", width=w * 1.2, stiffness=0.6}
  b:load(stemwd, 1.0)
  b:stroke({{fx + ln * 0.5, (ty + fy) * 0.62}, {fx, fy + 2}}, {pressure={0.68, 0.76}, orient="across", ramps={0.05, 0.05}})
  b = brush{kind="filbert", width=w * 1.2, stiffness=0.6}
  b:load(stemw, 0.95)
  b:stroke({{fx + ln * 1.1, ty + 40}, {fx + ln * 0.5, (ty + fy) * 0.62}}, {pressure={0.55, 0.62}, orient="across", ramps={0.08, 0.05}})
  b = brush{kind="filbert", width=w * 3, stiffness=0.35}
  b:load(softtop, 0.8)
  b:stroke({{fx + ln * 1.15, ty + 26}, {fx + ln * 1.1, ty - 16}}, {pressure={0.28, 0.05}, orient="across", ramps={0.3, 0.4}})
end
print("the path of light on the water, and three more stems")

--@ chunk 291
-- 96. a patch of reflected light in the water where she stands, and a soft shadow under her to plant her
warmw = pile{{"lead white", 4}, {"yellow ochre", 0.8}, {"red earth", 0.2}, medium=0.6}
work(ellipse(790, 1150, 200, 150):soften(90), {hand="glaze", pile=warmw, tool="filbert 26",
     coverage=0.3, pressure={0.28, 0.22}, clip=true})
work(ellipse(748, 1292, 46, 16):soften(14), {hand="glaze", pile=pile{{"pale smalt", 3}, {"bone black", 1.4}, medium=0.45},
     tool="filbert 20", coverage=0.5, pressure={0.3, 0.24}, clip=true})
print("the light at her feet")

--@ chunk 292
-- 97. the last of the value: weight at the top of the sky and a deeper band of water under the mist
skdeep = pile{{"pale smalt", 4.4}, {"smalt", 1}, {"raw umber", 1.6}, medium=0.5}
work(skyab * rect(-30, -20, 1060, 150):soften(90), {hand="glaze", pile=skdeep, tool="filbert 26",
     coverage=0.45, pressure={0.3, 0.24}, clip=true, angle=0.05})
middeep2 = pile{{"pale smalt", 3.4}, {"bone black", 1.1}, {"raw umber", 1.4}, medium=0.4}
work(water * rect(-30, 630, 1060, 190):soften(70), {hand="glaze", pile=middeep2, tool="filbert 26",
     coverage=0.5, pressure={0.32, 0.26}, clip=true, angle=0.02})
print("the sky weighted at the top, the water under the mist deepened")

--@ chunk 293
-- 98. the cool strip at the left edge warmed to match the sky, and the near trunks modelled down their length
leftsky = pile{{"lead white", 4.5}, {"pale smalt", 2.4}, {"yellow ochre", 0.6}, medium=0.25}
strip = rect(-10, -20, 56, 640):soften(14) * (-(mA:grow(6)))
work(strip, {hand="glaze", pile=leftsky, tool="filbert 20", coverage=0.7, pressure={0.3, 0.24}, clip=true})
-- the trunks: a soft light down the middle of each, and a deeper shadow at the left edge
model = pile{{"lead white", 3}, {"pale smalt", 2.2}, medium=0.22}
mb = brush{kind="filbert", width=18, stiffness=0.35}
for _, t in ipairs({{82, 40, 560}, {200, 40, 570}}) do
  mb:load(model, 0.85)
  mb:stroke({{t[1], t[2]}, {t[1] + 1, (t[2] + t[3]) * 0.5}, {t[1], t[3]}}, {pressure={0.3, 0.4, 0.15}, orient="across", ramps={0.3, 0.3}})
end
dmodel = pile{{"raw umber", 4}, {"bone black", 2.4}, medium=0.2}
for _, t in ipairs({{52, 30, 570}, {170, 40, 580}}) do
  mb:load(dmodel, 0.85)
  mb:stroke({{t[1], t[2]}, {t[1] + 1, (t[2] + t[3]) * 0.5}, {t[1], t[3]}}, {pressure={0.35, 0.45, 0.15}, orient="across", ramps={0.3, 0.3}})
end
print("the left edge and the near trunks")

--@ chunk 294
-- 99. the striped modelling taken off: the trunks back to one solid body, and the left edge one warm sky
work(mA, {hand="body", pile=near2, tool="filbert 20", coverage=3.0, pressure={0.92, 0.95},
          clip=true, edge="firm", length={40, 80}, angle=1.45, fill=true, dips={1, 1.0, 0.1}})
work(mB, {hand="body", pile=near2, tool="filbert 18", coverage=3.0, pressure={0.9, 0.94},
          clip=true, edge="firm", length={40, 80}, angle=1.45, fill=true, dips={1, 1.0, 0.1}})
blend(mA, {angle=1.45})
blend(mB, {angle=1.45})
edge2 = pile{{"lead white", 5}, {"pale smalt", 2.8}, {"yellow ochre", 0.5}, medium=0.05}
work(rect(-10, -20, 56, 640):soften(14) * (-(mA:grow(8))), {hand="body", pile=edge2, tool="filbert 14", coverage=1.8,
     pressure={0.5, 0.44}, clip=true, edge="soft", length={70, 130}, angle=0.05, fill=true})
print("the trunks solid again")

--@ chunk 295
-- 100. the edges of the near birches cut back into the sky, so they stand firm against it
cut = pile{{"raw umber", 4}, {"bone black", 3}, medium=0.04}
cb2 = brush{kind="filbert", width=4, stiffness=0.5}
for _, e in ipairs({{118, 20, 118, 300, 121, 600}, {218, 20, 216, 300, 214, 600}}) do
  cb2:load(cut, 0.9)
  cb2:stroke({{e[1], e[2]}, {e[3], e[4]}, {e[5], e[6]}}, {pressure={0.3, 0.55, 0.35}, orient="across", ramps={0.25, 0.3}})
end
print("the edges of the birches cut")

--@ chunk 296
-- the paint left to stand
print(wait(60 * 24 * 3))
print("sky", drying(600, 200), "water", drying(500, 1000), "wood", drying(445, 800), "bank", drying(500, 1330), "figure", drying(746, 1230), "trunks", drying(80, 300))

--@ chunk 297
-- 101. her drawn once more, a little larger, so that she is a person and not a peg
work(rect(700, 1160, 100, 150):soften(16) * water, {hand="body", pile=fgw, tool="filbert 12", coverage=2.2,
     pressure={0.55, 0.48}, clip=true, edge="lost", length={80, 150}, angle=0.02, fill=true, dips={1, 1.0, 0.1}})
work(rect(700, 1160, 100, 150):soften(16) * water, {hand="glaze", pile=fgw, tool="filbert 24", coverage=1.0,
     pressure={0.32, 0.26}, clip=true, angle=0.02})
figdark = pile{{"raw umber", 3}, {"bone black", 3}, medium=0.01}
sk:load(figdark, 1.0)
sk:stroke({{748, 1226}, {748, 1258}, {748, 1292}}, {pressure=0.62, orient="across", swell={1, 1.05, 1.45}, ramps={0.05, 0.05}})
tor:load(figdark, 1.0)
tor:stroke({{748, 1202}, {748, 1234}}, {pressure={0.74, 0.66}, orient="across", ramps={0.08, 0.08}})
sh2:load(figdark, 1.0)
sh2:stroke({{748, 1198}, {748, 1214}}, {pressure={0.72, 0.5}, orient="across", swell={0.85, 1.35}, ramps={0.1, 0.15}})
hd2:load(figdark, 1.0)
hd2:stroke({{748, 1184}, {748, 1197}}, {pressure={0.72, 0.82}, orient="across", ramps={0.15, 0.15}})
nk:load(figdark, 0.95)
nk:stroke({{748, 1192}, {748, 1205}}, {pressure={0.76, 0.7}, orient="across", ramps={0.12, 0.12}})
print("her, a little larger")

--@ chunk 298
local pts = {{500,100},{500,400},{500,560},{500,700},{500,950},{500,1100},{150,400},{150,900},{600,1300}}
for _,p in ipairs(pts) do
  print(p[1], p[2], drying(p[1], p[2]))
end
print("W,H", W, H)

--@ chunk 299
print("trunks:", trunks, #trunks)
for i,v in ipairs(trunks) do print("  t"..i, v) end
print("stems:", stems, #stems)
for i,v in ipairs(stems) do print("  s"..i, v) end
print("bank:", bank, "bankm:", bankm, "spot2:", spot2, "finmask:", finmask)
print(wait(2 * 24 * 60))

--@ chunk 300
-- near birches, drawn as they should be: leaning, tapering, flaring at the foot
t1 = ribbon({{52, -30}, {66, 190}, {80, 430}, {88, 660}, {96, 900}}, {46, 42, 40, 44, 52})
t2 = ribbon({{172, -30}, {186, 200}, {198, 470}, {210, 720}, {222, 940}}, {60, 55, 52, 58, 68})
t3 = ribbon({{112, -30}, {116, 260}, {121, 520}}, {14, 12, 11})
near = (t1 + t2 + t3):blur(3):grow(7)

nz = noise{seed=11, period=210, octaves=3, persistence=0.55}
skyd = pile{{"lead white", 3}, {"pale smalt", 5}, {"raw umber", 2.2}, {"cobalt blue", 0.3}, medium=0.06}
mA = above(function(x) return 170 + 30 * nz(x, 50) end)
work(mA - near, {hand="broad", tool="filbert 18", pile=skyd, coverage=2.6, angle=0.0,
                clip=true, edge="soft", dips={2, 0.75, 0.2}})
print("zone A laid", mA:area())

--@ chunk 301
skym = pile{{"lead white", 3.4}, {"pale smalt", 4.2}, {"raw umber", 1.5}, {"cobalt blue", 0.25}, medium=0.05}
skyl = pile{{"lead white", 4.2}, {"pale smalt", 3.4}, {"raw umber", 0.9}, {"cobalt blue", 0.2}, medium=0.05}
mB = (below(function(x) return 345 + 36 * nz(x, 260) end) - above(function(x) return 120 + 28 * nz(x, 200) end)):blur(15)
mC = (below(function(x) return 505 + 40 * nz(x, 300) end) - above(function(x) return 300 + 30 * nz(x, 240) end)):blur(17)
work(mB - near, {hand="broad", tool="filbert 18", pile=skym, coverage=2.4, angle=0.02, clip=true, edge="soft", dips={2, 0.75, 0.2}})
work(mC - near, {hand="broad", tool="filbert 18", pile=skyl, coverage=2.3, angle=-0.02, clip=true, edge="soft", dips={2, 0.75, 0.2}})
print("B", mB:area(), "C", mC:area())

--@ chunk 302
B = function(a, b) return (below(a) * above(b)):blur(13) end
skyB = B(function(x) return 352 + 36 * nz(x, 260) end, function(x) return 122 + 28 * nz(x, 200) end)
skyC = B(function(x) return 508 + 40 * nz(x, 300) end, function(x) return 302 + 30 * nz(x, 240) end)
skyD = B(function(x) return 606 + 30 * nz(x, 340) end, function(x) return 438 + 30 * nz(x, 280) end)
mistb = B(function(x) return 772 + 26 * nz(x, 360) end, function(x) return 528 + 30 * nz(x, 300) end)
watr  = B(function(x) return 1240 + 40 * nz(x, 420) end, function(x) return 700 + 30 * nz(x, 360) end)
bankm2 = above(function(x) return 1195 + 34 * nz(x, 300) + 0.055 * x end)
for k, v in pairs({skyA=above(function(x) return 170+30*nz(x,50) end), skyB=skyB, skyC=skyC, skyD=skyD, mist=mistb, water=watr, bank=bankm2}) do
  print(k, string.format("%8.0f", v:area()))
end

--@ chunk 303
f = function(x) return 400 end
g = function(x) return 200 end
p1 = below(f); p2 = above(g)
print("below400", p1:area(), "above200", p2:area(), "canvas", W*H)
print("inter", (p1*p2):area())
print("below400 - below200", (p1 - below(g)):area())
print("below400 - above200", (p1 - p2):area())
print("mult raw", p1:at(500,300), p2:at(500,300), (p1*p2):at(500,300))
print("blur test", (p1*p2):blur(13):area())

--@ chunk 304
r1 = rect(0, 0, 1000, 400)
r2 = rect(0, 200, 1000, 600)
print("rect areas", r1:area(), r2:area(), "r1*r2", (r1*r2):area(), "r1-r2", (r1-r2):area())
p1 = below(function(x) return 400 end)
print("type below", p1, p1:at(500,600), p1:at(500,100))
print("below-1 ...", (-p1):area())
bm = mask(function(x, y) return smoothstep(150, 250, y + 30 * nz(x, 300)) * (1 - smoothstep(350, 450, y + 30 * nz(x, 260))) end)
print("my band area", bm:area(), bm:at(500,300), bm:at(500,100), bm:at(500,600))

--@ chunk 305
function band(hi, lo, sd, amp, soft)
  local n = noise{seed=sd, period=250, octaves=3, persistence=0.55}
  return mask(function(x, y)
    return smoothstep(hi - soft, hi + soft, y + amp * n(x, 60)) *
           (1 - smoothstep(lo - soft, lo + soft, y + amp * n(x, 640)))
  end)
end
skym2 = pile{{"lead white", 3.6}, {"pale smalt", 4.6}, {"raw umber", 1.7}, {"cobalt blue", 0.25}, medium=0.05}
skyl2 = pile{{"lead white", 4.2}, {"pale smalt", 3.8}, {"raw umber", 1.2}, {"cobalt blue", 0.2}, medium=0.05}
skyw2 = pile{{"lead white", 5}, {"pale smalt", 2.6}, {"yellow ochre", 1.2}, {"red earth", 0.35}, medium=0.06}
skyB = band(118, 336, 21, 30, 26)
skyC = band(300, 486, 22, 34, 28)
skyD = band(444, 622, 23, 26, 26)
print(string.format("B %.0f  C %.0f  D %.0f", skyB:area(), skyC:area(), skyD:area()))
work(skyB - near, {hand="broad", tool="filbert 18", pile=skym2, coverage=2.3, angle=0.02, clip=true, edge="soft", dips={2, 0.75, 0.2}})
work(skyC - near, {hand="broad", tool="filbert 18", pile=skyl2, coverage=2.2, angle=-0.02, clip=true, edge="soft", dips={2, 0.75, 0.2}})
work(skyD - near, {hand="broad", tool="filbert 18", pile=skyw2, coverage=2.0, angle=0.01, clip=true, edge="soft", dips={2, 0.75, 0.2}})

--@ chunk 306
print("globals ok", waterp, waterm)
waterp = pile{{"lead white", 2.6}, {"pale smalt", 3.4}, {"raw umber", 3.0}, medium=0.04}
wf1 = mask(function(x, y) return smoothstep(655, 790, y + 18 * nz(x, 700)) * (1 - smoothstep(880, 990, y + 16 * nz(x, 400))) end)
wf2 = mask(function(x, y) return smoothstep(900, 1010, y + 16 * nz(x, 400)) * (1 - smoothstep(1060, 1180, y + 16 * nz(x, 250))) end)
wf3 = mask(function(x, y) return smoothstep(1080, 1200, y + 14 * nz(x, 250)) end)
work(wf1, {hand="broad", tool="filbert 20", pile=waterp, coverage=1.5, angle=0.0, clip=true, edge="soft", dips={2, 0.8, 0.2}})
work(wf2, {hand="broad", tool="filbert 20", pile=waterp, coverage=2.4, angle=0.01, clip=true, edge="soft", dips={2, 0.8, 0.2}})
work(wf3, {hand="broad", tool="filbert 20", pile=waterp, coverage=3.2, angle=-0.01, clip=true, edge="soft", dips={2, 0.8, 0.2}})
blend(mask(function(x, y) return smoothstep(810, 910, y + 14 * nz(x, 700)) end), {angle=0.0})

--@ chunk 307
waterp2 = pile{{"lead white", 3.2}, {"pale smalt", 4.4}, {"bone black", 1.5}, {"raw umber", 0.7}, {"cobalt blue", 0.3}, medium=0.05}
waterm = mask(function(x, y) return smoothstep(650, 800, y + 18 * nz(x, 700)) end)
work(waterm, {hand="broad", tool="filbert 20", pile=waterp2, coverage=2.2, angle=0.0, clip=true, edge="soft", dips={2, 0.8, 0.2}})
blend(mask(function(x, y) return smoothstep(700, 850, y + 14 * nz(x, 700)) end), {angle=0.0})

--@ chunk 308
deepw = pile{{"pale smalt", 3}, {"bone black", 2.2}, {"raw umber", 1.2}, {"lead white", 1.2}, medium=0.55}
wA = mask(function(x, y) return smoothstep(655, 800, y + 18 * nz(x, 700)) end)
wB = mask(function(x, y) return smoothstep(950, 1120, y + 20 * nz(x, 300)) end)
wC = mask(function(x, y) return smoothstep(1150, 1300, y + 18 * nz(x, 200)) end)
work(wA, {hand="glaze", pile=deepw, coverage=0.95, clip=true})
work(wB, {hand="glaze", pile=deepw, coverage=1.0, clip=true})
work(wC, {hand="glaze", pile=deepw, coverage=0.85, clip=true})

--@ chunk 309
darkw = pile{{"lead white", 2.0}, {"pale smalt", 3.0}, {"bone black", 2.6}, {"raw umber", 1.4}, medium=0.02}
pA = mask(function(x, y) return smoothstep(660, 830, y + 18 * nz(x, 700)) end)
pB = mask(function(x, y) return smoothstep(920, 1090, y + 22 * nz(x, 300)) end)
pC = mask(function(x, y) return smoothstep(1120, 1290, y + 20 * nz(x, 200)) end)
work(pA, {hand="broad", tool="filbert 20", pile=darkw, coverage=1.3, angle=0.0, clip=true, edge="soft", dips={2, 0.8, 0.2}})
work(pB, {hand="broad", tool="filbert 20", pile=darkw, coverage=1.3, angle=0.01, clip=true, edge="soft", dips={2, 0.8, 0.2}})
work(pC, {hand="broad", tool="filbert 20", pile=darkw, coverage=1.4, angle=-0.01, clip=true, edge="soft", dips={2, 0.8, 0.2}})
blend(mask(function(x, y) return smoothstep(700, 880, y + 14 * nz(x, 700)) end), {angle=0.0})

--@ chunk 310
print("darkw", darkw)
test1 = pile{{"raw umber", 5}, {"bone black", 5}}
print("test1", test1)
work(rect(430, 950, 150, 150), {hand="body", pile=test1, coverage=2.5})
work(rect(600, 950, 150, 150), {hand="broad", tool="filbert 20", pile=darkw, coverage=2.5})
work(rect(770, 950, 150, 150), {hand="glaze", pile=test1, coverage=1.0})

--@ chunk 311
work(rect(400, 1150, 150, 150), {hand="body", pile=test1, coverage=6, fill=true})
work(rect(600, 1150, 150, 150), {hand="hatch", pile=test1, coverage=3.0})
work(rect(800, 1150, 150, 150), {hand="body", pile=test1, coverage=10})
tb2 = brush("filbert", 9)
for i = 0, 5 do
  tb2:load(test1, 1.0)
  for j = 1, 8 do
    tb2:stroke({{415 + j*18, 1155}, {425 + j*18, 1295}}, {pressure=0.9})
  end
end
print("done")

--@ chunk 312
work(rect(380, 1150, 150, 150), {hand="body", pile=test1, coverage=3, fill=true, load=1.0, dips={6, 1.0, 0.0}})
work(rect(380, 1150, 150, 150), {hand="body", pile=test1, coverage=3, fill=true, load=1.0, dips={6, 1.0, 0.0}})
work(rect(380, 1150, 150, 150), {hand="body", pile=test1, coverage=3, fill=true, load=1.0, dips={6, 1.0, 0.0}})
work(rect(560, 1150, 150, 150), {hand="broad", tool="filbert 20", pile=test1, coverage=4, load=1.0, dips={4, 1.0, 0.0}})
work(rect(560, 1150, 150, 150), {hand="broad", tool="filbert 20", pile=test1, coverage=4, load=1.0, dips={4, 1.0, 0.0}})
work(rect(560, 1150, 150, 150), {hand="broad", tool="filbert 20", pile=test1, coverage=4, load=1.0, dips={4, 1.0, 0.0}})
print("ok")

--@ chunk 313
waterp3 = pile{{"lead white", 2.4}, {"pale smalt", 3.4}, {"bone black", 2.0}, {"raw umber", 1.0}, medium=0.03}
for i = 1, 3 do
  work(waterm, {hand="broad", tool="filbert 20", pile=waterp3, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="soft"})
end
fgw = mask(function(x, y) return smoothstep(930, 1120, y + 22 * nz(x, 300)) end)
for i = 1, 2 do
  work(fgw, {hand="broad", tool="filbert 20", pile=waterp3, coverage=2.0, load=1.0, dips={3, 1.0, 0.0}, angle=0.01, clip=true, edge="soft"})
end

--@ chunk 314
wfar = pile{{"lead white", 3.0}, {"pale smalt", 3.6}, {"bone black", 1.4}, {"raw umber", 0.8}, medium=0.03}
wnear = pile{{"lead white", 1.5}, {"pale smalt", 2.6}, {"bone black", 3.0}, {"raw umber", 1.6}, medium=0.03}
wf = mask(function(x, y) return smoothstep(640, 800, y + 18 * nz(x, 700)) * (1 - smoothstep(900, 1080, y + 22 * nz(x, 300))) end)
wn = mask(function(x, y) return smoothstep(930, 1130, y + 22 * nz(x, 300)) end)
for i = 1, 3 do
  work(wf, {hand="broad", tool="filbert 20", pile=wfar, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="soft"})
  work(wn, {hand="broad", tool="filbert 20", pile=wnear, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="soft"})
end

--@ chunk 315
wfar2 = pile{{"lead white", 2.0}, {"pale smalt", 3.0}, {"bone black", 2.2}, {"raw umber", 1.2}, medium=0.02}
wnear2 = pile{{"lead white", 1.2}, {"pale smalt", 2.2}, {"bone black", 3.4}, {"raw umber", 1.8}, medium=0.02}
for i = 1, 3 do
  work(wf, {hand="broad", tool="filbert 20", pile=wfar2, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, pressure={0.45, 0.85}, angle=0.0, clip=true, edge="soft"})
  work(wn, {hand="broad", tool="filbert 20", pile=wnear2, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, pressure={0.45, 0.85}, angle=0.0, clip=true, edge="soft"})
end

--@ chunk 316
mistc = pile{{"lead white", 5.5}, {"pale smalt", 2.4}, {"yellow ochre", 0.45}, medium=0.14}
mistm = mask(function(x, y) return smoothstep(505, 655, y + 22 * nz(x, 300)) * (1 - smoothstep(690, 840, y + 22 * nz(x, 900))) end)
print(string.format("mist %.0f", mistm:area()))
for i = 1, 3 do
  work(mistm, {hand="broad", tool="filbert 22", pile=mistc, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, pressure={0.4, 0.8}, angle=0.0, clip=true, edge="soft"})
end

--@ chunk 317
bankpoly = poly({{0, 1172}, {64, 1180}, {130, 1198}, {198, 1228}, {264, 1252}, {332, 1242},
                    {396, 1212}, {458, 1202}, {518, 1220}, {580, 1250}, {648, 1268}, {718, 1274},
                    {790, 1258}, {864, 1244}, {936, 1256}, {1000, 1276}, {1000, 1389}, {0, 1389}}, true)
bankm2 = bankpoly:roughen(6, 34, 7, 0.6):blur(3)
print(string.format("bank %.0f", bankm2:area()))
bankd = pile{{"raw umber", 3.6}, {"bone black", 2.6}, {"lead white", 1.0}, {"pale smalt", 1.2}, medium=0.03}
for i = 1, 3 do
  work(bankm2, {hand="broad", tool="filbert 20", pile=bankd, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, pressure={0.45, 0.85}, angle=0.06, clip=true, edge="soft"})
end

--@ chunk 318
bankd2 = pile{{"raw umber", 4.4}, {"bone black", 3.4}, {"pale smalt", 0.6}, {"lead white", 0.3}, medium=0.02}
for i = 1, 4 do
  work(bankm2, {hand="broad", tool="filbert 20", pile=bankd2, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, pressure={0.5, 0.9}, angle=0.06, clip=true, edge="soft"})
end

--@ chunk 319
function stand(list, waterline)
  local m = above(function(x) return -900 end)
  for _, t in ipairs(list) do
    local xb, xt, yt, wb, wt, bow = t[1], t[2], t[3], t[4], t[5], t[6]
    m = m + ribbon({{xb, waterline}, {(xb + xt) / 2 + bow, (waterline + yt) / 2}, {xt, yt}},
                   {wb, (wb + wt) / 2, wt})
  end
  return m
end
WT = {{252,258,672,12,7,2},{268,264,744,7,4,-2},{300,306,700,15,9,3},{350,345,716,9,5,-3},
      {372,378,660,13,8,2},{404,400,750,6,4,-2},{460,466,690,11,6,3},{478,473,752,7,4,-2},
      {512,519,706,14,8,-3},{578,573,720,8,5,2},{606,612,662,12,7,-2},{628,624,748,6,4,2},
      {700,707,700,13,8,3},{728,722,754,7,4,-2},{756,763,716,10,6,-3},{842,836,706,12,7,-2},
      {872,879,748,8,5,2},{908,902,684,14,8,3}}
FT = {{336,339,700,5,3,1},{432,429,716,4,3,-1},{545,549,692,6,3,2},
      {664,660,720,5,3,-2},{800,805,700,6,4,1},{938,934,722,5,3,-2}}
wood = stand(WT, 908)
woodf = stand(FT, 882)
print(string.format("wood %.0f  far %.0f", wood:area(), woodf:area()))
woodfar = pile{{"raw umber", 3.0}, {"bone black", 2.0}, {"pale smalt", 1.4}, {"lead white", 0.6}, medium=0.03}
woodd  = pile{{"raw umber", 3.4}, {"bone black", 2.6}, {"pale smalt", 0.8}, {"lead white", 0.3}, medium=0.02}
for i = 1, 3 do
  work(woodf, {hand="body", pile=woodfar, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true})
  work(wood, {hand="body", pile=woodd, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true})
end

--@ chunk 320
BR = {{{{300,702},{322,674},{340,666}}, {4.2,2.8,1.6}}, {{{300,716},{281,694},{269,688}}, {3.6,2.4,1.5}},
      {{{372,672},{398,652},{416,650}}, {4.4,2.8,1.7}}, {{{372,694},{349,680}}, {3.4,2.0}},
      {{{512,714},{540,692},{558,688}}, {4.0,2.6,1.6}}, {{{512,728},{489,710}}, {3.4,2.0}},
      {{{606,672},{634,654}}, {3.8,2.2}}, {{{606,700},{588,688}}, {3.2,1.9}},
      {{{700,708},{726,686},{740,684}}, {4.2,2.7,1.6}}, {{{700,732},{684,720}}, {3.4,2.0}},
      {{{756,724},{780,706}}, {3.6,2.1}}, {{{908,694},{934,672},{952,670}}, {4.4,2.9,1.7}},
      {{{252,682},{238,666}}, {3.6,2.1}}, {{{460,696},{444,678}}, {3.4,2.0}},
      {{{842,712},{864,696}}, {3.8,2.2}}, {{{628,752},{644,740}}, {3.2,1.9}},
      {{{872,752},{858,740}}, {3.0,1.8}}, {{{332,742},{346,730}}, {3.0,1.8}}}
brm = above(function(x) return -900 end)
for _, q in ipairs(BR) do brm = brm + ribbon(q[1], q[2]) end
print(string.format("branches %.0f", brm:area()))
for i = 1, 2 do
  work(wood, {hand="body", pile=woodd, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true})
end
work(brm, {hand="detail", pile=woodd, coverage=1.8, clip=true})

--@ chunk 321
cutm = mask(function(x, y) return y < 906 and 1 or 0 end)
woodflat = wood * cutm
for i = 1, 2 do
  work(woodflat, {hand="glaze", pile=woodd, coverage=1.0, clip=true})
end
work(brm, {hand="detail", pile=woodd, coverage=3.5, load=1.0, dips={5, 1.0, 0.0}, clip=true})
veilp = pile{{"lead white", 4.0}, {"pale smalt", 3.4}, {"raw umber", 0.5}, medium=0.30}
veilm = mask(function(x, y) return smoothstep(608, 700, y + 16 * nz(x, 500)) * (1 - smoothstep(736, 826, y + 16 * nz(x, 900))) end)
for i = 1, 2 do
  work(veilm, {hand="glaze", pile=veilp, coverage=1.1, clip=true})
end

--@ chunk 322
woodflat = wood * cutm
for i = 1, 4 do
  work(woodflat, {hand="body", pile=woodd, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true})
end
work(brm, {hand="detail", pile=woodd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true})
woodmid = pile{{"raw umber", 2.6}, {"bone black", 1.4}, {"pale smalt", 1.8}, {"lead white", 0.8}, medium=0.03}
wtop = mask(function(x, y) return smoothstep(742, 690, y + 12 * nz(x, 500)) end)
wvery = mask(function(x, y) return smoothstep(700, 640, y + 12 * nz(x, 500)) end)
for i = 1, 2 do
  work(woodflat * wtop, {hand="body", pile=woodmid, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true})
  work(woodflat * wvery, {hand="body", pile=woodfar, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true})
end

--@ chunk 323
mistonly = mistm - woodflat - woodf - brm
print(string.format("mistonly %.0f  mistm %.0f", mistonly:area(), mistm:area()))
print("samples", mistonly:at(500, 620), mistonly:at(300, 700), mistonly:at(268, 800), mistonly:at(500, 900))

--@ chunk 324
for i = 1, 3 do
  work(mistonly, {hand="broad", tool="filbert 22", pile=mistc, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, pressure={0.4, 0.8}, angle=0.0, clip=true, edge="soft"})
end
for i = 1, 2 do
  work(woodflat * wtop, {hand="body", pile=woodmid, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true})
end

--@ chunk 325
for i = 1, 3 do
  work(woodflat, {hand="body", pile=woodd, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", pile=woodd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
wtop2 = mask(function(x, y) return smoothstep(724, 668, y + 12 * nz(x, 500)) end)
work(woodflat * wtop2, {hand="body", pile=woodmid, coverage=1.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 326
-- each reflection drawn as its own shape: {x1,y1,w1, x2,y2,w2, x3,y3,w3}
RFL = {
 {254,908,13, 250,985,15, 246,1046,9},
 {270,908,8, 274,962,10, 272,1004,6},
 {302,908,16, 306,1000,19, 302,1062,11},
 {348,908,10, 344,964,8},
 {374,908,14, 378,992,16, 374,1040,9},
 {402,908,7, 398,952,5},
 {462,908,12, 466,986,14, 462,1030,8},
 {476,908,8, 472,950,6},
 {514,908,15, 520,1004,18, 516,1058,10},
 {576,908,9, 572,958,7},
 {608,908,13, 612,996,16, 608,1046,9},
 {626,908,7, 622,948,5},
 {702,908,14, 708,1000,17, 704,1052,10},
 {726,908,8, 722,956,6},
 {758,908,11, 764,992,13, 760,1032,7},
 {840,908,13, 836,980,15, 840,1026,9},
 {870,908,9, 866,950,7},
 {910,908,15, 916,1006,18, 912,1060,10},
}
refl = above(function(x) return -900 end)
for _, r in ipairs(RFL) do
  local pts, wid = {}, {}
  for j = 1, #r, 3 do
    pts[#pts + 1] = {r[j], r[j + 1]}
    wid[#wid + 1] = r[j + 2]
  end
  refl = refl + ribbon(pts, wid)
end
refl = refl:blur(4)
reflp = pile{{"raw umber", 2.6}, {"bone black", 1.8}, {"pale smalt", 1.4}, {"lead white", 0.6}, medium=0.05}
print(string.format("refl %.0f", refl:area()))
work(refl, {hand="body", pile=reflp, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
work(refl, {hand="body", pile=reflp, coverage=1.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 327
function segs(r, cuts)
  local pts = {}
  for j = 1, #r, 3 do pts[#pts + 1] = {r[j], r[j + 1], r[j + 2]} end
  local function at(t)
    local u = t * (#pts - 1)
    local i = math.floor(u) + 1
    if i >= #pts then i = #pts - 1 end
    local f = u - (i - 1)
    return {lerp(pts[i][1], pts[i + 1][1], f), lerp(pts[i][2], pts[i + 1][2], f), lerp(pts[i][3], pts[i + 1][3], f)}
  end
  local m = above(function(x) return -900 end)
  for _, c in ipairs(cuts) do
    local a, b = at(c[1]), at(c[2])
    local m1 = ribbon({{a[1], a[2]}, {(a[1] + b[1]) / 2, (a[2] + b[2]) / 2}, {b[1], b[2]}},
                      {a[3], (a[3] + b[3]) / 2, b[3] * 0.55})
    m = m + m1
  end
  return m
end
CUTS = {{0, 0.34}, {0.42, 0.72}, {0.79, 1.0}}
refl = above(function(x) return -900 end)
for i, r in ipairs(RFL) do
  local sh = ((i % 5) - 2) * 3
  local rr = {}
  for j = 1, #r, 3 do rr[#rr + 1] = r[j] + sh; rr[#rr + 1] = r[j + 1]; rr[#rr + 1] = r[j + 2] end
  refl = refl + segs(rr, CUTS)
end
refl = refl:blur(3)
reflp2 = pile{{"raw umber", 3.0}, {"bone black", 2.4}, {"pale smalt", 1.0}, {"lead white", 0.2}, medium=0.04}
print(string.format("refl %.0f", refl:area()))
for i = 1, 3 do
  work(refl, {hand="body", pile=reflp2, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 328
reflold = above(function(x) return -900 end)
for _, r in ipairs(RFL) do
  local pts, wid = {}, {}
  for j = 1, #r, 3 do pts[#pts + 1] = {r[j], r[j + 1]}; wid[#wid + 1] = r[j + 2] end
  reflold = reflold + ribbon(pts, wid)
end
reflold = reflold:blur(7)
wmid = pile{{"lead white", 1.9}, {"pale smalt", 3.0}, {"bone black", 2.6}, {"raw umber", 1.4}, medium=0.02}
for i = 1, 3 do
  work(reflold, {hand="body", pile=wmid, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="soft"})
end
CUTS = {{0, 0.44}, {0.53, 1.0}}
refl = above(function(x) return -900 end)
for i, r in ipairs(RFL) do
  local sh = ((i % 5) - 2) * 3
  local rr = {}
  for j = 1, #r, 3 do rr[#rr + 1] = r[j] + sh; rr[#rr + 1] = r[j + 1]; rr[#rr + 1] = r[j + 2] end
  refl = refl + segs(rr, CUTS)
end
refl = refl:blur(3)
for i = 1, 3 do
  work(refl, {hand="body", pile=reflp2, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 329
reflband = mask(function(x, y) return smoothstep(884, 902, y) * (1 - smoothstep(1070, 1090, y)) end)
bur1 = reflband * mask(function(x, y) return y < 935 and 1 or 0 end)
bur2 = reflband * mask(function(x, y) return y >= 935 and 1 or 0 end)
for i = 1, 3 do
  work(bur1, {hand="body", pile=wfar2, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(bur2, {hand="body", pile=wnear2, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
reflfull = above(function(x) return -900 end)
for i, r in ipairs(RFL) do
  local sh = ((i % 5) - 2) * 3
  local pts, wid = {}, {}
  for j = 1, #r, 3 do pts[#pts + 1] = {r[j] + sh, r[j + 1]}; wid[#wid + 1] = r[j + 2] end
  reflfull = reflfull + ribbon(pts, wid)
end
gapm = above(function(x) return -900 end)
for _, g in ipairs({{928, 940}, {956, 966}, {984, 996}, {1012, 1022}, {1038, 1050}}) do
  local n = noise{seed=g[1], period=180, octaves=2}
  gapm = gapm + mask(function(x, y)
    local yy = y + 9 * n(x, 40)
    return smoothstep(g[1] - 4, g[1] + 4, yy) * (1 - smoothstep(g[2] - 4, g[2] + 4, yy))
  end)
end
reflb = (reflfull - gapm):blur(3)
reflp3 = pile{{"raw umber", 2.6}, {"bone black", 1.8}, {"pale smalt", 1.6}, {"lead white", 0.4}, medium=0.05}
print(string.format("reflb %.0f", reflband:area()))
for i = 1, 3 do
  work(reflb, {hand="body", pile=reflp3, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 330
wmask = mask(function(x, y) return smoothstep(655, 800, y + 18 * nz(x, 700)) end)
split = mask(function(x, y) return smoothstep(880, 1030, y + 20 * nz(x, 300)) end)
wf = wmask * (-split)
wnn = wmask * split
for i = 1, 3 do
  work(wf, {hand="broad", tool="filbert 20", pile=wfar2, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="soft"})
  work(wnn, {hand="broad", tool="filbert 20", pile=wnear2, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="soft"})
end
for i = 1, 3 do
  work(woodflat, {hand="body", pile=woodd, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", pile=woodd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
print("water and wood redone")

--@ chunk 331
function reflshape(r, seed)
  local n = noise{seed=seed, period=160, octaves=2}
  local pts, wid = {}, {}
  for j = 1, #r, 3 do pts[#pts + 1] = {r[j], r[j + 1]}; wid[#wid + 1] = r[j + 2] end
  local m = ribbon(pts, wid)
  local o1 = (seed * 13) % 19 - 9
  local o2 = (seed * 29) % 25 - 12
  local g1 = mask(function(x, y) local yy = y + 7 * n(x, 30)
    return smoothstep(941 + o1 - 5, 941 + o1 + 5, yy) * (1 - smoothstep(941 + o1 + 7, 941 + o1 + 18, yy)) end)
  local g2 = mask(function(x, y) local yy = y + 7 * n(x, 95)
    return smoothstep(986 + o2 - 5, 986 + o2 + 5, yy) * (1 - smoothstep(986 + o2 + 7, 986 + o2 + 18, yy)) end)
  return m - g1 - g2
end
reflb = above(function(x) return -900 end)
for i, r in ipairs(RFL) do reflb = reflb + reflshape(r, i) end
reflb = reflb:blur(3)
reflp4 = pile{{"raw umber", 2.4}, {"bone black", 1.5}, {"pale smalt", 2.0}, {"lead white", 0.6}, medium=0.06}
print(string.format("reflb %.0f", reflb:area()))
for i = 1, 2 do
  work(reflb, {hand="body", pile=reflp4, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 332
tA = ribbon({{54,-30},{70,300},{84,600},{96,880}}, {50,46,45,54})
tB = ribbon({{113,-30},{117,250},{122,560}}, {16,14,13})
tC = ribbon({{172,-30},{188,300},{200,600},{214,900}}, {62,58,58,70})
nearbody = tA + tB + tC
barkd2 = pile{{"raw umber", 3.4}, {"bone black", 2.6}, {"pale smalt", 0.8}, medium=0.02}
for i = 1, 2 do
  work(nearbody, {hand="body", pile=barkd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
paleA = ribbon({{90,-20},{98,300},{104,600},{112,880}}, {9,8,7,9})
paleC = ribbon({{213,-20},{221,300},{227,600},{235,900}}, {12,11,10,12})
barkpale = pile{{"lead white", 3.4}, {"pale smalt", 3.0}, {"raw umber", 1.0}, medium=0.04}
for i = 1, 2 do
  work(paleA + paleC, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="soft"})
end

--@ chunk 333
PA = {{92,-20,112,8},{95,130,252,7},{99,278,374,8},{101,398,522,6},{105,550,642,8},{108,666,762,7},{112,786,886,9}}
PC = {{215,-20,98,11},{219,122,242,10},{223,266,362,11},{226,386,502,9},{229,526,618,10},{232,642,750,9},{236,774,902,12}}
pale = above(function(x) return -900 end)
for _, s in ipairs(PA) do pale = pale + ribbon({{s[1], s[2]}, {s[1], s[3]}}, {s[4] * 0.8, s[4]}) end
for _, s in ipairs(PC) do pale = pale + ribbon({{s[1], s[2]}, {s[1], s[3]}}, {s[4] * 0.8, s[4]}) end
pale = pale:blur(1.5)
for i = 1, 2 do
  work(pale, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="soft"})
end
SC = {{86,150,16},{80,300,22},{92,430,18},{86,560,24},{96,690,16},{90,800,20},
      {208,90,20},{202,236,26},{210,380,18},{204,520,24},{212,660,20},{206,790,26},
      {64,210,14},{76,470,18},{70,700,16},{196,160,16},{200,430,18},{196,690,20}}
scars = above(function(x) return -900 end)
for _, s in ipairs(SC) do scars = scars + ribbon({{s[1], s[2]}, {s[1] + s[3], s[2] + 1}}, {3.4, 2.6}) end
work(scars, {hand="detail", pile=barkd2, coverage=2.4, clip=true, edge="soft"})

--@ chunk 334
col = rect(74, -20, 34, 940) + rect(202, -20, 44, 940)
col = col - nearbody
print(string.format("col %.0f", col:area()))
function hband(m, a, b) return m * mask(function(x, y) return smoothstep(a, a + 45, y) * (1 - smoothstep(b - 45, b, y)) end) end
work(hband(col, -20, 140), {hand="broad", tool="filbert 18", pile=skyd, coverage=2.6, angle=0.0, clip=true, edge="found"})
work(hband(col, 100, 352), {hand="broad", tool="filbert 18", pile=skym2, coverage=2.3, angle=0.0, clip=true, edge="found"})
work(hband(col, 300, 496), {hand="broad", tool="filbert 18", pile=skyl2, coverage=2.2, angle=0.0, clip=true, edge="found"})
work(hband(col, 452, 624), {hand="broad", tool="filbert 18", pile=skyw2, coverage=2.0, angle=0.0, clip=true, edge="found"})
work(hband(col, 600, 806), {hand="broad", tool="filbert 22", pile=mistc, coverage=2.4, angle=0.0, clip=true, edge="found"})
work(hband(col, 780, 920), {hand="broad", tool="filbert 20", pile=wfar2, coverage=2.4, angle=0.0, clip=true, edge="found"})

--@ chunk 335
PA = {{-20,118,74,80,6},{140,262,81,86,5},{286,386,87,92,6},{410,512,95,99,5},
      {538,648,101,107,6},{672,772,110,114,5},{796,886,116,119,7}}
PC = {{-20,108,197,203,9},{132,250,205,209,8},{274,374,212,216,9},{398,500,218,221,7},
      {524,626,223,229,8},{650,758,232,237,7},{782,902,240,246,9}}
for i, s in ipairs(PA) do
  local ok, err = pcall(function() return ribbon({{s[1], s[2]}, {s[3], s[4]}}, {s[5] * 0.7, s[5]}) end)
  print("PA", i, ok, err)
end
for i, s in ipairs(PC) do
  local ok, err = pcall(function() return ribbon({{s[1], s[2]}, {s[3], s[4]}}, {s[5] * 0.7, s[5]}) end)
  print("PC", i, ok, err)
end

--@ chunk 336
NB = {{{200,96},{250,66},{286,56}}, {{205,150},{262,140},{300,138}}, {{150,60},{108,30},{80,22}},
      {{207,200},{250,214},{276,228}}, {{80,120},{104,110},{118,108}}, {{38,70},{14,52},{0,46}},
      {{122,240},{150,224},{166,218}}}
for i, q in ipairs(NB) do
  local ok, err = pcall(function() return ribbon(q[1], q[2]) end)
  print("NB", i, ok, err)
end

--@ chunk 337
NB = {{{200,96},{250,66},{286,56}}, {{205,150},{262,140},{300,138}}, {{150,60},{108,30},{80,22}},
      {{207,200},{250,214},{276,228}}, {{80,120},{104,110},{118,108}}, {{38,70},{14,52},{0,46}},
      {{122,240},{150,224},{166,218}}}
NW = {{7,4,2}, {6,4,2}, {6,4,2}, {5,3,2}, {6,4,2}, {6,4,2}, {4,3,2}}
pale2 = above(function(x) return -900 end)
for _, s in ipairs(PA) do pale2 = pale2 + ribbon({{s[1], s[2]}, {s[3], s[4]}}, {s[5] * 0.7, s[5]}) end
for _, s in ipairs(PC) do pale2 = pale2 + ribbon({{s[1], s[2]}, {s[3], s[4]}}, {s[5] * 0.7, s[5]}) end
pale2 = pale2:blur(1.2)
for i = 1, 2 do
  work(pale2, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="soft"})
end
nb = above(function(x) return -900 end)
for i, q in ipairs(NB) do nb = nb + ribbon(q, NW[i]) end
nb = nb:blur(1)
for i = 1, 2 do
  work(nb, {hand="detail", pile=barkd2, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="soft"})
end

--@ chunk 338
tz = (rect(112, 24, 540, 700):blur(46)) - nearbody - woodflat - woodf
print(string.format("tz %.0f", tz:area()))
work(hband(tz, -20, 140), {hand="broad", tool="filbert 18", pile=skyd, coverage=2.6, angle=0.0, clip=true, edge="found"})
work(hband(tz, 100, 352), {hand="broad", tool="filbert 18", pile=skym2, coverage=2.3, angle=0.0, clip=true, edge="found"})
work(hband(tz, 300, 496), {hand="broad", tool="filbert 18", pile=skyl2, coverage=2.2, angle=0.0, clip=true, edge="found"})
work(hband(tz, 452, 624), {hand="broad", tool="filbert 18", pile=skyw2, coverage=2.0, angle=0.0, clip=true, edge="found"})
work(hband(tz, 600, 806), {hand="broad", tool="filbert 22", pile=mistc, coverage=2.4, angle=0.0, clip=true, edge="found"})
work(hband(tz, 780, 930), {hand="broad", tool="filbert 20", pile=wfar2, coverage=2.4, angle=0.0, clip=true, edge="found"})

--@ chunk 339
for i = 1, 3 do
  work(nearbody, {hand="body", pile=barkd2, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 2 do
  work(pale2, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", pile=barkd2, coverage=2.4, clip=true, edge="found"})

--@ chunk 340
print(drying(60, 650), drying(180, 650), drying(300, 650), drying(60, 300), drying(180, 300))
for i = 1, 3 do
  work(nearbody, {hand="broad", tool="filbert 20", pile=barkd2, coverage=3.0, load=1.0, dips={3, 1.0, 0.0}, clip=true, edge="found", angle=1.5})
end

--@ chunk 341
print(wait(2 * 24 * 60))
logp = ribbon({{140,1256},{258,1274},{362,1252},{462,1226},{560,1238},{646,1266}}, {18,22,19,17,20,14})
work(logp, {hand="body", pile=barkpale, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
logd = ribbon({{132,1266},{252,1284},{358,1262},{458,1236},{556,1248},{640,1274}}, {10,13,11,10,12,8})
work(logd, {hand="body", pile=barkd2, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
figd = pile{{"raw umber", 3.0}, {"bone black", 3.0}, {"pale smalt", 0.5}, medium=0.02}
fig = ellipse(700, 1161, 7, 8.5) + ribbon({{700,1168},{702,1194}}, {19, 23})
fig = fig + poly({{687,1192},{713,1192},{719,1258},{681,1258}}, true)
work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 342
streak = (ribbon({{195,120},{490,510},{800,905}}, {26,24,30}) - nearbody - woodflat - woodf):blur(8)
print(string.format("streak %.0f", streak:area()))
work(hband(streak, 60, 170), {hand="broad", tool="filbert 18", pile=skyd, coverage=2.6, angle=0.0, clip=true, edge="found"})
work(hband(streak, 110, 352), {hand="broad", tool="filbert 18", pile=skym2, coverage=2.3, angle=0.0, clip=true, edge="found"})
work(hband(streak, 300, 496), {hand="broad", tool="filbert 18", pile=skyl2, coverage=2.2, angle=0.0, clip=true, edge="found"})
work(hband(streak, 452, 624), {hand="broad", tool="filbert 18", pile=skyw2, coverage=2.0, angle=0.0, clip=true, edge="found"})
work(hband(streak, 600, 806), {hand="broad", tool="filbert 22", pile=mistc, coverage=2.4, angle=0.0, clip=true, edge="found"})
work(hband(streak, 780, 960), {hand="broad", tool="filbert 20", pile=wfar2, coverage=2.4, angle=0.0, clip=true, edge="found"})

--@ chunk 343
upper = rect(-10, -30, 1020, 945) - nearbody - woodflat - woodf - brm - nb
print(string.format("upper %.0f", upper:area()))
for i = 1, 2 do
  work(hband(upper, 60, 150), {hand="broad", tool="filbert 18", pile=skyd, coverage=2.6, angle=0.0, clip=true, edge="found"})
  work(hband(upper, 110, 352), {hand="broad", tool="filbert 18", pile=skym2, coverage=2.4, angle=0.0, clip=true, edge="found"})
  work(hband(upper, 300, 496), {hand="broad", tool="filbert 18", pile=skyl2, coverage=2.6, angle=0.0, clip=true, edge="found"})
  work(hband(upper, 452, 624), {hand="broad", tool="filbert 18", pile=skyw2, coverage=2.6, angle=0.0, clip=true, edge="found"})
  work(hband(upper, 600, 806), {hand="broad", tool="filbert 22", pile=mistc, coverage=2.6, angle=0.0, clip=true, edge="found"})
end
work(hband(upper, 780, 912), {hand="broad", tool="filbert 20", pile=wfar2, coverage=2.4, angle=0.0, clip=true, edge="found"})

--@ chunk 344
print(wait(24 * 60))
blob = (rect(580, 690, 260, 240):blur(14)) - woodflat - woodf - brm
for i = 1, 3 do
  work(hband(blob, 640, 800), {hand="broad", tool="filbert 22", pile=mistc, coverage=2.6, angle=0.0, clip=true, edge="found"})
  work(hband(blob, 770, 940), {hand="broad", tool="filbert 20", pile=wfar2, coverage=2.8, angle=0.0, clip=true, edge="found"})
end

--@ chunk 345
print(drying(750, 835))
testp = pile{{"red earth", 5}, {"yellow ochre", 2}}
work(rect(735, 818, 30, 34), {hand="body", pile=testp, coverage=4, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
print("mask at 750,835", (rect(580,690,260,240):blur(14) - woodflat - woodf - brm):at(750, 835))
print("streakmask", streak:at(750, 835))

--@ chunk 346
sr = ribbon({{686,764},{744,832},{802,900}}, {34,34,34})
sr = sr - woodflat - woodf - brm
print(string.format("sr %.0f", sr:area()))
for i = 1, 4 do
  work(hband(sr, 700, 812), {hand="hatch", tool="round 2.6", pile=mistc, coverage=4.0, load=1.0, dips={5, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(sr, 760, 940), {hand="hatch", tool="round 2.6", pile=wfar2, coverage=4.0, load=1.0, dips={5, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 347
print(wait(2 * 24 * 60))
waterzone = mask(function(x, y)
  return smoothstep(660, 800, y + 18 * nz(x, 700)) * (1 - smoothstep(1170, 1215, y + 14 * nz(x, 200)))
end)
wz = waterzone - nearbody
sp = mask(function(x, y) return smoothstep(840, 1010, y + 20 * nz(x, 300)) end)
wzu = wz * (-sp)
wzl = wz * sp
for i = 1, 3 do
  work(wzu, {hand="broad", tool="filbert 20", pile=wfar2, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wzl, {hand="broad", tool="filbert 20", pile=wnear2, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 348
wfront = pile{{"lead white", 2.4}, {"pale smalt", 3.6}, {"bone black", 1.5}, {"raw umber", 1.1}, medium=0.03}
wfz = mask(function(x, y) return smoothstep(990, 1130, y + 18 * nz(x, 300)) end)
for i = 1, 3 do
  work(wfz, {hand="broad", tool="filbert 20", pile=wfront, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
bankd3 = pile{{"raw umber", 4.6}, {"bone black", 3.6}, {"pale smalt", 0.5}, {"lead white", 0.2}, medium=0.02}
for i = 1, 3 do
  work(bankm2, {hand="broad", tool="filbert 20", pile=bankd3, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end
work(logp, {hand="body", pile=barkpale, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
work(logd, {hand="body", pile=barkd2, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 349
wmid2 = pile{{"lead white", 2.6}, {"pale smalt", 3.6}, {"bone black", 1.6}, {"raw umber", 1.2}, medium=0.03}
wmz = mask(function(x, y) return smoothstep(845, 985, y + 22 * nz(x, 300)) * (1 - smoothstep(1210, 1268, y + 16 * nz(x, 200))) end)
for i = 1, 3 do
  work(wmz, {hand="broad", tool="filbert 20", pile=wmid2, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 2 do
  work(bankm2, {hand="broad", tool="filbert 20", pile=bankd3, coverage=2.8, load=1.0, dips={3, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end

--@ chunk 350
bankd4 = pile{{"raw umber", 4.0}, {"bone black", 4.5}, {"pale smalt", 0.3}, medium=0.01}
for i = 1, 3 do
  work(bankm2, {hand="hatch", tool="round 2.6", pile=bankd4, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end
for i = 1, 3 do
  work(woodflat, {hand="body", pile=woodd, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", pile=woodd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 2 do
  work(reflb, {hand="body", pile=reflp4, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 351
function reflone(x, y0, y1, w, sd)
  local m = ribbon({{x, y0}, {x + 2.5, (y0 + y1) / 2}, {x - 3, y1}}, {w * 0.85, w * 1.05, w * 0.6})
  local n = noise{seed=sd, period=150, octaves=2}
  local gy = y0 + (y1 - y0) * (0.34 + 0.03 * (sd % 7))
  local g = mask(function(xx, yy)
    local z = yy + 9 * n(xx, 30)
    return smoothstep(gy - 6, gy + 6, z) * (1 - smoothstep(gy + 9, gy + 24, z))
  end)
  return (m - g):blur(4)
end
reflnew = above(function(x) return -900 end)
for i, t in ipairs(WT) do
  local len = (906 - t[3]) * 0.72
  reflnew = reflnew + reflone(t[1] + 1, 906, 906 + len, t[4], i)
end
for i, t in ipairs(FT) do
  reflnew = reflnew + reflone(t[1], 882, 882 + (882 - t[3]) * 0.5, t[4] * 0.9, 30 + i)
end
reflnew = reflnew:blur(2)
print(string.format("reflnew %.0f", reflnew:area()))
reflp5 = pile{{"raw umber", 2.8}, {"bone black", 2.0}, {"pale smalt", 1.6}, {"lead white", 0.5}, medium=0.05}
for i = 1, 3 do
  work(reflnew, {hand="body", pile=reflp5, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 352
rb = (mask(function(x, y) return smoothstep(878, 906, y) * (1 - smoothstep(1088, 1122, y)) end) - woodflat)
for i = 1, 3 do
  work(rb, {hand="body", pile=wmid2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 3 do
  work(reflnew, {hand="body", pile=reflp5, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 353
wz2 = mask(function(x, y)
  return smoothstep(655, 800, y + 18 * nz(x, 700)) * (1 - smoothstep(1170, 1228, y + 14 * nz(x, 200)))
end)
sp1 = mask(function(x, y) return smoothstep(860, 1090, y + 22 * nz(x, 300)) end)
sp2 = mask(function(x, y) return smoothstep(1050, 1275, y + 18 * nz(x, 250)) end)
b1 = wz2 * (-sp1)
b2 = wz2 * (sp1 * (-sp2))
b3 = wz2 * sp2
work(b1 - nearbody - woodflat - woodf, {hand="broad", tool="filbert 20", pile=wmid2, coverage=1.5, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
work(b2 - nearbody - woodflat - woodf, {hand="broad", tool="filbert 20", pile=wmid2, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
work(b3 - nearbody - woodflat - woodf, {hand="broad", tool="filbert 20", pile=wmid2, coverage=3.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})

--@ chunk 354
for i = 1, 3 do
  work(reflnew, {hand="body", pile=reflp5, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
ripzone = wz2 - nearbody - woodflat - woodf - bankm2
ripp = pile{{"lead white", 3.0}, {"pale smalt", 2.6}, medium=0.32}
ripd = pile{{"raw umber", 2.4}, {"bone black", 1.4}, {"pale smalt", 1.2}, medium=0.32}
work(ripzone, {hand="glaze", pile=ripp, coverage=0.9, angle=0.0, length={90, 250}, clip=true, edge="found"})
work(ripzone, {hand="glaze", pile=ripd, coverage=0.7, angle=0.0, length={110, 280}, clip=true, edge="found"})

--@ chunk 355
bedge = bankm2 - bankm2:shrink(32)
print(string.format("bedge %.0f", bedge:area()))
banklit = pile{{"raw umber", 2.6}, {"lead white", 1.5}, {"pale smalt", 1.2}, {"green earth", 0.9}, medium=0.03}
for i = 1, 3 do
  work(bedge, {hand="body", pile=banklit, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 3 do
  work(logp, {hand="body", pile=barkpale, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(logd, {hand="body", pile=barkd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 3 do
  work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
end
fighl = ribbon({{708,1160},{712,1195},{717,1252}}, {2.6,3.2,3.6})
work(fighl, {hand="detail", pile=barkpale, coverage=2.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 356
for i = 1, 3 do
  work(bankm2, {hand="hatch", tool="round 2.6", pile=bankd4, coverage=3.5, load=1.0, dips={5, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end
for i = 1, 2 do
  work(bedge, {hand="body", pile=banklit, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
logp3 = ribbon({{118,1256},{246,1278},{366,1250},{478,1222},{586,1240},{662,1268}}, {26,30,25,19,15,10})
logd3 = ribbon({{124,1268},{248,1290},{368,1262},{480,1234},{586,1252},{652,1276}}, {10,12,10,8,6,4})
work(logd3, {hand="body", pile=barkd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(logp3, {hand="body", pile=barkpale, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
STUB = {{{302,1262},{326,1236},{338,1230}}, {{432,1236},{452,1214},{460,1212}}, {{202,1266},{180,1250},{174,1248}}}
SW = {{9,6,3}, {8,5,3}, {8,5,3}}
stubs = above(function(x) return -900 end)
for i, q in ipairs(STUB) do stubs = stubs + ribbon(q, SW[i]) end
for i = 1, 2 do
  work(stubs, {hand="body", pile=barkpale, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
LSC = {{168,1250,26},{214,1266,30},{286,1262,24},{332,1242,20},{398,1236,18},{452,1214,16},{512,1222,14},{572,1238,12},{620,1252,10}}
lscars = above(function(x) return -900 end)
for _, s in ipairs(LSC) do lscars = lscars + ribbon({{s[1], s[2]}, {s[1] + s[3] * 0.7, s[2] + 3}}, {4.0, 3.0}) end
work(lscars, {hand="detail", pile=barkd2, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 357
abovebank = (mask(function(x, y) return smoothstep(1110, 1175, y + 12 * nz(x, 200)) * (1 - smoothstep(1265, 1320, y + 12 * nz(x, 200))) end) - bankm2 - fig)
for i = 1, 3 do
  work(abovebank, {hand="broad", tool="filbert 20", pile=wmid2, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 3 do
  work(bankm2, {hand="hatch", tool="round 2.6", pile=bankd4, coverage=3.5, load=1.0, dips={5, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end
logp4 = ribbon({{112,1262},{244,1284},{366,1258},{478,1240},{586,1264}}, {26,30,25,20,14})
logd4 = ribbon({{118,1274},{248,1296},{368,1270},{480,1252},{584,1274}}, {10,12,10,8,5})
work(logd4, {hand="body", pile=barkd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(logp4, {hand="body", pile=barkpale, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 358
tA = ribbon({{54,-30},{70,300},{84,600},{96,900}}, {50,46,45,54})
tB = ribbon({{113,-30},{117,300},{124,650},{130,900}}, {16,14,13,15})
tC = ribbon({{172,-30},{188,300},{200,600},{214,910}}, {62,58,58,68})
nearbody3 = (tA + tB + tC) * mask(function(x, y) return y < 957 and 1 or 0 end)
print(string.format("nearbody3 %.0f", nearbody3:area()))
for i = 1, 3 do
  work(nearbody3, {hand="body", pile=barkd2, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 2 do
  work(pale2, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nb, {hand="detail", pile=barkd2, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
  work(scars, {hand="detail", pile=barkd2, coverage=2.2, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
  work(bankm2, {hand="hatch", tool="round 2.6", pile=bankd4, coverage=3.5, load=1.0, dips={5, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end

--@ chunk 359
footm = nearbody3 * mask(function(x, y) return smoothstep(858, 900, y) end)
for i = 1, 2 do
  work(footm, {hand="body", pile=barkd2, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
nrefl = reflone(96, 957, 1157, 44, 7) + reflone(131, 957, 1097, 13, 11) + reflone(214, 957, 1177, 60, 13)
nrefl = nrefl:blur(5)
print(string.format("nrefl %.0f", nrefl:area()))
for i = 1, 3 do
  work(nrefl, {hand="body", pile=reflp5, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 360
corridor = (ribbon({{83,62},{417,417},{799,896}}, {30,30,34}) + ribbon({{30,30},{210,230}}, {18,20}))
corridor = corridor - woodflat - woodf - nearbody3 - brm - nb
print(string.format("corridor %.0f", corridor:area()))
blend(corridor, {angle=0.62})

--@ chunk 361
print(wait(24 * 60))
corridor = (ribbon({{83,62},{417,417},{799,896}}, {30,30,34}) + ribbon({{30,30},{210,230}}, {18,20})) - woodflat - woodf - brm - nb
stipple(hband(corridor, -20, 160), {pile=skyd, width=13, coverage=26, pressure={0.55, 0.95}})
stipple(hband(corridor, 110, 352), {pile=skym2, width=13, coverage=26, pressure={0.55, 0.95}})
stipple(hband(corridor, 300, 496), {pile=skyl2, width=13, coverage=26, pressure={0.55, 0.95}})
stipple(hband(corridor, 452, 624), {pile=skyw2, width=13, coverage=26, pressure={0.55, 0.95}})
stipple(hband(corridor, 600, 806), {pile=mistc, width=13, coverage=26, pressure={0.55, 0.95}})
stipple(hband(corridor, 780, 960), {pile=wmid2, width=13, coverage=26, pressure={0.55, 0.95}})

--@ chunk 362
heavy = brush("filbert", 24)
cor2 = ribbon({{83,62},{417,417},{799,896}}, {40,40,44})
for i = 1, 4 do
  heavy:load(skym2, 1.0)
  heavy:stroke({{83,62},{417,417},{799,896}}, {pressure={1.0,1.0,1.0}, clip=hband(cor2, -40, 460)})
  heavy:load(mistc, 1.0)
  heavy:stroke({{83,62},{417,417},{799,896}}, {pressure={1.0,1.0,1.0}, clip=hband(cor2, 430, 780)})
  heavy:load(wmid2, 1.0)
  heavy:stroke({{83,62},{417,417},{799,896}}, {pressure={1.0,1.0,1.0}, clip=hband(cor2, 750, 940)})
end
print("heavy strokes done")

--@ chunk 363
vtest = pile{{"vermilion", 5}}
for i = 1, 3 do
  work(hband(cor2, 260, 520), {hand="body", pile=vtest, coverage=4.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
print("drying on streak", drying(450, 400), "beside", drying(350, 400), drying(600, 400))

--@ chunk 364
skyd2 = pile{{"lead white", 3.2}, {"raw umber", 2.0}, {"pale smalt", 0.6}, medium=0.02}
skym2o = pile{{"lead white", 4.0}, {"raw umber", 1.4}, {"pale smalt", 0.8}, medium=0.02}
skyl2o = pile{{"lead white", 4.6}, {"raw umber", 0.9}, {"pale smalt", 1.0}, medium=0.02}
skyw2o = pile{{"lead white", 5.0}, {"yellow ochre", 1.2}, {"red earth", 0.5}, {"raw umber", 0.4}, medium=0.03}
misto = pile{{"lead white", 5.5}, {"yellow ochre", 0.5}, {"raw umber", 0.3}, medium=0.05}
allup = rect(-10, -30, 1020, 950)
for i = 1, 3 do
  work(hband(allup, -30, 150), {hand="broad", tool="filbert 18", pile=skyd2, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 110, 352), {hand="broad", tool="filbert 18", pile=skym2o, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 300, 496), {hand="broad", tool="filbert 18", pile=skyl2o, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 452, 624), {hand="broad", tool="filbert 18", pile=skyw2o, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 600, 806), {hand="broad", tool="filbert 22", pile=misto, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 365
watero = pile{{"lead white", 2.8}, {"raw umber", 2.6}, {"bone black", 1.0}, {"pale smalt", 0.8}, medium=0.02}
waterf = pile{{"lead white", 2.2}, {"raw umber", 3.0}, {"bone black", 1.5}, {"pale smalt", 0.7}, medium=0.02}
wa = mask(function(x, y) return smoothstep(700, 830, y + 18 * nz(x, 700)) * (1 - smoothstep(900, 1120, y + 22 * nz(x, 300))) end)
wb = mask(function(x, y) return smoothstep(940, 1180, y + 22 * nz(x, 300)) * (1 - smoothstep(1210, 1290, y + 16 * nz(x, 200))) end)
for i = 1, 3 do
  work(wa, {hand="broad", tool="filbert 20", pile=watero, coverage=2.2, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb, {hand="broad", tool="filbert 20", pile=waterf, coverage=2.2, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 366
skyd3 = pile{{"lead white", 3.0}, {"pale smalt", 2.2}, {"raw umber", 1.8}, medium=0.02}
skym3 = pile{{"lead white", 3.6}, {"pale smalt", 2.6}, {"raw umber", 1.2}, medium=0.02}
skyl3 = pile{{"lead white", 4.2}, {"pale smalt", 2.8}, {"raw umber", 0.7}, medium=0.02}
skyw3 = pile{{"lead white", 4.6}, {"pale smalt", 2.2}, {"yellow ochre", 0.8}, {"red earth", 0.3}, medium=0.03}
mist3 = pile{{"lead white", 5.2}, {"pale smalt", 1.6}, {"yellow ochre", 0.25}, medium=0.04}
watero2 = pile{{"lead white", 3.2}, {"pale smalt", 2.0}, {"bone black", 1.4}, medium=0.02}
waterf2 = pile{{"lead white", 2.6}, {"pale smalt", 2.0}, {"bone black", 2.0}, medium=0.02}
for i = 1, 3 do
  work(hband(allup, -30, 150), {hand="broad", tool="filbert 18", pile=skyd3, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 110, 352), {hand="broad", tool="filbert 18", pile=skym3, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 300, 496), {hand="broad", tool="filbert 18", pile=skyl3, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 452, 624), {hand="broad", tool="filbert 18", pile=skyw3, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 600, 812), {hand="broad", tool="filbert 22", pile=mist3, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wa, {hand="broad", tool="filbert 20", pile=watero2, coverage=2.2, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb, {hand="broad", tool="filbert 20", pile=waterf2, coverage=2.2, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 367
twA = pile{{"raw umber", 3.0}, {"bone black", 2.6}, {"lead white", 1.6}, {"pale smalt", 1.0}, medium=0.02}
twB = pile{{"raw umber", 2.0}, {"bone black", 1.4}, {"lead white", 2.6}, {"pale smalt", 1.6}, medium=0.02}
twC = pile{{"raw umber", 1.2}, {"bone black", 0.8}, {"lead white", 3.6}, {"pale smalt", 2.0}, medium=0.02}
for i = 1, 3 do
  work(rect(180, 1000, 130, 130), {hand="broad", tool="filbert 20", pile=twA, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(rect(360, 1000, 130, 130), {hand="broad", tool="filbert 20", pile=twB, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(rect(540, 1000, 130, 130), {hand="broad", tool="filbert 20", pile=twC, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
print("tests laid")

--@ chunk 368
sd4 = pile{{"lead white", 2.6}, {"pale smalt", 1.8}, {"bone black", 1.2}, medium=0.02}
sm4 = pile{{"lead white", 3.4}, {"pale smalt", 2.0}, {"bone black", 0.7}, medium=0.02}
sl4 = pile{{"lead white", 4.0}, {"pale smalt", 1.8}, {"bone black", 0.3}, medium=0.02}
sw4 = pile{{"lead white", 4.4}, {"pale smalt", 1.2}, {"yellow ochre", 1.0}, {"red earth", 0.3}, medium=0.03}
mi4 = pile{{"lead white", 5.4}, {"pale smalt", 1.4}, {"yellow ochre", 0.2}, medium=0.04}
for i = 1, 3 do
  work(hband(allup, -30, 150), {hand="broad", tool="filbert 18", pile=sd4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 110, 352), {hand="broad", tool="filbert 18", pile=sm4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 300, 496), {hand="broad", tool="filbert 18", pile=sl4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 452, 624), {hand="broad", tool="filbert 18", pile=sw4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 600, 812), {hand="broad", tool="filbert 22", pile=mi4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 369
wf4 = pile{{"lead white", 3.4}, {"pale smalt", 2.2}, {"bone black", 0.9}, medium=0.02}
wm4 = pile{{"lead white", 2.6}, {"pale smalt", 2.0}, {"bone black", 1.5}, medium=0.02}
wn4 = pile{{"lead white", 2.0}, {"pale smalt", 1.9}, {"bone black", 2.0}, medium=0.02}
for i = 1, 3 do
  work(wa, {hand="broad", tool="filbert 20", pile=wf4, coverage=2.2, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb, {hand="broad", tool="filbert 20", pile=wm4, coverage=2.2, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(mask(function(x, y) return smoothstep(1120, 1300, y + 18 * nz(x, 250)) * (1 - smoothstep(1230, 1330, y + 16 * nz(x, 200))) end),
       {hand="broad", tool="filbert 20", pile=wn4, coverage=2.2, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 370
for i = 1, 3 do
  work(woodflat, {hand="body", pile=woodd, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nearbody3, {hand="body", pile=barkd2, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 3 do
  work(woodf, {hand="body", pile=woodfar, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(brm, {hand="detail", pile=woodd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
  work(nb, {hand="detail", pile=barkd2, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
  work(pale2, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", pile=barkd2, coverage=2.2, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 371
for i = 1, 3 do
  work(reflnew, {hand="body", pile=reflp5, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl, {hand="body", pile=reflp5, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(bedge, {hand="body", pile=banklit, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(logd4, {hand="body", pile=barkd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(logp4, {hand="body", pile=barkpale, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(stubs, {hand="body", pile=barkpale, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(lscars, {hand="detail", pile=barkd2, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
end
work(fighl, {hand="detail", pile=barkpale, coverage=2.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 372
print(wait(3 * 24 * 60))
for _, p in ipairs({{500,100},{500,400},{500,700},{500,950},{500,1150},{60,650},{700,1250}}) do
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 373
print(wait(7 * 24 * 60))
for _, p in ipairs({{500,100},{500,400},{500,700},{500,950},{500,1150},{60,650},{700,1250},{300,1250}}) do
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 374
print(wait(10 * 24 * 60))
for _, p in ipairs({{500,100},{500,400},{500,700},{500,950},{500,1150},{60,650},{700,1250}}) do
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 375
wfB = pile{{"lead white", 3.2}, {"pale smalt", 1.8}, {"raw umber", 1.2}, {"bone black", 0.2}, medium=0.02}
wmB = pile{{"lead white", 2.6}, {"pale smalt", 1.6}, {"raw umber", 2.0}, {"bone black", 0.4}, medium=0.02}
wnB = pile{{"lead white", 2.0}, {"pale smalt", 1.5}, {"raw umber", 2.6}, {"bone black", 0.8}, medium=0.02}
wnzone = mask(function(x, y) return smoothstep(1110, 1290, y + 18 * nz(x, 250)) * (1 - smoothstep(1225, 1320, y + 16 * nz(x, 200))) end)
for i = 1, 3 do
  work(wa, {hand="broad", tool="filbert 20", pile=wfB, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb, {hand="broad", tool="filbert 20", pile=wmB, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wnzone, {hand="broad", tool="filbert 20", pile=wnB, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 376
wfC = pile{{"lead white", 3.0}, {"pale smalt", 1.8}, {"bone black", 2.4}, medium=0.02}
wmC = pile{{"lead white", 2.4}, {"pale smalt", 1.6}, {"bone black", 3.2}, medium=0.02}
wnC = pile{{"lead white", 1.9}, {"pale smalt", 1.5}, {"bone black", 4.0}, medium=0.02}
for i = 1, 3 do
  work(wa, {hand="broad", tool="filbert 20", pile=wfC, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb, {hand="broad", tool="filbert 20", pile=wmC, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wnzone, {hand="broad", tool="filbert 20", pile=wnC, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 377
woodd2 = pile{{"raw umber", 3.2}, {"bone black", 4.0}, {"pale smalt", 0.4}, medium=0.01}
woodfar2 = pile{{"raw umber", 3.0}, {"bone black", 2.6}, {"pale smalt", 1.0}, {"lead white", 0.4}, medium=0.02}
barkd3 = pile{{"raw umber", 3.2}, {"bone black", 4.2}, {"pale smalt", 0.3}, medium=0.01}
reflp6 = pile{{"raw umber", 2.6}, {"bone black", 2.6}, {"pale smalt", 0.8}, {"lead white", 0.2}, medium=0.03}
for i = 1, 3 do
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodf, {hand="body", pile=woodfar2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", pile=woodd2, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(reflnew, {hand="body", pile=reflp6, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl, {hand="body", pile=reflp6, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 3 do
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 378
print(wait(5 * 24 * 60))
for _, p in ipairs({{500,900},{500,1100},{300,1250},{700,1250},{60,650}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 379
print(wait(6 * 24 * 60))
for _, p in ipairs({{500,900},{500,1100},{300,1250},{700,1250}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 380
for i = 1, 3 do
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(bedge, {hand="body", pile=banklit, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 2 do
  work(pale2, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nb, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
  work(logp4, {hand="body", pile=barkpale, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", pile=barkd3, coverage=2.2, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
work(lscars, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
end
work(fighl, {hand="detail", pile=barkpale, coverage=2.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 381
cor2w = ribbon({{83,62},{417,417},{799,896}}, {46,46,50})
test_area = hband(cor2w, 40, 500) - nearbody3 - nb
print(string.format("test_area %.0f", test_area:area()))
blend(test_area, {angle=0.62})

--@ chunk 382
corridorfull = ribbon({{83,62},{417,417},{799,896}}, {60,60,64}) + ribbon({{20,20},{240,260}}, {44,48})
corridorfull = corridorfull - nearbody3 - woodflat - woodf - nb
print(string.format("corridorfull %.0f", corridorfull:area()))
for i = 1, 3 do
  work(corridorfull, {hand="hatch", tool="round 2.6", pile=bankd4, coverage=4.0, load=1.0, dips={5, 1.0, 0.0}, angle=0.9, clip=true, edge="found"})
end
print(wait(3 * 24 * 60))

--@ chunk 383
function cover(m, p)
  work(m, {hand="hatch", tool="round 2.6", pile=p, coverage=3.5, load=1.0, dips={5, 1.0, 0.0}, angle=0.35, clip=true, edge="found"})
  work(m, {hand="hatch", tool="round 2.6", pile=p, coverage=3.5, load=1.0, dips={5, 1.0, 0.0}, angle=-0.4, clip=true, edge="found"})
  work(m, {hand="broad", tool="filbert 20", pile=p, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
cover(hband(corridorfull, -30, 152), sd4)
cover(hband(corridorfull, 108, 354), sm4)
cover(hband(corridorfull, 298, 498), sl4)
cover(hband(corridorfull, 450, 626), sw4)
cover(hband(corridorfull, 598, 814), mi4)
cover(hband(corridorfull, 700, 912), wfC)

--@ chunk 384
sdo = pile{{"lead white", 2.7}, {"bone black", 1.6}, {"cobalt blue", 0.22}, medium=0.02}
smo = pile{{"lead white", 3.4}, {"bone black", 0.9}, {"cobalt blue", 0.18}, medium=0.02}
slo = pile{{"lead white", 4.1}, {"bone black", 0.35}, {"cobalt blue", 0.14}, medium=0.02}
swo = pile{{"lead white", 4.4}, {"yellow ochre", 1.0}, {"red earth", 0.3}, {"bone black", 0.25}, medium=0.03}
mio = pile{{"lead white", 5.4}, {"yellow ochre", 0.3}, {"bone black", 0.15}, medium=0.04}
for i = 1, 3 do
  work(hband(allup, -30, 152), {hand="broad", tool="filbert 18", pile=sdo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 108, 354), {hand="broad", tool="filbert 18", pile=smo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 298, 498), {hand="broad", tool="filbert 18", pile=slo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 450, 626), {hand="broad", tool="filbert 18", pile=swo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 598, 814), {hand="broad", tool="filbert 22", pile=mio, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 385
wfo = pile{{"lead white", 3.2}, {"bone black", 2.2}, {"cobalt blue", 0.2}, medium=0.02}
wmo = pile{{"lead white", 2.5}, {"bone black", 3.2}, {"cobalt blue", 0.22}, medium=0.02}
wno = pile{{"lead white", 2.0}, {"bone black", 4.2}, {"cobalt blue", 0.24}, medium=0.02}
for i = 1, 3 do
  work(wa, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wnzone, {hand="broad", tool="filbert 20", pile=wno, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(bankm2, {hand="broad", tool="filbert 20", pile=bankd4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end

--@ chunk 386
for i = 1, 3 do
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodf, {hand="body", pile=woodfar2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", pile=woodd2, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(reflnew, {hand="body", pile=reflp6, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl, {hand="body", pile=reflp6, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 387
for i = 1, 3 do
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(bedge, {hand="body", pile=banklit, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(logp4, {hand="body", pile=barkpale, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 2 do
  work(pale2, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nb, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
  work(stubs, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", pile=barkd3, coverage=2.2, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
work(lscars, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
end
work(fighl, {hand="detail", pile=barkpale, coverage=2.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 388
print(wait(10 * 24 * 60))
for _, p in ipairs({{500,300},{500,700},{500,950},{500,1150},{60,650},{300,400}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 389
for i = 1, 4 do
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 390
print(wait(3 * 24 * 60))
print(drying(60, 400), drying(300, 700))
for i = 1, 2 do
  work(pale2, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nb, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", pile=barkd3, coverage=2.2, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 391
function hatchcover(m, p)
  for k = 1, 3 do
    work(m, {hand="hatch", tool="round 2.6", pile=p, coverage=4.0, load=1.0, dips={5, 1.0, 0.0}, angle=(k - 2) * 0.7, clip=true, edge="found"})
  end
  work(m, {hand="broad", tool="filbert 20", pile=p, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
hatchcover(hband(corridorfull, -30, 152), sdo)
hatchcover(hband(corridorfull, 108, 354), smo)
hatchcover(hband(corridorfull, 298, 498), slo)
hatchcover(hband(corridorfull, 450, 626), swo)
hatchcover(hband(corridorfull, 598, 814), mio)
hatchcover(hband(corridorfull, 700, 912), wfo)

--@ chunk 392
function stipp(m, p, cov)
  stipple(m, {pile=p, width=12, coverage=cov, pressure={0.7, 1.0}, cluster=0.4, seed=17})
end
stipp(hband(corridorfull, -30, 152), sdo, 44)
stipp(hband(corridorfull, 108, 354), smo, 44)
stipp(hband(corridorfull, 298, 498), slo, 44)
stipp(hband(corridorfull, 450, 626), swo, 44)
stipp(hband(corridorfull, 598, 814), mio, 44)
stipp(hband(corridorfull, 700, 912), wfo, 44)

--@ chunk 393
cor3 = ribbon({{40,50},{250,300},{420,500},{600,720},{760,900}}, {130,130,130,130,130})
cor3 = cor3 + ribbon({{5,5},{215,265},{385,465},{565,685},{725,865}}, {90,90,90,90,90})
cor3 = cor3 - nearbody3 - woodflat - woodf - nb
print(string.format("cor3 %.0f", cor3:area()))
print("samples", cor3:at(250,300), cor3:at(420,500), cor3:at(600,720), cor3:at(300,360))
for i = 1, 3 do
  work(cor3, {hand="hatch", tool="round 2.6", pile=bankd4, coverage=4.0, load=1.0, dips={5, 1.0, 0.0}, angle=0.9, clip=true, edge="found"})
end

--@ chunk 394
cor4 = (cor3:grow(40)) - nearbody3 - woodflat - woodf - nb
for i = 1, 2 do
  work(cor4, {hand="hatch", tool="round 2.6", pile=bankd4, coverage=4.0, load=1.0, dips={5, 1.0, 0.0}, angle=1.4, clip=true, edge="found"})
end
hatchcover(hband(cor4, -30, 152), sdo)
hatchcover(hband(cor4, 108, 354), smo)
hatchcover(hband(cor4, 298, 498), slo)
hatchcover(hband(cor4, 450, 626), swo)
hatchcover(hband(cor4, 598, 814), mio)
hatchcover(hband(cor4, 700, 940), wfo)

--@ chunk 395
allup = rect(-10, -30, 1020, 1010)
for i = 1, 3 do
  work(hband(allup, -30, 152), {hand="broad", tool="filbert 18", pile=sdo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 108, 354), {hand="broad", tool="filbert 18", pile=smo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 298, 498), {hand="broad", tool="filbert 18", pile=slo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 450, 626), {hand="broad", tool="filbert 18", pile=swo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 598, 814), {hand="broad", tool="filbert 22", pile=mio, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 396
bowl = ellipse(745, 945, 140, 58):blur(10)
for i = 1, 3 do
  work(wa, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wnzone, {hand="broad", tool="filbert 20", pile=wno, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(bowl, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(bankm2, {hand="broad", tool="filbert 20", pile=bankd4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end

--@ chunk 397
print(wait(4 * 24 * 60))
print(drying(500, 300), drying(500, 900), drying(500, 1100), drying(300, 1250))

--@ chunk 398
print(wait(6 * 24 * 60))
print(drying(500, 300), drying(500, 900), drying(500, 1100), drying(300, 1250))

--@ chunk 399
for i = 1, 4 do
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodf, {hand="body", pile=woodfar2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(reflnew, {hand="body", pile=reflp6, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl, {hand="body", pile=reflp6, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", pile=woodd2, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 400
bar = rect(330, 570, 340, 50):blur(8)
for i = 1, 3 do
  work(bar, {hand="broad", tool="filbert 20", pile=mio, coverage=2.8, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(bedge, {hand="body", pile=banklit, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(logp4, {hand="body", pile=barkpale, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(logd4, {hand="body", pile=barkd3, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
work(stubs, {hand="body", pile=barkpale, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
work(lscars, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
end
work(fighl, {hand="detail", pile=barkpale, coverage=2.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 401
barkmid = pile{{"lead white", 2.6}, {"pale smalt", 2.0}, {"raw umber", 1.8}, medium=0.03}
banklit2 = pile{{"raw umber", 3.0}, {"lead white", 1.0}, {"pale smalt", 0.8}, {"green earth", 0.6}, medium=0.03}
bar2 = rect(310, 545, 390, 70):blur(26)
for i = 1, 3 do
  work(bar2, {hand="broad", tool="filbert 22", pile=swo, coverage=2.8, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(bedge, {hand="body", pile=banklit2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(logd4, {hand="body", pile=barkd3, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(logp4, {hand="body", pile=barkmid, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(stubs, {hand="body", pile=barkmid, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(lscars, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 402
for i = 1, 2 do
  work(bar2, {hand="broad", tool="filbert 22", pile=swo, coverage=2.8, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
blend(bar2:grow(26), {angle=0.3})

--@ chunk 403
smid = pile{{"lead white", 4.8}, {"yellow ochre", 0.7}, {"red earth", 0.15}, {"bone black", 0.2}, medium=0.03}
function softband(a1, a2, b1, b2)
  return mask(function(x, y)
    return smoothstep(a1, a2, y + 26 * nz(x, 200)) * (1 - smoothstep(b1, b2, y + 26 * nz(x, 700)))
  end)
end
hz1 = softband(430, 530, 520, 610)
hz2 = softband(515, 600, 610, 700)
hz3 = softband(595, 680, 820, 900)
for i = 1, 3 do
  work(hz1, {hand="broad", tool="filbert 22", pile=swo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hz2, {hand="broad", tool="filbert 22", pile=smid, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hz3, {hand="broad", tool="filbert 22", pile=mio, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 404
for i = 1, 3 do
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodf, {hand="body", pile=woodfar2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", pile=woodd2, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
woodmist = pile{{"raw umber", 2.4}, {"bone black", 1.2}, {"pale smalt", 1.6}, {"lead white", 1.4}, medium=0.03}
wt1 = mask(function(x, y) return smoothstep(792, 700, y + 14 * nz(x, 500)) end)
wt2 = mask(function(x, y) return smoothstep(716, 640, y + 14 * nz(x, 500)) end)
for i = 1, 2 do
  work(woodflat * wt1, {hand="body", pile=woodmist, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodflat * wt2, {hand="body", pile=woodmist, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 405
bury = (mask(function(x, y) return smoothstep(890, 912, y) * (1 - smoothstep(1080, 1110, y)) end) - woodflat)
for i = 1, 3 do
  work(bury, {hand="body", pile=wmo, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
function reflone2(x, y0, y1, w, sd)
  local m = ribbon({{x, y0}, {x + 2, (y0 + y1) / 2}, {x - 2, y1}}, {w * 0.72, w * 0.88, w * 0.42})
  local n = noise{seed=sd, period=140, octaves=2}
  local g1 = y0 + (y1 - y0) * (0.30 + 0.035 * (sd % 6))
  local g2 = y0 + (y1 - y0) * (0.62 + 0.035 * (sd % 5))
  local function gap(gy)
    return mask(function(xx, yy)
      local z = yy + 8 * n(xx, 30)
      return smoothstep(gy - 5, gy + 5, z) * (1 - smoothstep(gy + 8, gy + 20, z))
    end)
  end
  return (m - gap(g1) - gap(g2)):blur(4)
end
reflnew2 = above(function(x) return -900 end)
for i, t in ipairs(WT) do reflnew2 = reflnew2 + reflone2(t[1] + 1, 906, 906 + (906 - t[3]) * 0.46, t[4] * 0.85, i) end
for i, t in ipairs(FT) do reflnew2 = reflnew2 + reflone2(t[1], 882, 882 + (882 - t[3]) * 0.3, t[4] * 0.8, 30 + i) end
reflnew2 = reflnew2:blur(2)
reflp7 = pile{{"raw umber", 2.2}, {"bone black", 1.7}, {"pale smalt", 1.6}, {"lead white", 0.5}, medium=0.05}
print(string.format("reflnew2 %.0f", reflnew2:area()))
for i = 1, 2 do
  work(reflnew2, {hand="body", pile=reflp7, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 406
lb = mask(function(x, y) return smoothstep(1000, 1090, y + 16 * nz(x, 300)) * (1 - smoothstep(1190, 1262, y + 16 * nz(x, 200))) end)
mid2 = pile{{"lead white", 3.6}, {"pale smalt", 1.4}, {"yellow ochre", 0.4}, {"bone black", 1.2}, medium=0.03}
sh = mask(function(x, y) return smoothstep(770, 850, y + 16 * nz(x, 400)) * (1 - smoothstep(900, 1000, y + 16 * nz(x, 600))) end)
for i = 1, 3 do
  work(lb, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(sh, {hand="broad", tool="filbert 22", pile=mid2, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 2 do
  work(nrefl, {hand="body", pile=reflp7, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 407
subject = nearbody3 + woodflat + woodf + brm + nb + reflnew2 + nrefl + logp4 + logd4 + stubs + fig
print(string.format("subject %.0f", subject:area()))
print(wait(7 * 24 * 60))
for _, p in ipairs({{500,300},{500,900},{500,1100},{500,1250}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 408
print(wait(7 * 24 * 60))
for _, p in ipairs({{500,300},{500,900},{500,1100},{500,1250},{60,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 409
print(wait(8 * 24 * 60))
for _, p in ipairs({{500,900},{500,1100},{500,1250},{700,900}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 410
for i = 1, 4 do
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodf, {hand="body", pile=woodfar2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(reflnew2, {hand="body", pile=reflp7, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl, {hand="body", pile=reflp7, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 411
for i = 1, 4 do
  work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
  work(bedge, {hand="body", pile=banklit2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(logd4, {hand="body", pile=barkd3, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(logp4, {hand="body", pile=barkmid, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(stubs, {hand="body", pile=barkmid, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(lscars, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
work(fighl, {hand="detail", pile=barkmid, coverage=2.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
print(wait(8 * 24 * 60))

--@ chunk 412
for i = 1, 2 do
  work(woodflat * wt1, {hand="body", pile=woodmist, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodflat * wt2, {hand="body", pile=woodmist, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(pale2, {hand="body", pile=barkmid, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 2 do
  work(nb, {hand="detail", pile=barkd3, coverage=2.6, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", pile=barkd3, coverage=2.2, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
work(fighl, {hand="detail", pile=barkmid, coverage=2.2, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 413
cor5 = ribbon({{10,100},{200,300},{400,500},{610,700},{820,890}}, {120,120,120,120,120})
cor5 = cor5 + ribbon({{-25,70},{165,270},{365,470},{575,670},{785,860}}, {80,80,80,80,80})
print(string.format("cor5 %.0f", cor5:area()))
for i = 1, 3 do
  work(cor5, {hand="hatch", tool="round 2.6", pile=bankd4, coverage=4.0, load=1.0, dips={5, 1.0, 0.0}, angle=0.85, clip=true, edge="found"})
end
skym = allup - subject
for i = 1, 3 do
  work(hband(skym, -30, 152), {hand="broad", tool="filbert 18", pile=sdo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(skym, 108, 354), {hand="broad", tool="filbert 18", pile=smo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(skym, 298, 498), {hand="broad", tool="filbert 18", pile=slo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(skym, 450, 626), {hand="broad", tool="filbert 18", pile=swo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(skym, 598, 814), {hand="broad", tool="filbert 22", pile=mio, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 414
blob2 = (ellipse(752, 862, 130, 85):blur(14)) - subject
for i = 1, 3 do
  work(blob2, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 2 do
  work(woodflat * wt1, {hand="body", pile=woodmist, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodflat * wt2, {hand="body", pile=woodmist, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 415
for i = 1, 2 do
  work(pale2, {hand="detail", tool="filbert 4", pile=barkmid, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
  work(nb, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.4, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
work(brm, {hand="detail", tool="filbert 3", pile=woodd2, coverage=2.6, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 416
skym = allup - subject
for i = 1, 4 do
  work(hband(skym, -30, 152), {hand="broad", tool="filbert 18", pile=sdo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(skym, 108, 354), {hand="broad", tool="filbert 18", pile=smo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(skym, 298, 498), {hand="broad", tool="filbert 18", pile=slo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(skym, 450, 626), {hand="broad", tool="filbert 18", pile=swo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(skym, 598, 814), {hand="broad", tool="filbert 22", pile=mio, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 417
pale2 = above(function(x) return -900 end)
for _, s in ipairs(PA) do pale2 = pale2 + ribbon({{s[3], s[1]}, {s[4], s[2]}}, {s[5] * 0.7, s[5]}) end
for _, s in ipairs(PC) do pale2 = pale2 + ribbon({{s[3], s[1]}, {s[4], s[2]}}, {s[5] * 0.7, s[5]}) end
pale2 = pale2:blur(1.2)
for i = 1, 3 do
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 1 do
  work(woodflat * wt1, {hand="body", pile=woodmist, coverage=2.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 418
for i = 1, 2 do
  work(pale2, {hand="detail", tool="filbert 4", pile=barkmid, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
midmist2 = pile{{"lead white", 4.2}, {"pale smalt", 1.0}, {"yellow ochre", 0.35}, {"bone black", 0.5}, medium=0.03}
blobs = (rect(400, 570, 220, 70):blur(16)) + (ellipse(752, 862, 150, 100):blur(20))
blobs = blobs - subject
for i = 1, 3 do
  work(blobs, {hand="broad", tool="filbert 22", pile=midmist2, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 419
field = (rect(-10, -30, 1020, 1010) - subject)
mf = hband(field, 540, 820)
for i = 1, 3 do
  work(mf, {hand="broad", tool="filbert 22", pile=mio, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(sh - subject, {hand="broad", tool="filbert 22", pile=mid2, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wa - subject, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb - subject, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 420
print(wait(7 * 24 * 60))
print(drying(500, 700), drying(500, 900), drying(500, 1200))
for i = 1, 4 do
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodf, {hand="body", pile=woodfar2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 421
print(wait(8 * 24 * 60))
print(drying(500, 700), drying(500, 900), drying(500, 1200), drying(500, 400))

--@ chunk 422
for i = 1, 5 do
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", tool="filbert 3", pile=woodd2, coverage=2.6, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
veil = woodflat * mask(function(x, y) return smoothstep(716, 664, y + 12 * nz(x, 500)) end)
work(veil, {hand="body", pile=woodmist, coverage=1.3, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 423
function cloudband(y0, y1, sd, amp)
  local n = noise{seed=sd, period=210, octaves=3, persistence=0.6}
  return mask(function(x, y)
    return smoothstep(y0 - 55, y0 + 55, y + amp * n(x, 80)) * (1 - smoothstep(y1 - 55, y1 + 55, y + amp * n(x, 300)))
  end)
end
cw1 = pile{{"lead white", 2.8}, {"raw umber", 1.6}, {"pale smalt", 0.6}, medium=0.03}
cw2 = pile{{"lead white", 4.2}, {"yellow ochre", 0.8}, {"red earth", 0.25}, medium=0.03}
cw3 = pile{{"lead white", 3.0}, {"pale smalt", 1.4}, {"bone black", 0.5}, medium=0.03}
skyf = rect(-10, -30, 1020, 700) - subject
CB = {{60, 210, 31, 26}, {170, 320, 32, 30}, {270, 410, 33, 28}, {350, 480, 34, 26}, {430, 560, 35, 24}}
CP = {cw1, cw2, cw3, cw2, cw3}
for i = 1, 2 do
  for k, c in ipairs(CB) do
    work(cloudband(c[1], c[2], c[3], c[4]) * skyf, {hand="broad", tool="filbert 22", pile=CP[k], coverage=1.8, load=1.0, dips={2, 1.0, 0.15}, angle=0.02, clip=true, edge="found"})
  end
end

--@ chunk 424
print(wait(5 * 24 * 60))
print(drying(500, 200), drying(500, 400), drying(700, 900))
print(wait(4 * 24 * 60))
print(drying(500, 200), drying(500, 400), drying(700, 900))

--@ chunk 425
blend(skyf, {angle=0.06})

--@ chunk 426
print(wait(7 * 24 * 60))
print(drying(500, 300), drying(700, 900), drying(700, 1050), drying(500, 1200))

--@ chunk 427
print(wait(4 * 24 * 60))
print(drying(700, 900), drying(700, 1000), drying(400, 950))
waterz = (rect(-10, 700, 1020, 500) - subject)
rpale = pile{{"lead white", 3.4}, {"pale smalt", 2.4}, medium=0.30}
rdark = pile{{"raw umber", 2.2}, {"bone black", 1.6}, {"pale smalt", 1.0}, medium=0.30}
for i = 1, 2 do
  work(waterz, {hand="glaze", pile=rpale, coverage=0.85, angle=0.0, length={80, 240}, load=1.0, dips={2, 1.0, 0.15}, clip=true, edge="found"})
  work(waterz, {hand="glaze", pile=rdark, coverage=0.75, angle=0.0, length={110, 300}, load=1.0, dips={2, 1.0, 0.15}, clip=true, edge="found"})
end

--@ chunk 428
print(wait(6 * 24 * 60))
print(drying(500, 900), drying(700, 1000))
print(wait(4 * 24 * 60))
print(drying(500, 900), drying(700, 1000), drying(500, 1150))

--@ chunk 429
waterf3 = (rect(-10, 700, 1020, 500) - subject)
for i = 1, 3 do
  work(wa - subject, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wb - subject, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(wnzone - subject, {hand="broad", tool="filbert 20", pile=wno, coverage=2.4, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 430
function wfix(m, p)
  work(m, {hand="hatch", tool="round 2.6", pile=p, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, angle=0.4, clip=true, edge="found"})
  for k = 1, 3 do
    work(m, {hand="broad", tool="filbert 20", pile=p, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  end
end
wfix(wa - subject, wfo)
wfix(wb - subject, wmo)
wfix(wnzone - subject, wno)

--@ chunk 431
zA = mask(function(x, y) return smoothstep(640, 750, y + 18 * nz(x, 700)) * (1 - smoothstep(890, 1010, y + 22 * nz(x, 300))) end)
zB = mask(function(x, y) return smoothstep(870, 990, y + 22 * nz(x, 300)) * (1 - smoothstep(1070, 1210, y + 16 * nz(x, 200))) end)
zC = mask(function(x, y) return smoothstep(1050, 1190, y + 18 * nz(x, 250)) * (1 - smoothstep(1225, 1330, y + 16 * nz(x, 200))) end)
for i = 1, 3 do
  work(zA - subject, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zB - subject, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC - subject, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 3 do
  work(reflnew2, {hand="body", pile=reflp7, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl, {hand="body", pile=reflp7, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 432
print("subject at trunk", subject:at(300, 800), subject:at(380, 800), subject:at(700, 800), subject:at(100, 500))
print("woodflat", woodflat:at(300, 800), woodflat:at(380, 800))
print("zA", zA:at(300, 800))

--@ chunk 433
for i = 1, 4 do
  work(woodflat, {hand="body", pile=woodd2, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", tool="filbert 3", pile=woodd2, coverage=2.6, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
veil2 = woodflat * mask(function(x, y) return smoothstep(712, 660, y + 12 * nz(x, 500)) end)
work(veil2, {hand="body", pile=woodmist, coverage=1.2, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
reflp8 = pile{{"raw umber", 2.6}, {"bone black", 2.4}, {"pale smalt", 1.0}, {"lead white", 0.3}, medium=0.04}
reflsoft = (reflnew2:blur(4)):blur(4)
for i = 1, 3 do
  work(reflsoft, {hand="body", pile=reflp8, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl, {hand="body", pile=reflp8, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 434
for i = 1, 2 do
  work(zB - subject, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC - subject, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
function reflone3(x, y0, y1, w, sd)
  local m = ribbon({{x, y0}, {x + 2.5, (y0 + y1) / 2}, {x - 2, y1}}, {w * 0.82, w * 0.95, w * 0.66})
  local n = noise{seed=sd, period=150, octaves=2}
  local gy = y0 + (y1 - y0) * (0.40 + 0.03 * (sd % 5))
  local g = mask(function(xx, yy)
    local z = yy + 10 * n(xx, 30)
    return smoothstep(gy - 6, gy + 6, z) * (1 - smoothstep(gy + 9, gy + 24, z))
  end)
  return (m - g):blur(6)
end
reflnew3 = above(function(x) return -900 end)
for i, t in ipairs(WT) do reflnew3 = reflnew3 + reflone3(t[1] + 1, 906, 906 + (906 - t[3]) * 0.82, t[4] * 0.8, i) end
for i, t in ipairs(FT) do reflnew3 = reflnew3 + reflone3(t[1], 882, 882 + (882 - t[3]) * 0.45, t[4] * 0.7, 30 + i) end
nrefl3 = reflone3(96, 957, 957 + 195, 40, 7) + reflone3(131, 957, 957 + 120, 12, 11) + reflone3(214, 957, 957 + 215, 56, 13)
print(string.format("reflnew3 %.0f  nrefl3 %.0f", reflnew3:area(), nrefl3:area()))
for i = 1, 3 do
  work(reflnew3, {hand="body", pile=reflp8, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl3, {hand="body", pile=reflp8, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 435
for i = 1, 2 do
  work(zB - subject, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC - subject, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
function reflone4(x, y0, y1, w, sd)
  local m = ribbon({{x, y0}, {x + 3, (y0 + y1) / 2}, {x - 2, y1}}, {w * 0.9, w * 1.05, w * 0.8})
  local n = noise{seed=sd, period=170, octaves=2}
  local function thin(frac, h)
    local gy = y0 + (y1 - y0) * frac
    return mask(function(xx, yy)
      local z = yy + 11 * n(xx, 40)
      return smoothstep(gy - 3, gy + 3, z) * (1 - smoothstep(gy + h - 3, gy + h + 3, z))
    end)
  end
  return (m - thin(0.34 + 0.02 * (sd % 4), 7) - thin(0.70 + 0.02 * (sd % 3), 6)):blur(8)
end
reflnew4 = above(function(x) return -900 end)
for i, t in ipairs(WT) do reflnew4 = reflnew4 + reflone4(t[1] + 1, 906, 906 + (906 - t[3]) * 0.85, t[4] * 0.9, i) end
for i, t in ipairs(FT) do reflnew4 = reflnew4 + reflone4(t[1], 882, 882 + (882 - t[3]) * 0.45, t[4] * 0.75, 30 + i) end
nrefl4 = reflone4(96, 957, 957 + 200, 46, 7) + reflone4(131, 957, 957 + 125, 13, 11) + reflone4(214, 957, 957 + 220, 60, 13)
for i = 1, 3 do
  work(reflnew4, {hand="body", pile=reflp8, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl4, {hand="body", pile=reflp8, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 436
oldlog = (logp4 + logd4 + stubs + lscars)
for i = 1, 3 do
  work(oldlog - bankm2, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(oldlog * bankm2, {hand="broad", tool="filbert 20", pile=bankd4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end
logbody = ribbon({{132,1252},{224,1282},{320,1292},{410,1272},{498,1252},{586,1274},{650,1306}}, {22,26,24,20,17,14,10})
logund  = ribbon({{138,1264},{228,1294},{322,1304},{412,1284},{500,1264},{588,1286},{650,1316}}, {7,9,8,7,6,5,4})
logtop  = ribbon({{130,1246},{222,1276},{318,1286},{408,1266},{496,1246},{584,1268},{648,1300}}, {5,6,6,5,4,4,3})
work(logund, {hand="body", pile=barkd3, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(logbody, {hand="body", pile=barkmid, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(logtop, {hand="detail", tool="filbert 6", pile=barkpale, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 437
left = (oldlog + logbody + logtop + logund) - bankm2
leftb = (oldlog + logbody + logtop + logund) * bankm2
for i = 1, 3 do
  work(left, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(leftb, {hand="broad", tool="filbert 20", pile=bankd4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end
lb2 = ribbon({{146,1226},{230,1262},{316,1267},{402,1232},{486,1235},{566,1265},{634,1287}}, {20,24,22,19,16,13,9})
lu2 = ribbon({{148,1238},{232,1274},{318,1279},{404,1244},{488,1247},{568,1277},{634,1297}}, {7,9,8,7,6,5,4})
lt2 = ribbon({{144,1216},{228,1252},{314,1257},{400,1222},{484,1225},{564,1255},{632,1277}}, {4,5,5,4,4,3,3})
logmid = pile{{"lead white", 2.0}, {"pale smalt", 1.4}, {"raw umber", 2.4}, medium=0.03}
loglit = pile{{"lead white", 3.4}, {"pale smalt", 1.6}, {"raw umber", 0.8}, medium=0.03}
work(lu2, {hand="body", pile=barkd3, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(lb2, {hand="body", pile=logmid, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(lt2, {hand="detail", tool="filbert 6", pile=loglit, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 438
left = (oldlog + logbody + logtop + logund + lb2 + lu2 + lt2) - bankm2
for i = 1, 3 do
  work(left, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
lb3 = ribbon({{150,1238},{236,1272},{322,1278},{408,1248},{492,1250},{572,1280},{642,1304}}, {20,24,22,19,16,13,9})
lu3 = ribbon({{152,1250},{238,1284},{324,1290},{410,1260},{494,1262},{574,1292},{642,1314}}, {7,9,8,7,6,5,4})
lt3 = ribbon({{148,1228},{234,1262},{320,1268},{406,1238},{490,1240},{570,1270},{640,1294}}, {4,5,5,4,4,3,3})
work(lu3, {hand="body", pile=barkd3, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(lb3, {hand="body", pile=logmid, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(lt3, {hand="detail", tool="filbert 6", pile=loglit, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 2 do
  work(bankm2, {hand="broad", tool="filbert 20", pile=bankd4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end
bedge2 = bankm2 - bankm2:shrink(58)
banklit3 = pile{{"lead white", 2.2}, {"raw umber", 2.2}, {"pale smalt", 0.8}, {"green earth", 1.0}, medium=0.03}
for i = 1, 2 do
  work(bedge2, {hand="body", pile=banklit3, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 439
for i = 1, 3 do
  work(fig, {hand="detail", pile=figd, coverage=3.0, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
end
fig2 = ellipse(702, 1157, 6.5, 8) + ribbon({{701,1163},{703,1198}}, {16, 21})
fig2 = fig2 + poly({{689,1193},{715,1193},{723,1264},{681,1264}}, true)
for i = 1, 4 do
  work(fig2, {hand="detail", pile=figd, coverage=2.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
fighl2 = ribbon({{709,1156},{712,1196},{719,1258}}, {2.4, 3.0, 3.4})
work(fighl2, {hand="detail", tool="filbert 4", pile=loglit, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
TUFT = {{318,1284}, {392,1256}, {468,1250}, {548,1288}, {772,1290}, {846,1272}, {916,1284}, {262,1298}}
tufts = above(function(x) return -900 end)
for i, t in ipairs(TUFT) do
  tufts = tufts + ribbon({{t[1], t[2] + 14}, {t[1] - 5, t[2] - 8}}, {3.0, 1.4})
  tufts = tufts + ribbon({{t[1], t[2] + 14}, {t[1] + 5, t[2] - 10}}, {3.0, 1.4})
  tufts = tufts + ribbon({{t[1], t[2] + 14}, {t[1] + 1, t[2] - 13}}, {2.6, 1.2})
end
work(tufts, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 440
fig3 = ellipse(702, 1156, 6, 7.5)
fig3 = fig3 + ribbon({{701,1164},{703,1188}}, {15, 20})
fig3 = fig3 + poly({{688,1184},{714,1184},{722,1262},{682,1262}})
fig3 = fig3:blur(0.8)
for i = 1, 4 do
  work(fig3, {hand="detail", pile=figd, coverage=2.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
figlit = pile{{"lead white", 2.4}, {"raw umber", 1.4}, {"pale smalt", 0.8}, medium=0.04}
fighl3 = ribbon({{710,1157},{713,1190},{719,1258}}, {1.8, 2.2, 2.4})
work(fighl3, {hand="detail", tool="filbert 3", pile=figlit, coverage=2.0, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
shadow = ellipse(703, 1265, 26, 5):blur(4)
work(shadow, {hand="body", pile=barkd3, coverage=1.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 441
for i = 1, 3 do
  work((fig:grow(10)), {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 4 do
  work(fig3, {hand="detail", pile=figd, coverage=2.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
figlit2 = pile{{"lead white", 2.0}, {"raw umber", 1.8}, {"pale smalt", 0.6}, medium=0.05}
fighl4 = ribbon({{711,1160},{714,1192},{719,1256}}, {1.4, 1.6, 1.8})
work(fighl4, {hand="detail", tool="filbert 3", pile=figlit2, coverage=1.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 442
for i = 1, 3 do
  work((fig:grow(34)), {hand="broad", tool="filbert 20", pile=wno, coverage=2.8, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 4 do
  work(fig3, {hand="detail", pile=figd, coverage=2.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
work(fighl4, {hand="detail", tool="filbert 3", pile=figlit2, coverage=1.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 443
halofix = pile{{"lead white", 1.4}, {"pale smalt", 1.2}, {"bone black", 5.0}, medium=0.02}
for i = 1, 3 do
  work((fig3:grow(30)), {hand="broad", tool="filbert 20", pile=halofix, coverage=2.8, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 444
tw1 = pile{{"lead white", 2.0}, {"pale smalt", 1.5}, {"bone black", 4.0}, medium=0.02}
tw2 = pile{{"lead white", 1.7}, {"pale smalt", 1.35}, {"bone black", 4.5}, medium=0.02}
tw3 = pile{{"lead white", 1.5}, {"pale smalt", 1.25}, {"bone black", 4.9}, medium=0.02}
for i = 1, 3 do
  work(rect(380, 1120, 90, 90), {hand="broad", tool="filbert 20", pile=tw1, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(rect(490, 1120, 90, 90), {hand="broad", tool="filbert 20", pile=tw2, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(rect(600, 1120, 90, 90), {hand="broad", tool="filbert 20", pile=tw3, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 445
fixz = ((fig3:grow(70)) + rect(350, 1090, 380, 150)):blur(20)
for i = 1, 4 do
  work(fixz, {hand="broad", tool="filbert 20", pile=tw1, coverage=2.8, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 4 do
  work(fig3, {hand="detail", pile=figd, coverage=2.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
work(fighl4, {hand="detail", tool="filbert 3", pile=figlit2, coverage=1.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
work(shadow, {hand="body", pile=barkd3, coverage=1.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 446
subj2 = subject + reflnew4 + nrefl4 + fig3 + lb3 + lu3 + lt3 + tufts
for i = 1, 3 do
  work(zA - subj2, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zB - subj2, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC - subj2, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 447
print(wait(8 * 24 * 60))
print(drying(500, 900), drying(700, 1100), drying(500, 1250), drying(500, 400))

--@ chunk 448
print(wait(8 * 24 * 60))
print(drying(500, 900), drying(700, 1100), drying(500, 1250))

--@ chunk 449
print(wait(10 * 24 * 60))
print(drying(500, 900), drying(700, 1100), drying(500, 1250), drying(300, 900))

--@ chunk 450
ripzp = (rect(-10, 700, 1020, 500) - subject - reflnew4 - nrefl4)
ripz1 = pile{{"lead white", 3.0}, {"pale smalt", 2.2}, medium=0.30}
ripz2 = pile{{"lead white", 1.8}, {"pale smalt", 1.4}, {"bone black", 1.8}, medium=0.30}
stipple(ripzp, {pile=ripz1, width=7, coverage=7, drag={16, 0}, pressure={0.3, 0.6}, cluster=0.5, seed=23})
stipple(ripzp, {pile=ripz2, width=7, coverage=6, drag={20, 0}, pressure={0.3, 0.6}, cluster=0.5, seed=29})

--@ chunk 451
for i = 1, 3 do
  work(zA - subj2, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zB - subj2, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC - subj2, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 3 do
  work(reflnew4, {hand="body", pile=reflp8, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl4, {hand="body", pile=reflp8, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 452
RIP = {{300,1092,520,1098},{560,1122,780,1128},{820,1082,980,1088},{262,1172,440,1180},
      {500,1212,700,1218},{760,1162,960,1168},{300,982,470,988},{540,942,720,948},{800,902,970,908},
      {262,852,400,856},{620,862,800,868},{340,1232,520,1240},{600,1252,780,1258},{180,1040,300,1045}}
RIP2 = {{420,1140,600,1146},{700,1040,880,1045},{300,1130,430,1135},{840,1130,980,1136},
       {380,960,540,965},{660,1200,820,1206},{240,1100,360,1105},{480,1270,640,1276}}
ripdark = pile{{"lead white", 1.6}, {"pale smalt", 1.3}, {"bone black", 2.2}, medium=0.35}
riplite = pile{{"lead white", 3.0}, {"pale smalt", 2.4}, medium=0.35}
rb1 = brush("round", 2.6)
for i = 1, 2 do
  for _, s in ipairs(RIP) do
    rb1:load(ripdark, 0.9)
    rb1:stroke({{s[1], s[2]}, {(s[1] + s[2] * 0) + s[3], s[4]}}, {pressure={0.42, 0.30}})
  end
end
for _, s in ipairs(RIP2) do
  rb1:load(riplite, 0.8)
  rb1:stroke({{s[1], s[2]}, {s[3], s[4]}}, {pressure={0.36, 0.26}})
end

--@ chunk 453
subj3 = subj2 - reflnew4 - nrefl4
for i = 1, 2 do
  work(zB - subj3, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC - subj3, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
function reflone5(x, y0, y1, w, sd)
  local n = noise{seed=sd, period=190, octaves=2}
  local bow = 3 * (n(x, 10))
  local m = ribbon({{x, y0}, {x + bow, (y0 + y1) / 2}, {x - bow * 0.6, y1}}, {w * 0.78, w * 0.86, w * 0.5})
  return m:blur(9)
end
reflnew5 = above(function(x) return -900 end)
for i, t in ipairs(WT) do reflnew5 = reflnew5 + reflone5(t[1] + 1, 906, 906 + (906 - t[3]) * 0.8, t[4] * 0.8, i) end
for i, t in ipairs(FT) do reflnew5 = reflnew5 + reflone5(t[1], 882, 882 + (882 - t[3]) * 0.45, t[4] * 0.7, 30 + i) end
nrefl5 = reflone5(96, 957, 957 + 195, 42, 7) + reflone5(131, 957, 957 + 120, 12, 11) + reflone5(214, 957, 957 + 210, 56, 13)
print(string.format("reflnew5 %.0f", reflnew5:area()))
for i = 1, 2 do
  work(reflnew5, {hand="body", pile=reflp7, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl5, {hand="body", pile=reflp7, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 454
for i = 1, 3 do
  work((fig3:grow(16)), {hand="broad", tool="filbert 20", pile=wno, coverage=2.8, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
fig4 = ellipse(701, 1153, 5.5, 7)
fig4 = fig4 + ribbon({{701,1160},{702,1181}}, {13, 19})
fig4 = fig4 + poly({{690,1179},{713,1179},{721,1261},{683,1261}})
fig4 = fig4:blur(0.7)
for i = 1, 4 do
  work(fig4, {hand="detail", pile=figd, coverage=2.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
fh = brush("round", 3.4)
fh:load(figlit2, 0.8)
fh:stroke({{708,1157},{711,1190},{716,1258}}, {pressure={0.4, 0.3}})

--@ chunk 455
print(wait(10 * 24 * 60))
print(drying(500, 400), drying(500, 900), drying(700, 1250), drying(700, 300))

--@ chunk 456
print(wait(8 * 24 * 60))
banksl = pile{{"raw umber", 4.2}, {"bone black", 3.0}, {"pale smalt", 0.6}, {"green earth", 0.4}, medium=0.02}
slope = bankm2 * mask(function(x, y) return smoothstep(1290, 1340, y + 14 * nz(x, 300)) end)
for i = 1, 2 do
  work(slope, {hand="body", pile=banksl, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
STN = {{196,1300,15,7},{388,1318,11,6},{556,1306,17,8},{806,1322,13,6},{936,1310,10,5},{128,1340,12,6},{300,1360,9,5}}
stones = above(function(x) return -900 end)
for _, s in ipairs(STN) do stones = stones + ellipse(s[1], s[2], s[3], s[4]) end
work(stones, {hand="detail", pile=logmid, coverage=2.4, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
for _, s in ipairs(STN) do
  work(ellipse(s[1], s[2] + s[4] * 0.7, s[3] * 1.05, s[4] * 0.5), {hand="detail", pile=barkd3, coverage=2.0, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 457
reflnew5b = reflnew5 * mask(function(x, y) return y > 914 and 1 or 0 end)
nrefl5b = nrefl5 * mask(function(x, y) return y > 964 and 1 or 0 end)
subj4 = subj2 - reflnew5 - nrefl5
for i = 1, 3 do
  work(zA - subj4, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zB - subj4, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC - subj4, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
for i = 1, 2 do
  work(reflnew5b, {hand="body", pile=reflp7, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl5b, {hand="body", pile=reflp7, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 458
subj5 = subj2 - reflnew5 - nrefl5
for i = 1, 3 do
  work(zA - subj5, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zB - subj5, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC - subj5, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end
reflnew5c = reflnew5 * mask(function(x, y) return y > 908 and 1 or 0 end)
nrefl5c = nrefl5 * mask(function(x, y) return y > 960 and 1 or 0 end)
for i = 1, 3 do
  work(reflnew5c, {hand="body", pile=reflp7, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl5c, {hand="body", pile=reflp7, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 459
print(wait(10 * 24 * 60))
print(drying(500, 400), drying(500, 900), drying(700, 1250), drying(200, 300))

--@ chunk 460
print(wait(10 * 24 * 60))
print(drying(500, 900), drying(700, 1250), drying(300, 1000), drying(700, 700))

--@ chunk 461
print(wait(10 * 24 * 60))
print(drying(500, 900), drying(700, 1250), drying(300, 1000), drying(700, 1300))

--@ chunk 462
whole = everywhere()
bindc = pile{{"lead white", 3.0}, {"pale smalt", 2.0}, {"raw umber", 1.0}, medium=0.42}
stipple(whole, {pile=bindc, width=3, coverage=3, pressure={0.18, 0.32}, cluster=0.4, seed=41, clip=true})

--@ chunk 463
print(wait(7 * 24 * 60))
print(drying(500, 400), drying(500, 900), drying(700, 1250))
for i = 1, 3 do
  work(hband(allup, -30, 152), {hand="broad", tool="filbert 18", pile=sdo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 108, 354), {hand="broad", tool="filbert 18", pile=smo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 298, 498), {hand="broad", tool="filbert 18", pile=slo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 450, 626), {hand="broad", tool="filbert 18", pile=swo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 598, 814), {hand="broad", tool="filbert 22", pile=mio, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 464
for i = 1, 3 do
  work(zA, {hand="broad", tool="filbert 20", pile=wfo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zB, {hand="broad", tool="filbert 20", pile=wmo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(zC, {hand="broad", tool="filbert 20", pile=wno, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(bankm2, {hand="broad", tool="filbert 20", pile=bankd4, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.06, clip=true, edge="found"})
end

--@ chunk 465
print(wait(9 * 24 * 60))
print(drying(500, 300), drying(500, 700), drying(700, 1000), drying(700, 1300))
for i = 1, 2 do
  for k, c in ipairs(CB) do
    work(cloudband(c[1], c[2], c[3], c[4]) * allup, {hand="broad", tool="filbert 22", pile=CP[k], coverage=1.8, load=1.0, dips={2, 1.0, 0.15}, angle=0.02, clip=true, edge="found"})
  end
end

--@ chunk 466
for i = 1, 4 do
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodf, {hand="body", pile=woodfar2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(brm, {hand="detail", tool="filbert 3", pile=woodd2, coverage=2.6, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
work(nb, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.4, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 2 do
  work(woodflat * mask(function(x, y) return smoothstep(722, 664, y + 12 * nz(x, 500)) end), {hand="body", pile=woodmist, coverage=1.4, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})
  work(pale2, {hand="detail", tool="filbert 4", pile=barkmid, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(reflnew5c, {hand="body", pile=reflp7, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl5c, {hand="body", pile=reflp7, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 467
for i = 1, 2 do
  work(slope, {hand="body", pile=banksl, coverage=2.2, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(bedge2, {hand="body", pile=banklit3, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(lu3, {hand="body", pile=barkd3, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
for i = 1, 3 do
  work(lb3, {hand="body", pile=logmid, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(lt3, {hand="detail", tool="filbert 6", pile=loglit, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
work(tufts, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
work(stones, {hand="detail", pile=logmid, coverage=2.4, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
for _, s in ipairs(STN) do
  work(ellipse(s[1], s[2] + s[4] * 0.7, s[3] * 1.05, s[4] * 0.5), {hand="detail", pile=barkd3, coverage=2.0, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 468
for i = 1, 4 do
  work(fig4, {hand="detail", pile=figd, coverage=2.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
fh:load(figlit2, 0.8)
fh:stroke({{708,1157},{711,1190},{716,1258}}, {pressure={0.4, 0.3}})
work(shadow, {hand="body", pile=barkd3, coverage=1.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
rb1:load(ripdark, 0.9)
for _, s in ipairs(RIP) do
  rb1:stroke({{s[1], s[2]}, {s[3], s[4]}}, {pressure={0.42, 0.30}})
end
rb1:load(riplite, 0.8)
for _, s in ipairs(RIP2) do
  rb1:stroke({{s[1], s[2]}, {s[3], s[4]}}, {pressure={0.36, 0.26}})
end

--@ chunk 469
banklit4 = pile{{"raw umber", 3.4}, {"bone black", 2.2}, {"pale smalt", 0.5}, {"green earth", 0.8}, medium=0.03}
logmid2 = pile{{"lead white", 1.6}, {"raw umber", 2.6}, {"pale smalt", 1.0}, medium=0.03}
loglit2 = pile{{"lead white", 2.6}, {"raw umber", 1.4}, {"pale smalt", 0.8}, medium=0.03}
for i = 1, 3 do
  work(bedge2, {hand="body", pile=banklit4, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 3 do
  work(lb3, {hand="body", pile=logmid2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(lt3, {hand="detail", tool="filbert 6", pile=loglit2, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
fig5 = ellipse(701, 1152, 5.5, 7)
fig5 = fig5 + ribbon({{701,1159},{702,1178}}, {12, 18})
fig5 = fig5 + poly({{692,1176},{712,1176},{716,1262},{688,1262}})
fig5 = fig5:blur(0.7)
for i = 1, 4 do
  work(fig5, {hand="detail", pile=figd, coverage=2.8, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
fh:load(figlit2, 0.8)
fh:stroke({{708,1156},{711,1186},{715,1258}}, {pressure={0.4, 0.3}})

--@ chunk 470
bankface = bankm2 * mask(function(x, y) return smoothstep(1288, 1330, y + 14 * nz(x, 300)) end)
banklit5 = pile{{"raw umber", 3.0}, {"bone black", 1.8}, {"pale smalt", 0.6}, {"green earth", 0.9}, medium=0.03}
bedge3 = bankm2 - bankm2:shrink(26)
for i = 1, 3 do
  work(bankface, {hand="body", pile=banksl, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, angle=0.15, clip=true, edge="found"})
  work(bedge3, {hand="body", pile=banklit5, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
figd2 = pile{{"raw umber", 2.4}, {"bone black", 1.8}, {"pale smalt", 0.8}, medium=0.04}
for i = 1, 3 do
  work(fig5, {hand="detail", pile=figd2, coverage=2.6, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 471
for i = 1, 3 do
  work(reflnew5c, {hand="body", pile=reflp8, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(nrefl5c, {hand="body", pile=reflp8, coverage=2.4, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
banklit6 = pile{{"raw umber", 3.4}, {"bone black", 2.6}, {"pale smalt", 0.4}, {"green earth", 0.5}, medium=0.03}
for i = 1, 3 do
  work(bedge3, {hand="body", pile=banklit6, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 472
for i = 1, 3 do
  work(bedge2, {hand="body", pile=banklit6, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(bedge3, {hand="body", pile=banklit6, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end

--@ chunk 473
print(wait(8 * 24 * 60))
print(drying(500, 300), drying(500, 450), drying(700, 600))

--@ chunk 474
skyonly = rect(250, -30, 760, 640)
blend(skyonly, {angle=0.05})

--@ chunk 475
for i = 1, 3 do
  work(hband(allup, -30, 152), {hand="broad", tool="filbert 18", pile=sdo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 108, 354), {hand="broad", tool="filbert 18", pile=smo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 298, 498), {hand="broad", tool="filbert 18", pile=slo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 450, 626), {hand="broad", tool="filbert 18", pile=swo, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
  work(hband(allup, 598, 814), {hand="broad", tool="filbert 22", pile=mio, coverage=2.6, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 476
for i = 1, 2 do
  for k, c in ipairs(CB) do
    work(cloudband(c[1], c[2], c[3], c[4]) * allup, {hand="broad", tool="filbert 22", pile=CP[k], coverage=1.8, load=1.0, dips={2, 1.0, 0.15}, angle=0.02, clip=true, edge="found"})
  end
end

--@ chunk 477
for i = 1, 4 do
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(woodflat, {hand="body", pile=woodd2, coverage=2.8, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
for i = 1, 2 do
  work(woodf, {hand="body", pile=woodfar2, coverage=2.6, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
  work(brm, {hand="detail", tool="filbert 3", pile=woodd2, coverage=2.6, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
  work(nb, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.4, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
  work(pale2, {hand="detail", tool="filbert 4", pile=barkmid, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
end
work(scars, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
work(woodflat * mask(function(x, y) return smoothstep(722, 664, y + 12 * nz(x, 500)) end), {hand="body", pile=woodmist, coverage=1.4, load=1.0, dips={5, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 478
print(wait(10 * 24 * 60))
print(drying(500, 300), drying(300, 700), drying(500, 900), drying(700, 1250))

--@ chunk 479
print(wait(8 * 24 * 60))
print(drying(300, 700), drying(500, 900), drying(700, 1250), drying(60, 400))
for i = 1, 4 do
  work(nearbody3, {hand="body", pile=barkd3, coverage=3.0, load=1.0, dips={4, 1.0, 0.0}, clip=true, edge="found"})
end
work(nb, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.6, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})
work(scars, {hand="detail", tool="filbert 3", pile=barkd3, coverage=2.2, load=1.0, dips={6, 1.0, 0.0}, clip=true, edge="found"})

--@ chunk 480
vap = mask(function(x, y) return smoothstep(520, 600, y + 20 * nz(x, 300)) * (1 - smoothstep(780, 850, y + 20 * nz(x, 700))) end) - subject
mistglz = pile{{"lead white", 3.0}, {"pale smalt", 1.6}, {"raw umber", 1.0}, medium=0.40}
for i = 1, 2 do
  work(vap, {hand="glaze", pile=mistglz, coverage=0.7, clip=true})
end
misttop = (mask(function(x, y) return smoothstep(545, 625, y + 20 * nz(x, 300)) * (1 - smoothstep(665, 745, y + 20 * nz(x, 700))) end) - subject)
for i = 1, 2 do
  work(misttop, {hand="broad", tool="filbert 22", pile=smid, coverage=2.0, load=1.0, dips={3, 1.0, 0.0}, angle=0.0, clip=true, edge="found"})
end

--@ chunk 481
local pts = {{100,100},{500,300},{500,700},{500,900},{500,1100},{700,1250},{140,400}}
for i,p in ipairs(pts) do
  print(string.format("%4d,%4d -> %s", p[1], p[2], drying(p[1], p[2])))
end
print("W="..W.." H="..H)

--@ chunk 482
print(wait(3*24*60))

-- the dark for deepening the evening sky: opaque earths carry it, smalt and cobalt cool it
skyd = pile{{"raw umber",2.0},{"bone black",2.0},{"pale smalt",1.2},{"lead white",0.9},
           {"cobalt blue",0.12}, medium=0.12}
print(skyd)

-- a glaze laid in graded bands, densest at the zenith
local z1 = mask(function(x,y) return 1 - smoothstep(40, 230, y) end)
work(z1, {hand="glaze", pile=skyd, coverage=2.2, edge="found", length={120,300}, seed=11})
print("band one laid")

--@ chunk 483
-- a thin, transparent dark: it sinks the blue-grey gaps and lets the cream bands burn through
skyT = pile{{"raw umber",1.7},{"bone black",1.7},{"pale smalt",1.1},{"cobalt blue",0.10}, medium=0.34}
print(skyT)

local z2 = mask(function(x,y) return (1 - smoothstep(150, 430, y)) * smoothstep(60, 190, y) + (1 - smoothstep(150, 430, y)) * 0.85 end)
local z3 = mask(function(x,y) return (1 - smoothstep(320, 620, y)) * 0.6 end)
work(z2, {hand="glaze", pile=skyT, coverage=1.8, edge="found", length={90,210}, dips={5,0.9,0.35}, seed=12})
work(z3, {hand="glaze", pile=skyT, coverage=1.5, edge="found", length={90,210}, dips={5,0.9,0.35}, seed=13})
print("bands two and three laid")

--@ chunk 484
-- a real evening sky, laid as opaque graded bands rather than a dark glaze over the old one
skyA = pile{{"lead white",1.6},{"raw umber",2.0},{"bone black",1.3},{"pale smalt",1.3},
           {"cobalt blue",0.10}, medium=0.06}
print(skyA)

local zA = mask(function(x,y) return 1 - smoothstep(90, 300, y) end)
work(zA, {hand="broad", tool="filbert 22", pile=skyA, coverage=1.6, edge="found",
          length={130,240}, dips={4,0.95,0.3}, clip=true, seed=21})
print("zenith band laid")

--@ chunk 485
-- a test strip in the top of the sky: four candidate mixtures, so I can read their real value
local P = {
  pile{{"lead white",3.0},{"raw umber",1.6},{"pale smalt",1.0},{"bone black",0.20}, medium=0.05},
  pile{{"lead white",2.6},{"raw umber",2.0},{"pale smalt",1.2},{"bone black",0.45}, medium=0.05},
  pile{{"lead white",2.0},{"raw umber",2.2},{"pale smalt",1.3},{"bone black",0.90}, medium=0.05},
  pile{{"lead white",1.5},{"raw umber",2.2},{"pale smalt",1.3},{"bone black",1.60}, medium=0.05},
}
for i=1,4 do
  local m = rect((i-1)*250, 0, 250, 96)
  work(m, {hand="body", pile=P[i], coverage=3, edge="found", clip=true, fill=true, seed=30+i})
  print(i, P[i])
end

--@ chunk 486
-- one even opaque sky field over everything above the mist, in three separate passes
skyF = pile{{"lead white",2.6},{"raw umber",2.0},{"pale smalt",1.2},{"bone black",0.45}, medium=0.05}
print(skyF)

skyall = rect(0, -10, 1000, 690)
for pass=1,3 do
  work(skyall, {hand="broad", tool="filbert 22", pile=skyF, coverage=1.6, edge="found",
                length={130,240}, dips={4,0.95,0.3}, clip=true, seed=40+pass})
  print("pass "..pass)
end

--@ chunk 487
for pass=4,5 do
  work(skyall, {hand="broad", tool="filbert 22", pile=skyF, coverage=1.6, edge="found",
                length={130,240}, dips={4,0.95,0.3}, clip=true, seed=40+pass})
end
print("field made opaque")

-- zenith: a truly transparent glaze, thin, so it sinks the sky instead of tearing it
zen = pile{{"raw umber",1.2},{"bone black",1.0},{"pale smalt",1.4},{"cobalt blue",0.08}, medium=0.45}
local zt = mask(function(x,y) return 1 - smoothstep(40, 300, y) end)
work(zt, {hand="glaze", pile=zen, coverage=1.0, edge="found", length={110,200}, dips={6,0.9,0.35}, seed=51})
print("zenith glaze")

--@ chunk 488
-- a cool veil over the sky: evening blue, not pink
cool = pile{{"lead white",1.0},{"pale smalt",1.7},{"cobalt blue",0.10}, medium=0.45}
work(skyall, {hand="glaze", pile=cool, coverage=1.0, edge="found", length={110,220},
              dips={6,0.9,0.35}, seed=61})
print("sky cooled")

-- the vapour rising off the far water, with an irregular upper boundary
nh = noise{seed=7, octaves=3, period=340}
mistp = pile{{"lead white",3.2},{"pale smalt",0.8},{"yellow ochre",0.35},{"bone black",0.15}, medium=0.05}
local rise = mask(function(x,y) return smoothstep(455 + 70*nh(x, y), 700, y) end)
for p=1,2 do
  work(rise, {hand="broad", tool="filbert 22", pile=mistp, coverage=1.5, edge="found",
              length={140,260}, dips={4,0.95,0.3}, clip=true, seed=70+p})
end
print("vapour risen")

--@ chunk 489
-- WATER. Decided now and not to be changed: far water 64, mid 56, near 46.
wf = pile{{"lead white",2.2},{"raw umber",1.6},{"pale smalt",1.3},{"bone black",0.30}, medium=0.05}
print(wf)
nw = noise{seed=13, octaves=3, period=260}
waterz = mask(function(x,y) return 1 - smoothstep(748 + 30*nw(x, y), 905, y) end)
for p=1,3 do
  work(waterz, {hand="broad", tool="filbert 22", pile=wf, coverage=1.6, edge="found",
                length={140,260}, dips={4,0.95,0.3}, clip=true, seed=80+p})
end
print("water laid")

--@ chunk 490
waterz = mask(function(x,y) return smoothstep(752 + 26*nw(x, y), 838, y) end)
print(string.format("water mask:  y=700 %.2f  y=800 %.2f  y=900 %.2f  y=1100 %.2f  y=1300 %.2f",
  waterz:at(500,700), waterz:at(500,800), waterz:at(500,900), waterz:at(500,1100), waterz:at(500,1300)))
for p=1,3 do
  work(waterz, {hand="broad", tool="filbert 22", pile=wf, coverage=1.6, edge="found",
                length={140,260}, dips={4,0.95,0.3}, clip=true, seed=80+p})
end
print("water laid")

--@ chunk 491
wmid  = pile{{"lead white",2.0},{"raw umber",2.2},{"pale smalt",1.3},{"bone black",0.75}, medium=0.05}
wnear = pile{{"lead white",1.5},{"raw umber",2.4},{"pale smalt",1.3},{"bone black",1.35}, medium=0.05}
print(wmid); print(wnear)

nearz = smoothstep and mask(function(x,y) return smoothstep(955, 1135, y) end) or nil
print(string.format("near mask: 900 %.2f  1050 %.2f  1200 %.2f",
  nearz:at(500,900), nearz:at(500,1050), nearz:at(500,1200)))

for p=1,3 do
  work(waterz, {hand="broad", tool="filbert 22", pile=wmid, coverage=1.6, edge="found",
                length={140,260}, dips={4,0.95,0.3}, clip=true, seed=90+p})
end
for p=1,3 do
  work(nearz, {hand="broad", tool="filbert 22", pile=wnear, coverage=1.6, edge="found",
               length={140,260}, dips={4,0.95,0.3}, clip=true, seed=95+p})
end
print("water deepened")

--@ chunk 492
local Q = {
  pile{{"lead white",2.0},{"raw umber",2.2},{"pale smalt",1.3},{"bone black",0.75}, medium=0.05},
  pile{{"lead white",1.2},{"raw umber",2.8},{"pale smalt",1.3},{"bone black",1.5}, medium=0.05},
  pile{{"lead white",0.8},{"raw umber",3.0},{"pale smalt",1.2},{"bone black",2.2}, medium=0.05},
  pile{{"lead white",0.5},{"raw umber",2.6},{"pale smalt",1.0},{"bone black",3.0}, medium=0.05},
  pile{{"lead white",0.3},{"raw umber",2.0},{"pale smalt",0.8},{"bone black",4.0}, medium=0.05},
  pile{{"lead white",1.5},{"raw umber",2.6},{"pale smalt",1.4},{"bone black",1.8}, medium=0.05},
}
local lane = rect(0, 1195, 1000, 150)
for i=1,6 do
  local lo, hi = (i-1)*1000/6, i*1000/6
  local m = mask(function(x,y) if x >= lo and x < hi then return 1.0 else return 0.0 end end) * lane
  for p=1,2 do
    work(m, {hand="body", pile=Q[i], coverage=1.8, edge="found", clip=true, seed=110+i*3+p})
  end
  print(i, Q[i])
end

--@ chunk 493
wnear2 = pile{{"lead white",1.4},{"raw umber",2.8},{"bone black",1.5},{"pale smalt",1.2}, medium=0.05}
print(wnear2)
for p=1,3 do
  work(nearz, {hand="broad", tool="filbert 22", pile=wnear2, coverage=1.6, edge="found",
               length={140,260}, dips={4,0.95,0.3}, clip=true, seed=120+p})
end
print("near water")

--@ chunk 494
-- cool slate water against the warm vapour: the contrast the picture is built on
wcool   = pile{{"lead white",1.7},{"pale smalt",2.0},{"raw umber",2.4},{"bone black",0.80}, medium=0.05}
wcoold  = pile{{"lead white",1.2},{"pale smalt",1.7},{"raw umber",2.8},{"bone black",1.60}, medium=0.05}
print(wcool); print(wcoold)
for p=1,3 do
  work(waterz, {hand="broad", tool="filbert 22", pile=wcool, coverage=1.6, edge="found",
                length={140,260}, dips={4,0.95,0.3}, clip=true, seed=130+p})
end
for p=1,3 do
  work(nearz, {hand="broad", tool="filbert 22", pile=wcoold, coverage=1.6, edge="found",
               length={140,260}, dips={4,0.95,0.3}, clip=true, seed=140+p})
end
print("water, cool")

--@ chunk 495
print(wait(24*60))

-- white and smalt carry the coolness, black gives depth, a little green earth greys it
wsw = pile{{"lead white",3.0},{"pale smalt",2.4},{"bone black",0.70},{"green earth",0.35}, medium=0.05}
print(wsw)
for p=1,4 do
  work(waterz, {hand="broad", tool="filbert 22", pile=wsw, coverage=1.6, edge="found",
                length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true, seed=150+p})
end
print("cool water laid")

--@ chunk 496
-- a cool evening sky, laid solid: pressed paint and fill, so nothing of the ground shows through
skyc = pile{{"lead white",2.2},{"pale smalt",2.2},{"bone black",0.75},{"green earth",0.20},
           {"cobalt blue",0.08}, medium=0.05}
skyz = pile{{"lead white",1.6},{"pale smalt",2.0},{"bone black",1.6},{"green earth",0.20},
           {"cobalt blue",0.10}, medium=0.05}
print(skyc); print(skyz)

skym = mask(function(x,y) return 1 - smoothstep(590, 745, y) end)
for p=1,3 do
  work(skym, {hand="broad", tool="filbert 22", pile=skyc, coverage=1.8, edge="found",
              length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true,
              pressure={0.7,1.0}, seed=160+p})
end
-- the zenith, darker, carried down in a soft gradation
local zt2 = mask(function(x,y) return 1 - smoothstep(60, 380, y) end)
for p=1,2 do
  work(zt2, {hand="broad", tool="filbert 22", pile=skyz, coverage=1.7, edge="found",
             length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true,
             pressure={0.65,0.95}, seed=170+p})
end
print("sky laid solid")

--@ chunk 497
wat  = pile{{"lead white",2.0},{"pale smalt",2.4},{"bone black",1.6},{"green earth",0.30}, medium=0.05}
watd = pile{{"lead white",1.3},{"pale smalt",2.2},{"bone black",2.6},{"green earth",0.30}, medium=0.05}
vapp = pile{{"lead white",3.4},{"yellow ochre",0.50},{"pale smalt",0.50},{"bone black",0.08}, medium=0.05}
print(wat); print(watd); print(vapp)

na = noise{seed=23, octaves=3, period=300}
nb = noise{seed=29, octaves=3, period=220}
watm  = mask(function(x,y) return smoothstep(775 + 22*na(x, y), 880, y) end)
watdm = mask(function(x,y) return smoothstep(1000, 1160, y) end)
vapm  = mask(function(x,y) return smoothstep(520 + 45*na(x,y), 700, y) * (1 - smoothstep(772 + 30*nb(x,y), 872, y)) end)
print(string.format("vapour at 400 %.2f  650 %.2f  740 %.2f  830 %.2f  950 %.2f",
  vapm:at(400,400), vapm:at(400,650), vapm:at(400,740), vapm:at(400,830), vapm:at(400,950)))

for p=1,3 do
  work(watm, {hand="broad", tool="filbert 22", pile=wat, coverage=1.8, edge="found",
              length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true, pressure={0.7,1.0}, seed=180+p})
end
for p=1,3 do
  work(watdm, {hand="broad", tool="filbert 22", pile=watd, coverage=1.8, edge="found",
               length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true, pressure={0.7,1.0}, seed=190+p})
end
for p=1,3 do
  work(vapm, {hand="broad", tool="filbert 22", pile=vapp, coverage=1.8, edge="found",
              length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true, pressure={0.7,1.0}, seed=200+p})
end
print("water and vapour laid")

--@ chunk 498
-- a gradation with no threshold crossing: the mask stays above 0.3, so the paint thins instead of stopping
zt3 = mask(function(x,y) return 1 - 0.60*smoothstep(20, 820, y) end)
print(string.format("zt3 at 0 %.2f  200 %.2f  400 %.2f  600 %.2f  800 %.2f",
  zt3:at(500,0), zt3:at(500,200), zt3:at(500,400), zt3:at(500,600), zt3:at(500,800)))
for p=1,2 do
  work(zt3, {hand="broad", tool="filbert 22", pile=skyz, coverage=1.7, edge="found",
             length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true, pressure={0.65,0.95}, seed=210+p})
end
print("sky gradated")

--@ chunk 499
-- the vapour: dimmer, with long soft irregular edges so it reads as vapour lying on the water
vap2 = pile{{"lead white",2.9},{"yellow ochre",0.55},{"pale smalt",0.75},{"bone black",0.22}, medium=0.05}
nc = noise{seed=37, octaves=3, period=260}
nd = noise{seed=43, octaves=3, period=180}
vapm2 = mask(function(x,y) return smoothstep(430 + 60*nc(x,y), 690, y) * (1 - smoothstep(768 + 34*nd(x,y), 872, y)) end)
for p=1,2 do
  work(vapm2, {hand="broad", tool="filbert 22", pile=vap2, coverage=1.7, edge="found",
               length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true, pressure={0.7,1.0}, seed=220+p})
end

-- the deep water, graded without ever crossing the anchoring threshold
wdeep2 = mask(function(x,y) return 1 - 0.58*smoothstep(880, 1389, y) end)
for p=1,2 do
  work(wdeep2, {hand="broad", tool="filbert 22", pile=watd, coverage=1.7, edge="found",
                length={140,260}, dips={4,0.95,0.3}, clip=true, fill=true, pressure={0.7,1.0}, seed=230+p})
end
print("vapour calmed, deep water graded")

--@ chunk 500
print(wait(24*60))

skyzone = mask(function(x,y) return 1 - smoothstep(500, 650, y) end)
watzone = mask(function(x,y) return smoothstep(770 + 20*nc(x,y), 880, y) end)
vapzone = mask(function(x,y) return smoothstep(430 + 60*nc(x,y), 690, y) * (1 - smoothstep(768 + 34*nd(x,y), 880, y)) end)

for p=1,2 do
  work(skyzone, {hand="glaze", pile=skyc, coverage=2.6, edge="found", length={190,340},
                 dips={4,0.9,0.3}, clip=true, seed=240+p})
end
for p=1,2 do
  work(watzone, {hand="glaze", pile=wat, coverage=2.6, edge="found", length={190,340},
                 dips={4,0.9,0.3}, clip=true, seed=250+p})
end
for p=1,2 do
  work(vapzone, {hand="glaze", pile=vap2, coverage=2.8, edge="found", length={190,340},
                 dips={4,0.9,0.3}, clip=true, seed=260+p})
end
print("fields re-laid with the glaze hand")

--@ chunk 501
-- lean, flowing paint: at medium 0.05 nothing could level, and every field came out as lumps
skyc2 = pile{{"lead white",2.2},{"pale smalt",2.2},{"bone black",0.75},{"green earth",0.20},
            {"cobalt blue",0.08}, medium=0.30}
print(skyc2)
for p=1,2 do
  work(skyzone, {hand="broad", tool="filbert 30", pile=skyc2, coverage=2.2, edge="found",
                 length={200,360}, dips={4,0.9,0.25}, clip=true, fill=true,
                 pressure={0.55,0.85}, seed=270+p})
end
print("sky with oil in it")

--@ chunk 502
wat3 = pile{{"lead white",2.0},{"pale smalt",2.4},{"bone black",1.6},{"green earth",0.30}, medium=0.30}
watd3 = pile{{"lead white",1.3},{"pale smalt",2.2},{"bone black",2.6},{"green earth",0.30}, medium=0.30}
vap3 = pile{{"lead white",2.9},{"yellow ochre",0.55},{"pale smalt",0.75},{"bone black",0.22}, medium=0.30}
print(wat3); print(watd3); print(vap3)

for p=1,2 do
  work(watzone, {hand="broad", tool="filbert 30", pile=wat3, coverage=2.2, edge="found",
                 length={200,360}, dips={4,0.9,0.25}, clip=true, fill=true,
                 pressure={0.55,0.85}, seed=280+p})
end
-- the water deepening toward the near bank, as a ramp that never crosses 0.3
wdeep3 = mask(function(x,y) return 1 - 0.55*smoothstep(880, 1389, y) end)
for p=1,2 do
  work(wdeep3, {hand="broad", tool="filbert 30", pile=watd3, coverage=2.0, edge="found",
                length={200,360}, dips={4,0.9,0.25}, clip=true, fill=true,
                pressure={0.55,0.85}, seed=290+p})
end
for p=1,2 do
  work(vapzone, {hand="broad", tool="filbert 30", pile=vap3, coverage=2.4, edge="found",
                 length={200,360}, dips={4,0.9,0.25}, clip=true, fill=true,
                 pressure={0.55,0.85}, seed=300+p})
end
print("water and vapour with oil in them")

--@ chunk 503
for _,p in ipairs({{500,300},{500,1000},{500,1250},{500,700}}) do
  print(string.format("%d,%d %s", p[1], p[2], drying(p[1], p[2])))
end
local d = drying(500,1000)
if d == "open" or d == "setting" then
  blend(watzone, {angle=0.03})
  blend(vapzone, {angle=0.03})
  blend(skyzone, {angle=0.03})
  print("blended")
else
  print("too set to blend")
end

--@ chunk 504
print("skyzone", skyzone ~= nil, "vapzone", vapzone ~= nil, "watzone", watzone ~= nil)
print("skyc2", tostring(skyc2))

--@ chunk 505
-- one trunk, drawn as its own shape: wide at the base, tapering up, with a slight drift
function trunk(xb, yb, xt, yt, w)
  return body_of{spine = {{xb, yb}, {(xb+xt)/2 + (yb-yt)*0.04, (yb+yt)/2}, {xt, yt}},
                 widths = {w, w*0.78, w*0.52}, blend=0.7}
end

FAR = {
 {228,818,221,706,2.6},{291,826,297,716,3.0},{361,815,355,698,2.5},{440,829,447,720,3.3},
 {511,821,506,704,2.7},{569,833,576,722,3.1},{648,824,641,708,2.8},{726,817,732,714,3.4},
 {809,830,802,700,2.5},{900,822,907,718,3.0}}
MID = {
 {270,868,258,672,4.2},{343,880,352,686,3.8},{420,866,412,664,4.8},{506,884,517,692,4.4},
 {593,870,585,676,5.2},{690,878,700,688,4.0},{798,862,789,658,4.6}}
NEAR = {
 {318,916,302,628,7.2},{455,928,468,644,6.4},{600,910,590,618,8.2},{760,922,775,640,7.0}}

local function woodmask(rank)
  local m = trunk(rank[1][1],rank[1][2],rank[1][3],rank[1][4],rank[1][5]):mask()
  for i=2,#rank do
    m = m + trunk(rank[i][1],rank[i][2],rank[i][3],rank[i][4],rank[i][5]):mask()
  end
  return m
end
woodF, woodM, woodN = woodmask(FAR), woodmask(MID), woodmask(NEAR)
print(string.format("far %.0f  mid %.0f  near %.0f sq units", woodF:area(), woodM:area(), woodN:area()))
print(string.format("near trunk mask at 600,760 %.2f   at 600,890 %.2f   at 320,300 %.2f",
  woodN:at(600,760), woodN:at(600,890), woodN:at(320,300)))

--@ chunk 506
wfarP  = pile{{"lead white",1.7},{"pale smalt",1.8},{"raw umber",1.3},{"bone black",0.55}, medium=0.14}
wmidP  = pile{{"lead white",1.1},{"pale smalt",1.4},{"raw umber",2.0},{"bone black",1.3}, medium=0.14}
wnearP = pile{{"lead white",0.6},{"pale smalt",1.0},{"raw umber",2.4},{"bone black",2.8}, medium=0.14}
print(wfarP); print(wmidP); print(wnearP)

for p=1,2 do
  work(woodF, {hand="body", tool="filbert 3", pile=wfarP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=310+p})
end
for p=1,2 do
  work(woodM, {hand="body", tool="filbert 5", pile=wmidP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=320+p})
end
for p=1,2 do
  work(woodN, {hand="body", tool="filbert 8", pile=wnearP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=330+p})
end
print("wood painted")

--@ chunk 507
local function reflmask(rank, len, spread)
  local m = nil
  for i=1,#rank do
    local t = rank[i]
    local xb, yb, w = t[1], t[2], t[5]
    local d = (t[3]-xb) * 0.7
    local r = ribbon({{xb, yb}, {xb + d*0.4, yb + len*0.45}, {xb + d, yb + len}},
                      {w*0.95, w*1.05, w*spread})
    m = m and (m + r) or r
  end
  return m
end
reflF = reflmask(FAR, 130, 0.7)
reflM = reflmask(MID, 185, 0.6)
reflN = reflmask(NEAR, 245, 0.5)

reflFp = pile{{"lead white",1.6},{"pale smalt",1.7},{"raw umber",1.5},{"bone black",0.80}, medium=0.30}
reflMp = pile{{"lead white",0.9},{"pale smalt",1.2},{"raw umber",2.0},{"bone black",1.80}, medium=0.30}
reflNp = pile{{"lead white",0.4},{"pale smalt",0.7},{"raw umber",2.2},{"bone black",3.20}, medium=0.30}
print(reflFp); print(reflMp); print(reflNp)

for p=1,2 do
  work(reflF, {hand="glaze", tool="filbert 9", pile=reflFp, coverage=1.5, edge="found",
               angle=math.pi/2, length={50,110}, clip=true, seed=340+p})
end
for p=1,2 do
  work(reflM, {hand="glaze", tool="filbert 12", pile=reflMp, coverage=1.5, edge="found",
               angle=math.pi/2, length={50,110}, clip=true, seed=350+p})
end
for p=1,2 do
  work(reflN, {hand="glaze", tool="filbert 18", pile=reflNp, coverage=1.5, edge="found",
               angle=math.pi/2, length={50,110}, clip=true, seed=360+p})
end
print("reflections laid")

--@ chunk 508
print(wait(24*60))
ne = noise{seed=53, octaves=3, period=240}
killz = mask(function(x,y) return smoothstep(836 + 26*ne(x,y), 962, y) end)
print(string.format("killz at 850 %.2f  900 %.2f  1000 %.2f", killz:at(500,850), killz:at(500,900), killz:at(500,1000)))
for p=1,2 do
  work(killz, {hand="broad", tool="filbert 30", pile=wat3, coverage=2.0, edge="found",
               length={200,360}, dips={4,0.9,0.25}, clip=true, fill=true,
               pressure={0.55,0.85}, seed=370+p})
end
for p=1,2 do
  work(wdeep3, {hand="broad", tool="filbert 30", pile=watd3, coverage=1.8, edge="found",
                length={200,360}, dips={4,0.9,0.25}, clip=true, fill=true,
                pressure={0.55,0.85}, seed=380+p})
end
print("water clean again")

--@ chunk 509
-- water as a band, graded downward, multiplied inside the mask so it cannot flood
wband = mask(function(x,y) return smoothstep(815, 905, y) * (1 - 0.52*smoothstep(1000, 1389, y)) end)
print(string.format("wband  800 %.2f  870 %.2f  960 %.2f  1100 %.2f  1300 %.2f",
  wband:at(500,800), wband:at(500,870), wband:at(500,960), wband:at(500,1100), wband:at(500,1300)))
print(string.format("vapzone  600 %.2f  700 %.2f  780 %.2f  860 %.2f  950 %.2f",
  vapzone:at(500,600), vapzone:at(500,700), vapzone:at(500,780), vapzone:at(500,860), vapzone:at(500,950)))
for p=1,2 do
  work(wband, {hand="broad", tool="filbert 30", pile=wat3, coverage=2.0, edge="found",
               length={200,360}, dips={4,0.9,0.25}, clip=true, fill=true,
               pressure={0.55,0.85}, seed=390+p})
end
for p=1,2 do
  work(vapzone, {hand="broad", tool="filbert 30", pile=vap3, coverage=2.2, edge="found",
                 length={200,360}, dips={4,0.9,0.25}, clip=true, fill=true,
                 pressure={0.55,0.85}, seed=400+p})
end
print("water band and vapour back")

--@ chunk 510
print("water", drying(500,1000), "vapour", drying(500,730))
blend(wband, {angle=0.02})
blend(vapzone, {angle=0.02})
print("blended")

--@ chunk 511
-- the wood, three ranks and three values
for p=1,2 do
  work(woodF, {hand="body", tool="filbert 3", pile=wfarP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=410+p})
  work(woodM, {hand="body", tool="filbert 5", pile=wmidP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=420+p})
  work(woodN, {hand="body", tool="filbert 8", pile=wnearP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=430+p})
end

-- bare limbs on the nearer trunks, each drawn as its own shape
BR = {
 { {{420,722},{368,668},{318,644}}, {2.4,1.6,0.9} },
 { {{508,742},{562,690},{604,672}}, {2.4,1.6,0.9} },
 { {{588,730},{542,672},{508,654}}, {2.2,1.4,0.8} },
 { {{702,746},{762,702},{804,688}}, {2.4,1.6,0.9} },
 { {{304,692},{258,644},{228,626}}, {3.0,2.0,1.1} },
 { {{470,702},{522,658},{556,640}}, {3.0,2.0,1.1} },
 { {{778,702},{832,662},{866,646}}, {3.2,2.1,1.2} },
}
branches = nil
for i=1,#BR do
  local r = ribbon(BR[i][1], BR[i][2])
  branches = branches and (branches + r) or r
end
work(branches, {hand="body", tool="filbert 3", pile=wmidP, coverage=2.6, edge="found",
                length={10,34}, clip=true, seed=440})

-- the light is from the right: a lit sliver down the right of the nearer trunks
litM = woodM - woodM:offset(-2.6)
litN = woodN - woodN:offset(-3.6)
litP = pile{{"lead white",2.4},{"pale smalt",1.2},{"raw umber",0.7}, medium=0.16}
work(litM, {hand="body", tool="filbert 4", pile=litP, coverage=2.2, edge="found", length={12,40}, clip=true, seed=450})
work(litN, {hand="body", tool="filbert 6", pile=litP, coverage=2.2, edge="found", length={12,40}, clip=true, seed=451})
print("wood back, with limbs and a light side")

--@ chunk 512
-- bury the beading: lay the wood clean again
for p=1,2 do
  work(woodF, {hand="body", tool="filbert 3", pile=wfarP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=460+p})
  work(woodM, {hand="body", tool="filbert 5", pile=wmidP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=470+p})
  work(woodN, {hand="body", tool="filbert 8", pile=wnearP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=480+p})
  work(branches, {hand="body", tool="filbert 3", pile=wmidP, coverage=2.4, edge="found",
                  length={10,34}, clip=true, seed=490+p})
end

-- the light down one side of a trunk, drawn as a stroke, not a clipped band
lb = brush("filbert", 3)
for _, rank in ipairs({MID, NEAR}) do
  for i=1,#rank do
    local xb, yb, xt, yt, w = rank[i][1], rank[i][2], rank[i][3], rank[i][4], rank[i][5]
    local o = w*0.30
    lb:load(litP, 0.75)
    lb:stroke({{xt + o, yt + 8}, {(xb+xt)/2 + o, (yb+yt)/2}, {xb + o, yb - 8}},
              {pressure={0.55,0.5,0.55}, ramps={0.18,0.22}})
  end
end
print("wood clean, light stroked down the near trunks")

--@ chunk 513
-- more taper: a birch narrows strongly toward the top
function trunk(xb, yb, xt, yt, w)
  return body_of{spine = {{xb, yb}, {(xb+xt)/2 + (yb-yt)*0.05, (yb+yt)/2}, {xt, yt}},
                 widths = {w, w*0.70, w*0.34}, blend=0.7}
end
local function woodmask(rank)
  local m = trunk(rank[1][1],rank[1][2],rank[1][3],rank[1][4],rank[1][5]):mask()
  for i=2,#rank do
    m = m + trunk(rank[i][1],rank[i][2],rank[i][3],rank[i][4],rank[i][5]):mask()
  end
  return m
end
woodF, woodM, woodN = woodmask(FAR), woodmask(MID), woodmask(NEAR)
print(string.format("far %.0f  mid %.0f  near %.0f", woodF:area(), woodM:area(), woodN:area()))

for p=1,2 do
  work(woodF, {hand="body", tool="filbert 3", pile=wfarP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=500+p})
  work(woodM, {hand="body", tool="filbert 5", pile=wmidP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=510+p})
  work(woodN, {hand="body", tool="filbert 8", pile=wnearP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=520+p})
end
work(branches, {hand="body", tool="filbert 3", pile=wmidP, coverage=2.4, edge="found",
                length={10,34}, clip=true, seed=530})

-- the light: over-stroke each trunk to one side with a paler, oilier paint
lightP = pile{{"lead white",2.6},{"pale smalt",1.0},{"raw umber",0.5}, medium=0.25}
local function sidelight(rank, tool, press, off)
  local bb = brush(tool)
  for i=1,#rank do
    local xb, yb, xt, yt, w = rank[i][1], rank[i][2], rank[i][3], rank[i][4], rank[i][5]
    local o = w*off
    bb:load(lightP, 0.6)
    bb:stroke({{xt + o*1.6, yt + 14}, {(xb+xt)/2 + o, (yb+yt)/2}, {xb + o, yb - 10}},
              {pressure={press, press*0.9, press}, ramps={0.22,0.25}})
  end
end
sidelight(MID, "filbert 3", 0.42, 0.30)
sidelight(NEAR, "filbert 6", 0.45, 0.28)
print("trunks retapered, light stroked")

--@ chunk 514
-- the whole wood drawn back into the vapour: a transparent veil, strong at the tops
woodall = (woodF + woodM + woodN + branches)
veilm = mask(function(x,y) return 1 - 0.55*smoothstep(620, 880, y) end)
print(string.format("veil at 600 %.2f  700 %.2f  800 %.2f  900 %.2f",
  veilm:at(500,600), veilm:at(500,700), veilm:at(500,800), veilm:at(500,900)))
veilP = pile{{"lead white",3.0},{"pale smalt",0.50},{"yellow ochre",0.25}, medium=0.60}
print(veilP)
for p=1,2 do
  work(woodall * veilm, {hand="glaze", tool="filbert 20", pile=veilP, coverage=2.4, edge="found",
                         length={70,160}, clip=true, seed=540+p})
end
print("wood veiled")

--@ chunk 515
for p=1,2 do
  work(woodF, {hand="body", tool="filbert 3", pile=wfarP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=550+p})
  work(woodM, {hand="body", tool="filbert 5", pile=wmidP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=560+p})
  work(woodN, {hand="body", tool="filbert 8", pile=wnearP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=570+p})
  work(branches, {hand="body", tool="filbert 3", pile=wmidP, coverage=2.4, edge="found",
                  length={10,34}, clip=true, seed=580+p})
end

function sidelight(rank, tool, press, off)
  local bb = brush(tool)
  for i=1,#rank do
    local r = rank[i]
    local xb, yb, xt, yt, w = r[1], r[2], r[3], r[4], r[5]
    local o = w*off
    bb:load(lightP, 0.6)
    bb:stroke({{xt + o*1.6, yt + 14}, {(xb+xt)/2 + o, (yb+yt)/2}, {xb + o, yb - 10}},
              {pressure={press, press*0.9, press}, ramps={0.22,0.25}})
  end
end
sidelight(MID, "filbert 3", 0.42, 0.30)
sidelight(NEAR, "filbert 6", 0.45, 0.28)

veilP2 = pile{{"lead white",3.0},{"pale smalt",0.50},{"yellow ochre",0.25}, medium=0.75}
work(woodall * veilm, {hand="glaze", tool="filbert 20", pile=veilP2, coverage=0.7, edge="found",
                       length={70,160}, clip=true, seed=590})
print("wood re-laid and whispered")

--@ chunk 516
print(wait(5*24*60))
for _,p in ipairs({{200,250},{600,250},{200,1000},{700,700}}) do
  print(string.format("%d,%d %s", p[1], p[2], drying(p[1], p[2])))
end

--@ chunk 517
-- solid covering passes: stiff paint, pressed, fill closed. No glaze, no blend, no oil.
skyS = pile{{"lead white",2.2},{"pale smalt",2.2},{"bone black",0.75},{"green earth",0.20},
           {"cobalt blue",0.08}, medium=0.04}
vapS = pile{{"lead white",2.9},{"yellow ochre",0.55},{"pale smalt",0.75},{"bone black",0.22}, medium=0.04}
watS = pile{{"lead white",2.0},{"pale smalt",2.4},{"bone black",1.6},{"green earth",0.30}, medium=0.04}
watdS= pile{{"lead white",1.3},{"pale smalt",2.2},{"bone black",2.6},{"green earth",0.30}, medium=0.04}
print(skyS); print(vapS); print(watS); print(watdS)

for p=1,3 do
  work(skyzone, {hand="broad", tool="filbert 24", pile=skyS, coverage=2.0, edge="found",
                 length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
                 pressure={0.75,1.0}, seed=600+p})
end
for p=1,3 do
  work(vapzone, {hand="broad", tool="filbert 24", pile=vapS, coverage=2.0, edge="found",
                 length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
                 pressure={0.75,1.0}, seed=610+p})
end
for p=1,3 do
  work(wband, {hand="broad", tool="filbert 24", pile=watS, coverage=2.0, edge="found",
               length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
               pressure={0.75,1.0}, seed=620+p})
end
deeps = mask(function(x,y) return smoothstep(950, 1160, y) end)
print(string.format("deeps 900 %.2f  1050 %.2f  1250 %.2f", deeps:at(500,900), deeps:at(500,1050), deeps:at(500,1250)))
for p=1,3 do
  work(deeps, {hand="broad", tool="filbert 24", pile=watdS, coverage=2.0, edge="found",
               length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
               pressure={0.75,1.0}, seed=630+p})
end
print("fields solid")

--@ chunk 518
-- new geometry: the vapour ends at the far shore's foot, and the wood stands in the water in front of it
vapzone2 = mask(function(x,y) return smoothstep(430 + 60*nc(x,y), 665, y) * (1 - smoothstep(742 + 30*nd(x,y), 818, y)) end)
watshall = mask(function(x,y) return smoothstep(798, 866, y) * (1 - 0.62*smoothstep(950, 1389, y)) end)
watdeep  = mask(function(x,y) return smoothstep(798, 866, y) * (0.35 + 0.65*smoothstep(950, 1389, y)) end)
print(string.format("vap  500 %.2f  640 %.2f  760 %.2f  810 %.2f  900 %.2f",
  vapzone2:at(400,500), vapzone2:at(400,640), vapzone2:at(400,760), vapzone2:at(400,810), vapzone2:at(400,900)))
print(string.format("watshall 790 %.2f  900 %.2f  1100 %.2f  1380 %.2f",
  watshall:at(400,790), watshall:at(400,900), watshall:at(400,1100), watshall:at(400,1380)))
print(string.format("watdeep  790 %.2f  900 %.2f  1100 %.2f  1380 %.2f",
  watdeep:at(400,790), watdeep:at(400,900), watdeep:at(400,1100), watdeep:at(400,1380)))

for p=1,3 do
  work(vapzone2, {hand="broad", tool="filbert 24", pile=vapS, coverage=2.0, edge="found",
                  length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
                  pressure={0.75,1.0}, seed=640+p})
end
for p=1,3 do
  work(watshall, {hand="broad", tool="filbert 24", pile=watS, coverage=2.0, edge="found",
                  length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
                  pressure={0.75,1.0}, seed=650+p})
end
for p=1,3 do
  work(watdeep, {hand="broad", tool="filbert 24", pile=watdS, coverage=2.0, edge="found",
                 length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
                 pressure={0.75,1.0}, seed=660+p})
end
print("vapour and water re-set")

--@ chunk 519
nbank = noise{seed=61, octaves=4, period=430}
function banky(x) return 1200 + 46*nbank(x, 1240) end
bankm = mask(function(x,y) return smoothstep(banky(x) - 55, banky(x) + 75, y) end)
print(string.format("bank  1100 %.2f  1200 %.2f  1300 %.2f  1380 %.2f",
  bankm:at(150,1100), bankm:at(150,1200), bankm:at(150,1300), bankm:at(150,1380)))

bankD = pile{{"lead white",0.7},{"raw umber",3.2},{"bone black",2.6},{"green earth",0.40}, medium=0.05}
print(bankD)
for p=1,3 do
  work(bankm, {hand="broad", tool="filbert 24", pile=bankD, coverage=2.0, edge="found",
               length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
               pressure={0.75,1.0}, seed=670+p})
end

-- the two near birches, cut by the left edge
birchA = body_of{spine = {{18, 1400}, {40, 700}, {72, -30}}, widths = {80, 66, 52}, blend=0.7}:mask()
birchB = body_of{spine = {{172, 1400}, {186, 700}, {198, -30}}, widths = {54, 46, 36}, blend=0.7}:mask()
birches = birchA + birchB
print(string.format("birchA at 60,400 %.2f  at 140,400 %.2f  birchB at 185,400 %.2f",
  birchA:at(60,400), birchA:at(140,400), birchB:at(185,400)))
birchD = pile{{"lead white",0.5},{"raw umber",3.0},{"bone black",3.0},{"pale smalt",0.30}, medium=0.05}
print(birchD)
for p=1,3 do
  work(birches, {hand="body", tool="filbert 20", pile=birchD, coverage=2.4, edge="found",
                 length={40,110}, clip=true, fill=true, pressure={0.8,1.0}, seed=680+p})
end
print("bank and near birches laid")

--@ chunk 520
for p=1,2 do
  work(birches, {hand="broad", tool="filbert 26", pile=birchD, coverage=2.2, edge="found",
                 length={220,420}, angle=math.pi/2, dips={5,0.9,0.25}, clip=true,
                 pressure={0.7,0.95}, seed=690+p})
end

litB = pile{{"lead white",2.0},{"raw umber",1.6},{"pale smalt",0.5}, medium=0.10}
print(litB)
lbB = brush("filbert", 14)
local SP = {
  {pts = {{30,1400},{52,700},{84,-30}}, off = 30},
  {pts = {{184,1400},{196,700},{208,-30}}, off = 15},
}
for i=1,2 do
  local q = SP[i].pts
  local o = SP[i].off
  lbB:load(litB, 0.55)
  lbB:stroke({{q[1][1]+o, q[1][2]}, {q[2][1]+o*0.8, q[2][2]}, {q[3][1]+o*0.6, q[3][2]}},
            {pressure={0.5, 0.45, 0.5}, ramps={0.25, 0.3}})
end
print("birches swept and lit")

--@ chunk 521
for p=1,2 do
  work(woodF, {hand="body", tool="filbert 3", pile=wfarP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=700+p})
end
for p=1,2 do
  work(woodM, {hand="body", tool="filbert 5", pile=wmidP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=710+p})
end
for p=1,2 do
  work(woodN, {hand="body", tool="filbert 8", pile=wnearP, coverage=3, edge="found",
               length={12,45}, clip=true, seed=720+p})
end
for p=1,2 do
  work(branches, {hand="body", tool="filbert 3", pile=wmidP, coverage=2.4, edge="found",
                  length={10,34}, clip=true, seed=730+p})
end
sidelight(MID, "filbert 3", 0.42, 0.30)
sidelight(NEAR, "filbert 6", 0.45, 0.28)
print("wood in the water")

--@ chunk 522
function reflmask(rank, len, spread)
  local m = nil
  for i=1,#rank do
    local t = rank[i]
    local xb, yb, w = t[1], t[2], t[5]
    local d = (t[3]-xb) * 0.7
    local r = ribbon({{xb, yb + 4}, {xb + d*0.4, yb + len*0.45}, {xb + d, yb + len}},
                      {w*0.9, w*1.0, w*spread}):blur(2.5)
    m = m and (m + r) or r
  end
  return m
end
rF = reflmask(FAR, 58, 0.8)
rM = reflmask(MID, 84, 0.7)
rN = reflmask(NEAR, 120, 0.6)

rFp = pile{{"lead white",1.5},{"pale smalt",1.9},{"bone black",1.3},{"raw umber",0.6}, medium=0.06}
rMp = pile{{"lead white",1.0},{"pale smalt",1.5},{"bone black",2.2},{"raw umber",0.9}, medium=0.06}
rNp = pile{{"lead white",0.5},{"pale smalt",1.1},{"bone black",3.2},{"raw umber",1.2}, medium=0.06}
print(rFp); print(rMp); print(rNp)

stipple(rF, {pile=rFp, width=3.2, coverage=2.6, drag={9, math.pi/2}, feather=0.35, clip=rF, seed=740})
stipple(rM, {pile=rMp, width=4.0, coverage=3.0, drag={11, math.pi/2}, feather=0.35, clip=rM, seed=741})
stipple(rN, {pile=rNp, width=5.5, coverage=3.2, drag={14, math.pi/2}, feather=0.35, clip=rN, seed=742})
print("reflections broken into the water")

--@ chunk 523
stipple(rF, {pile=rFp, width=3.6, coverage=4.0, drag={9, math.pi/2}, feather=0.3, clip=rF, seed=750})
stipple(rM, {pile=rMp, width=4.4, coverage=4.4, drag={11, math.pi/2}, feather=0.3, clip=rM, seed=751})
stipple(rN, {pile=rNp, width=6.0, coverage=5.0, drag={14, math.pi/2}, feather=0.3, clip=rN, seed=752})
print("reflections strengthened")

-- ripples: long horizontal sweeps across the whole flood
watrip = mask(function(x,y) return smoothstep(838, 902, y) end)
print(string.format("watrip 830 %.2f  900 %.2f  1100 %.2f  1380 %.2f",
  watrip:at(400,830), watrip:at(400,900), watrip:at(400,1100), watrip:at(400,1380)))
ripP = pile{{"lead white",2.6},{"pale smalt",2.0},{"bone black",0.80},{"green earth",0.20}, medium=0.06}
ripD = pile{{"lead white",1.2},{"pale smalt",1.8},{"bone black",2.20},{"green earth",0.20}, medium=0.06}
print(ripP); print(ripD)
for p=1,2 do
  work(watrip, {hand="body", tool="filbert 7", pile=ripP, angle=0.0, length={45,165},
                coverage=0.9, clip=true, fill=false, edge="found", broken=0.6, seed=760+p})
end
for p=1,2 do
  work(watrip, {hand="body", tool="filbert 5", pile=ripD, angle=0.0, length={35,130},
                coverage=0.7, clip=true, fill=false, edge="found", broken=0.5, seed=770+p})
end
print("ripples drawn")

--@ chunk 524
-- the bank swept clean: horizontal passes, no fill dabs
for p=1,2 do
  work(bankm, {hand="broad", tool="filbert 26", pile=bankD, coverage=2.2, angle=0.0,
               length={150,300}, dips={5,0.9,0.25}, clip=true, fill=false,
               pressure={0.75,1.0}, seed=780+p})
end

-- a fallen birch lying along the bank
logm = ribbon({{232,1272},{378,1258},{516,1246},{656,1234},{772,1228}}, {14,15,13,10,6})
logP = pile{{"lead white",2.4},{"raw umber",1.4},{"pale smalt",0.9}, medium=0.06}
print(logP)
for p=1,2 do
  work(logm, {hand="body", tool="filbert 10", pile=logP, coverage=2.4, angle=0.0,
              length={40,110}, clip=true, edge="found", seed=790+p})
end

-- the figure, seen from behind, at the water's edge
fx = 600
fy = banky(fx) + 8
print(string.format("bank at x=%d is %.0f, feet at %.0f", fx, banky(fx), fy))
figm = body_of{spine = {{fx, fy}, {fx-1, fy-30}, {fx+1, fy-49}}, widths = {13, 12, 9}}:mask()
       + ellipse(fx+1, fy-56, 4.2, 4.8)
figD = pile{{"lead white",0.4},{"raw umber",2.2},{"bone black",3.4},{"pale smalt",0.3}, medium=0.05}
print(figD)
for p=1,2 do
  work(figm, {hand="body", tool="filbert 6", pile=figD, coverage=2.6, edge="found",
              length={8,24}, clip=true, fill=true, seed=800+p})
end
-- the light on her right shoulder and sleeve
fb = brush("filbert", 3)
fb:load(litB, 0.7)
fb:stroke({{fx+5, fy-8}, {fx+6, fy-28}, {fx+5, fy-44}}, {pressure={0.45,0.5,0.42}, ramps={0.2,0.25}})
fb:touch(fx+3.5, fy-55, {pressure=0.4})
print("bank, fallen birch, figure")

--@ chunk 525
-- the figure: hem, waist, shoulders, collar, head — a woman seen from behind
figm2 = body_of{spine = {{600,1204},{600,1188},{600,1170},{600,1156},{601,1149}},
                widths = {13,11,9.5,11.2,9}}:mask() + ellipse(601,1141,4.0,4.6)
figD2 = pile{{"lead white",0.4},{"raw umber",2.6},{"bone black",2.4},{"pale smalt",0.40}, medium=0.05}
print(figD2)
for p=1,3 do
  work(figm2, {hand="body", tool="filbert 5", pile=figD2, coverage=2.2, edge="found",
               length={7,20}, clip=true, fill=true, seed=810+p})
end
-- the light on her right side, and a pale collar catching it
fb = brush("filbert", 2.6)
fb:load(litB, 0.6)
fb:stroke({{605,1198},{606,1176},{605,1156}}, {pressure={0.4,0.45,0.4}, ramps={0.2,0.25}})
fb:load(pile{{"lead white",2.8},{"pale smalt",0.6}, medium=0.05}, 0.7)
fb:stroke({{596,1154},{601,1151},{606,1153}}, {pressure={0.5,0.55,0.5}, ramps={0.2,0.2}})

-- the fallen birch, dulled and with a shadow under it
logm2 = ribbon({{236,1270},{380,1256},{516,1244},{652,1232},{766,1226}}, {11,12,10,8,5})
logP2 = pile{{"lead white",1.9},{"raw umber",1.9},{"pale smalt",0.9},{"bone black",0.5}, medium=0.06}
print(logP2)
for p=1,2 do
  work(logm2, {hand="body", tool="filbert 9", pile=logP2, coverage=2.4, angle=0.0,
               length={40,110}, clip=true, edge="found", seed=820+p})
end
logsh = ribbon({{238,1281},{380,1267},{516,1255},{652,1243},{766,1237}}, {10,11,9,7,4})
logShP = pile{{"lead white",0.3},{"raw umber",1.6},{"bone black",3.2}, medium=0.05}
work(logsh, {hand="body", tool="filbert 8", pile=logShP, coverage=2.0, angle=0.0,
             length={40,110}, clip=true, edge="found", seed=830})
print("figure and log redrawn")

--@ chunk 526
vapT = pile{{"lead white",2.6},{"yellow ochre",0.35},{"pale smalt",1.1},{"bone black",0.35}, medium=0.05}
print(vapT)
vapzone3 = mask(function(x,y) return smoothstep(520 + 55*nc(x,y), 700, y) * (1 - smoothstep(742 + 30*nd(x,y), 818, y)) end)
print(string.format("vap3  520 %.2f  620 %.2f  700 %.2f  770 %.2f  830 %.2f",
  vapzone3:at(400,520), vapzone3:at(400,620), vapzone3:at(400,700), vapzone3:at(400,770), vapzone3:at(400,830)))
for p=1,3 do
  work(vapzone3, {hand="broad", tool="filbert 24", pile=vapT, coverage=1.8, edge="found",
                  length={150,280}, dips={4,0.95,0.3}, clip=true, fill=true,
                  pressure={0.7,1.0}, seed=840+p})
end

-- striations lying along the vapour, so it is not a wall
vapLight = pile{{"lead white",3.0},{"yellow ochre",0.50},{"pale smalt",0.50},{"bone black",0.10}, medium=0.05}
vapDark  = pile{{"lead white",2.0},{"pale smalt",2.0},{"bone black",1.2},{"green earth",0.20}, medium=0.05}
local LS = { {180,690,300,18},{520,684,260,15},{300,712,220,13},{680,706,240,17},
             {120,730,200,20},{560,742,280,14},{420,762,190,12},{760,756,170,15} }
local DS = { {260,668,240,13},{620,662,200,12},{140,700,180,14},{460,724,260,11},
             {700,714,220,12},{340,748,200,10} }
local function streaks(L, blur)
  local m = nil
  for i=1,#L do
    local e = L[i]
    local r = ribbon({{e[1], e[2]}, {e[1]+e[3], e[2] + 4}}, e[4]):blur(blur)
    m = m and (m + r) or r
  end
  return m * vapzone3
end
work(streaks(LS, 9), {hand="body", tool="filbert 14", pile=vapLight, coverage=1.7, angle=0.0,
                      length={40,120}, clip=true, fill=true, seed=860})
work(streaks(DS, 8), {hand="body", tool="filbert 13", pile=vapDark, coverage=1.5, angle=0.0,
                      length={40,120}, clip=true, fill=true, seed=861})
print("vapour softened and striated")

--@ chunk 527
-- the zenith darker: a ramp that stays above the anchoring threshold
skydark = mask(function(x,y) return (1 - smoothstep(500, 640, y)) * (0.5 + 0.5*smoothstep(560, 40, y)) end)
print(string.format("skydark  40 %.2f  300 %.2f  500 %.2f  600 %.2f  700 %.2f",
  skydark:at(400,40), skydark:at(400,300), skydark:at(400,500), skydark:at(400,600), skydark:at(400,700)))
cloudDark = pile{{"lead white",1.6},{"pale smalt",2.2},{"bone black",1.1},{"green earth",0.15},
                {"cobalt blue",0.06}, medium=0.05}
cloudLite = pile{{"lead white",3.0},{"pale smalt",1.4},{"yellow ochre",0.25},{"bone black",0.10}, medium=0.05}
cloudWarm = pile{{"lead white",3.0},{"yellow ochre",0.80},{"pale smalt",0.70},{"bone black",0.12}, medium=0.05}
print(cloudDark); print(cloudLite); print(cloudWarm)
for p=1,2 do
  work(skydark, {hand="broad", tool="filbert 24", pile=cloudDark, coverage=1.8, edge="found",
                 length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                 pressure={0.7,0.95}, seed=870+p})
end

-- cloud bars drawn in, not coated on
local function band(x1, y, w, h, blur)
  return rect(x1, y, w, h):blur(blur)
end
local DARK = { {-40,96,1080,34,14},{80,214,760,26,13},{520,330,540,30,15},
               {-60,438,620,22,12},{240,516,700,20,12} }
local LITE = { {120,152,780,26,13},{-40,268,700,30,15},{300,380,720,24,13},
               {60,478,640,20,12} }
local WARM = { {140,556,720,22,13},{420,590,600,18,12} }
local function unionof(L)
  local m = nil
  for i=1,#L do
    local e = L[i]
    local b = band(e[1], e[2], e[3], e[4], e[5])
    b = b * mask(function(x,y) return 1 - smoothstep(470, 660, y) end)
    m = m and (m + b) or b
  end
  return m
end
work(unionof(DARK), {hand="body", tool="filbert 20", pile=cloudDark, coverage=1.6, angle=0.0,
                      length={120,260}, clip=true, fill=true, seed=880})
work(unionof(LITE), {hand="body", tool="filbert 20", pile=cloudLite, coverage=1.5, angle=0.0,
                      length={120,260}, clip=true, fill=true, seed=881})
work(unionof(WARM), {hand="body", tool="filbert 18", pile=cloudWarm, coverage=1.5, angle=0.0,
                      length={120,260}, clip=true, fill=true, seed=882})
print("sky graded and clouded")

--@ chunk 528
nc2 = noise{seed=71, octaves=4, period=380}
cloudmass = mask(function(x,y)
  return (1 - smoothstep(380, 620, y)) * (0.52 + 0.48*nc2(x, y*1.6))
end)
print(string.format("mass  100 %.2f  300 %.2f  450 %.2f  580 %.2f",
  cloudmass:at(400,100), cloudmass:at(400,300), cloudmass:at(400,450), cloudmass:at(400,580)))
cloudMid = pile{{"lead white",2.0},{"pale smalt",2.2},{"bone black",0.90},{"green earth",0.15},
               {"cobalt blue",0.05}, medium=0.05}
print(cloudMid)
for p=1,2 do
  work(cloudmass, {hand="broad", tool="filbert 26", pile=cloudMid, coverage=1.8, edge="found",
                   length={190,360}, dips={4,0.9,0.3}, clip=true, fill=true,
                   pressure={0.7,0.95}, seed=890+p})
end

-- knock the vapour's striations back toward one luminous mass
for p=1,2 do
  work(vapzone3, {hand="broad", tool="filbert 26", pile=vapT, coverage=1.3, edge="found",
                  length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                  pressure={0.7,0.95}, seed=895+p})
end
print("cloud mass laid, vapour massed")

--@ chunk 529
birchA = body_of{spine = {{20,1400},{38,1050},{52,700},{72,-30}}, widths = {86,74,64,50}, blend=0.7}:mask()
birchB = body_of{spine = {{174,1400},{186,1050},{194,700},{204,-30}}, widths = {56,50,44,34}, blend=0.7}:mask()
birches = birchA + birchB
for p=1,2 do
  work(birches, {hand="broad", tool="filbert 26", pile=birchD, coverage=2.2, edge="found",
                 length={220,420}, angle=math.pi/2, dips={5,0.9,0.25}, clip=true,
                 pressure={0.7,0.95}, seed=900+p})
end
lbB = brush("filbert", 12)
lbB:load(litB, 0.5)
lbB:stroke({{32,1400},{54,1050},{68,700},{92,-30}}, {pressure={0.42,0.38,0.4,0.42}, ramps={0.25,0.3}})
lbB:stroke({{186,1400},{198,1050},{206,700},{220,-30}}, {pressure={0.4,0.36,0.38,0.4}, ramps={0.25,0.3}})

-- the figure: take the hard white edge off her
figSoft = pile{{"lead white",1.2},{"raw umber",2.4},{"bone black",1.8},{"pale smalt",0.40}, medium=0.05}
fb2 = brush("filbert", 3.4)
fb2:load(figSoft, 0.6)
fb2:stroke({{606,1200},{607,1176},{606,1156}}, {pressure={0.4,0.42,0.4}, ramps={0.2,0.25}})

-- tufts along the lip of the bank
grassP = pile{{"lead white",1.0},{"green earth",2.2},{"raw umber",1.6},{"bone black",0.8}, medium=0.06}
gb = brush("filbert", 2.4)
local TUFT = {{300,1247},{392,1236},{470,1244},{676,1232},{742,1240},{820,1236},{236,1262},{880,1244}}
for i=1,#TUFT do
  local x, y = TUFT[i][1], TUFT[i][2]
  gb:load(grassP, 0.7)
  for k=1,4 do
    gb:stroke({{x + rand(-9,9), y + 2}, {x + rand(-13,13), y - rand(7,15)}},
              {pressure={0.5,0.2}})
  end
end
print("birches, figure, bank tufts")

--@ chunk 530
for p=1,3 do
  work(birches, {hand="body", tool="filbert 16", pile=birchD, coverage=2.2, edge="found",
                 length={40,110}, clip=true, fill=true, pressure={0.8,1.0}, seed=910+p})
end
lbB = brush("filbert", 8)
lbB:load(litB, 0.5)
lbB:stroke({{36,1390},{58,1050},{72,700},{96,-30}}, {pressure={0.45,0.4,0.42,0.45}, ramps={0.22,0.26}})
lbB:stroke({{190,1390},{202,1050},{210,700},{224,-30}}, {pressure={0.45,0.4,0.42,0.45}, ramps={0.22,0.26}})

-- bare limbs from the near birches, out into the evening sky
NB = {
  { {{56,300},{150,268},{214,258}}, {7,4.5,2.6} },
  { {{48,196},{132,158},{186,146}}, {6,4,2.4} },
  { {{70,404},{168,388},{246,386}}, {6.5,4.2,2.4} },
  { {{202,330},{280,300},{330,292}}, {5,3.4,2.0} },
  { {{206,224},{272,196},{314,188}}, {4.5,3,1.8} },
  { {{62,516},{142,500},{196,500}}, {6,4,2.2} },
}
nbm = nil
for i=1,#NB do
  local r = ribbon(NB[i][1], NB[i][2])
  nbm = nbm and (nbm + r) or r
end
work(nbm, {hand="body", tool="filbert 5", pile=birchD, coverage=2.4, edge="found",
           length={14,50}, clip=true, fill=true, seed=920})
print("birches dark, limbs added")

--@ chunk 531
-- a smooth dark body first, without fill dabs
for p=1,3 do
  work(birches, {hand="body", tool="filbert 20", pile=birchD, coverage=2.6, edge="found",
                 length={60,150}, clip=true, fill=false, pressure={0.85,1.0}, seed=930+p})
end
-- pale bark down the lit side
barkP = pile{{"lead white",2.8},{"raw umber",0.9},{"pale smalt",0.5},{"yellow ochre",0.25}, medium=0.06}
print(barkP)
bs = brush("filbert", 10)
bs:load(barkP, 0.6)
bs:stroke({{39,1400},{54,1050},{66,700},{83,-20}}, {pressure={0.5,0.48,0.5,0.5}, ramps={0.25,0.3}})
bs:stroke({{186,1400},{197,1050},{204,700},{211,-20}}, {pressure={0.5,0.48,0.5,0.5}, ramps={0.25,0.3}})

-- the dark scars on it
scarP = pile{{"lead white",0.3},{"raw umber",1.4},{"bone black",3.4}, medium=0.05}
sc = brush("filbert", 3.2)
local SC = {{45,140},{51,268},{58,392},{63,520},{68,648},{74,770},{79,900},{84,1030},
            {190,200},{195,330},{199,470},{203,610},{206,760},{208,910},{210,1080}}
for i=1,#SC do
  local x, y = SC[i][1], SC[i][2]
  sc:load(scarP, 0.7)
  sc:stroke({{x - rand(2,6), y}, {x + rand(3,8), y - rand(0,3)}},
            {pressure={0.55, 0.35}})
end
print("bark and scars")

--@ chunk 532
local s = {}
for x = 0, 130, 4 do s[#s+1] = string.format("%d:%.2f", x, birchA:at(x, 400)) end
print(table.concat(s, " "))
local s2 = {}
for x = 0, 130, 4 do s2[#s2+1] = string.format("%d:%.2f", x, birchA:at(x, 900)) end
print(table.concat(s2, " "))

--@ chunk 533
print("trunk", drying(50,400), drying(190,700))
blend(birchA + birchB, {angle=math.pi/2, clip=true})
print("trunks fused")

--@ chunk 534
local out = {}
for y = 100, 540, 20 do
  local x = 130
  while x > 0 and birchA:at(x, y) < 0.5 do x = x - 1 end
  out[#out+1] = string.format("y%d edge x%d", y, x)
end
print(table.concat(out, "  "))

--@ chunk 535
function trunkrib(x0, y0, x1, y1, w0, w1, bow)
  local pts, ws = {}, {}
  local n = 18
  for i = 0, n do
    local t = i / n
    pts[#pts+1] = {x0 + (x1-x0)*t + bow*math.sin(t*math.pi), y0 + (y1-y0)*t}
    ws[#ws+1] = w0 + (w1-w0)*t
  end
  return ribbon(pts, ws)
end
birchA = trunkrib(20, 1400, 72, -30, 86, 50, 7)
birchB = trunkrib(174, 1400, 204, -30, 56, 34, -5)
birches = birchA + birchB
local out = {}
for y = 100, 540, 20 do
  local x = 140
  while x > 0 and birchA:at(x, y) < 0.5 do x = x - 1 end
  out[#out+1] = string.format("y%d x%d", y, x)
end
print(table.concat(out, "  "))
for p=1,3 do
  work(birches, {hand="body", tool="filbert 18", pile=birchD, coverage=2.2, edge="found",
                 length={55,140}, clip=true, fill=true, pressure={0.85,1.0}, seed=940+p})
end
print("trunks, smooth")

--@ chunk 536
birchA = trunkrib(12, 1400, 64, -30, 100, 62, 7)
birchB = trunkrib(166, 1400, 196, -30, 68, 46, -5)
birches = birchA + birchB
for p=1,3 do
  work(birches, {hand="body", tool="filbert 18", pile=birchD, coverage=2.2, edge="found",
                 length={55,140}, clip=true, fill=true, pressure={0.85,1.0}, seed=950+p})
end
-- the lit bark down one side
bs = brush("filbert", 9)
bs:load(barkP, 0.6)
bs:stroke({{36,1400},{50,1050},{60,700},{74,-20}}, {pressure={0.5,0.48,0.5,0.5}, ramps={0.25,0.3}})
bs:stroke({{178,1400},{188,1050},{194,700},{204,-20}}, {pressure={0.5,0.48,0.5,0.5}, ramps={0.25,0.3}})
sc = brush("filbert", 3.2)
for i=1,#SC do
  local x, y = SC[i][1], SC[i][2]
  sc:load(scarP, 0.7)
  sc:stroke({{x - rand(2,6), y}, {x + rand(3,8), y - rand(0,3)}}, {pressure={0.55, 0.35}})
end
print("trunks widened, bark relaid")

--@ chunk 537
-- the dark middle the picture is missing: a treeline along the far shore
nt = noise{seed=91, octaves=4, period=150}
nq = noise{seed=97, octaves=3, period=70}
treeline = mask(function(x,y)
  local h = 800 - 40*nt(x, 780) - 14*nq(x, 790)
  return smoothstep(h - 7, h + 9, y) * (1 - smoothstep(826, 846, y))
end)
print(string.format("treeline  700 %.2f  760 %.2f  800 %.2f  830 %.2f  880 %.2f",
  treeline:at(400,700), treeline:at(400,760), treeline:at(400,800), treeline:at(400,830), treeline:at(400,880)))
treeP = pile{{"lead white",0.8},{"pale smalt",1.4},{"raw umber",2.0},{"bone black",1.8}, medium=0.05}
print(treeP)
for p=1,3 do
  work(treeline, {hand="body", tool="filbert 12", pile=treeP, coverage=2.2, edge="found",
                  length={30,80}, clip=true, fill=true, pressure={0.8,1.0}, seed=960+p})
end
print("far treeline laid")

--@ chunk 538
treeD = pile{{"lead white",0.4},{"pale smalt",1.0},{"raw umber",2.2},{"bone black",3.0}, medium=0.05}
print(treeD)
for p=1,2 do
  work(treeline, {hand="body", tool="filbert 12", pile=treeD, coverage=2.0, edge="found",
                  length={30,80}, clip=true, fill=true, pressure={0.85,1.0}, seed=970+p})
end
-- its reflection, broken and a little lighter than itself
ntr = noise{seed=101, octaves=4, period=190}
tref = mask(function(x,y)
  local d = y - 838
  return smoothstep(0, 14, d) * (1 - smoothstep(52, 104, d)) * (0.45 + 0.55*(0.5 + 0.5*ntr(x*1.4, y*2.4)))
end)
print(string.format("tref 840 %.2f  870 %.2f  910 %.2f  950 %.2f",
  tref:at(400,840), tref:at(400,870), tref:at(400,910), tref:at(400,950)))
treeR = pile{{"lead white",0.9},{"pale smalt",1.6},{"raw umber",2.4},{"bone black",2.0}, medium=0.06}
for p=1,2 do
  work(tref, {hand="body", tool="filbert 11", pile=treeR, coverage=1.3, angle=0.0,
              length={25,70}, clip=true, fill=false, edge="found", seed=975+p})
end
print("treeline darkened, reflection broken below")

--@ chunk 539
function trunkpts(xb, yb, xt, yt, w0, w1, bow)
  local pts, ws = {}, {}
  local n = 10
  for i = 0, n do
    local t = i / n
    pts[#pts+1] = {xb + (xt-xb)*t + bow*math.sin(t*math.pi), yb + (yt-yb)*t}
    ws[#ws+1] = w0 + (w1-w0)*t
  end
  return ribbon(pts, ws)
end
local function rankmask(L)
  local m = nil
  for i=1,#L do
    local e = L[i]
    local r = trunkpts(e[1], e[2], e[3], e[4], e[5], e[6], e[7] or 0)
    m = m and (m + r) or r
  end
  return m
end
FAR2 = {{228,836,221,700,4.0,1.6,2},{291,846,297,714,4.6,1.8,-2},{361,832,355,690,3.6,1.5,2},
        {440,850,447,722,5.0,2.0,-2},{511,842,506,698,4.0,1.6,2},{569,856,576,726,4.4,1.8,-2},
        {648,846,641,702,4.2,1.7,2},{726,838,732,716,5.0,2.0,-2},{809,852,802,694,3.8,1.5,2},
        {900,844,907,720,4.4,1.8,-2}}
MID2 = {{268,880,258,650,8.0,3.0,3},{345,892,354,664,7.0,2.8,-3},{418,876,410,642,9.0,3.2,3},
        {505,896,516,668,8.0,3.0,-3},{592,882,584,648,10.0,3.4,4},{690,890,700,662,7.5,3.0,-3},
        {798,872,788,634,9.0,3.2,3}}
NEAR2 = {{318,928,302,622,13.0,4.5,5},{455,940,470,640,11.0,4.0,-4},
         {600,918,588,614,15.0,5.0,6},{762,930,778,634,12.0,4.5,-5}}
woodF2, woodM2, woodN2 = rankmask(FAR2), rankmask(MID2), rankmask(NEAR2)
print(string.format("far %.0f mid %.0f near %.0f", woodF2:area(), woodM2:area(), woodN2:area()))

for p=1,2 do
  work(woodF2, {hand="body", tool="filbert 4", pile=wfarP, coverage=2.6, edge="found",
                length={14,50}, clip=true, fill=true, seed=980+p})
end
for p=1,2 do
  work(woodM2, {hand="body", tool="filbert 7", pile=wmidP, coverage=2.4, edge="found",
                length={16,55}, clip=true, fill=true, seed=985+p})
end
for p=1,2 do
  work(woodN2, {hand="body", tool="filbert 11", pile=wnearP, coverage=2.4, edge="found",
                length={18,60}, clip=true, fill=true, seed=990+p})
end
print("wood re-drawn in front of the mist")

--@ chunk 540
-- limbs on the middle-ground trees
WB = {
 { {{316,700},{268,664},{238,650}}, {3.0,2.0,1.1} },
 { {{317,772},{370,744},{402,734}}, {2.6,1.7,1.0} },
 { {{458,700},{510,668},{542,656}}, {2.8,1.8,1.0} },
 { {{462,782},{410,754},{382,744}}, {2.6,1.7,1.0} },
 { {{598,690},{548,656},{518,644}}, {3.2,2.1,1.2} },
 { {{592,770},{646,742},{680,732}}, {2.8,1.8,1.1} },
 { {{764,700},{818,668},{850,656}}, {3.0,2.0,1.1} },
 { {{766,782},{712,754},{684,744}}, {2.6,1.7,1.0} },
 { {{268,722},{230,696},{206,684}}, {1.8,1.2,0.7} },
 { {{350,732},{390,708},{412,698}}, {1.8,1.2,0.7} },
 { {{412,700},{376,676},{356,666}}, {1.8,1.2,0.7} },
 { {{510,742},{552,718},{574,708}}, {1.8,1.2,0.7} },
 { {{586,702},{548,676},{528,666}}, {2.0,1.3,0.8} },
 { {{696,732},{736,708},{758,698}}, {1.8,1.2,0.7} },
 { {{790,700},{754,674},{734,664}}, {2.0,1.3,0.8} },
}
br2 = nil
for i=1,#WB do
  local r = ribbon(WB[i][1], WB[i][2])
  br2 = br2 and (br2 + r) or r
end
work(br2, {hand="body", tool="filbert 4", pile=wmidP, coverage=2.2, edge="found",
            length={12,40}, clip=true, fill=true, seed=995})
work(br2 * woodM2, {hand="body", tool="filbert 4", pile=wmidP, coverage=1.6, edge="found",
                    length={12,40}, clip=true, fill=true, seed=996})

-- reflections: broken, straight down from each base, a breath darker than the water
function reflmask2(rank, len, spread)
  local m = nil
  for i=1,#rank do
    local e = rank[i]
    local xb, yb, xt, w = e[1], e[2], e[3], e[5]
    local d = (xt-xb) * 0.6
    local r = ribbon({{xb, yb + 6}, {xb + d*0.4, yb + len*0.5}, {xb + d, yb + len}},
                      {w*0.8, w*0.9, w*spread}):blur(3)
    m = m and (m + r) or r
  end
  return m
end
qM = reflmask2(MID2, 88, 0.7)
qN = reflmask2(NEAR2, 130, 0.6)
stipple(qM, {pile=rMp, width=4.4, coverage=4.0, drag={12, math.pi/2}, feather=0.3, clip=qM, seed=1000})
stipple(qN, {pile=rNp, width=6.0, coverage=4.6, drag={16, math.pi/2}, feather=0.3, clip=qN, seed=1001})
print("limbs and reflections")

--@ chunk 541
-- clear the near birches' smeared reflections: calm water in the left foreground
birm = mask(function(x,y) return (1 - smoothstep(170, 265, x)) * smoothstep(840, 884, y) * (1 - smoothstep(1130, 1240, y)) end)
print(string.format("birm 800 %.2f  880 %.2f  1000 %.2f  1180 %.2f  1260 %.2f",
  birm:at(100,800), birm:at(100,880), birm:at(100,1000), birm:at(100,1180), birm:at(100,1260)))
for p=1,2 do
  work(birm, {hand="broad", tool="filbert 26", pile=watS, coverage=2.0, edge="found",
              length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
              pressure={0.7,1.0}, seed=1010+p})
end
work(birm, {hand="broad", tool="filbert 26", pile=watdS, coverage=1.0, edge="found",
            length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
            pressure={0.7,0.95}, seed=1015})
for p=1,2 do
  work(birm, {hand="body", tool="filbert 7", pile=ripP, angle=0.0, length={45,165},
              coverage=0.7, clip=true, fill=false, edge="found", broken=0.6, seed=1020+p})
end

-- a few broad rests in the ripple, so the flood is not all scales
nr = noise{seed=113, octaves=3, period=260}
calm = mask(function(x,y) return smoothstep(840, 890, y) * clamp(0.62 + 0.55*nr(x*0.8, y*1.3), 0, 1) end)
work(calm, {hand="body", tool="filbert 20", pile=ripP, angle=0.0, length={90,260},
            coverage=0.55, clip=true, fill=false, edge="found", seed=1025})
print("foreground calmed")

--@ chunk 542
wz2 = mask(function(x,y) return smoothstep(842, 892, y) * (1 - 0.55*smoothstep(950, 1300, y)) end)
wdeepf = mask(function(x,y) return smoothstep(842, 892, y) * (0.35 + 0.65*smoothstep(950, 1300, y)) end)
print(string.format("wz2 880 %.2f 1000 %.2f 1200 %.2f 1380 %.2f",
  wz2:at(400,880), wz2:at(400,1000), wz2:at(400,1200), wz2:at(400,1380)))
for p=1,2 do
  work(wz2, {hand="broad", tool="filbert 26", pile=watS, coverage=2.0, edge="found",
             length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
             pressure={0.7,1.0}, seed=1030+p})
end
for p=1,2 do
  work(wdeepf, {hand="broad", tool="filbert 26", pile=watdS, coverage=1.9, edge="found",
                length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                pressure={0.7,1.0}, seed=1040+p})
end
ripP2 = pile{{"lead white",2.2},{"pale smalt",2.2},{"bone black",1.10},{"green earth",0.20}, medium=0.05}
ripD2 = pile{{"lead white",1.5},{"pale smalt",2.0},{"bone black",1.90},{"green earth",0.20}, medium=0.05}
for p=1,2 do
  work(wz2, {hand="body", tool="filbert 6", pile=ripP2, angle=0.0, length={40,150},
             coverage=0.85, clip=true, fill=false, edge="found", broken=0.5, seed=1050+p})
end
for p=1,2 do
  work(wz2, {hand="body", tool="filbert 5", pile=ripD2, angle=0.0, length={32,120},
             coverage=0.65, clip=true, fill=false, edge="found", broken=0.45, seed=1060+p})
end
stipple(qM, {pile=rMp, width=4.4, coverage=4.0, drag={12, math.pi/2}, feather=0.3, clip=qM, seed=1070})
stipple(qN, {pile=rNp, width=6.0, coverage=4.6, drag={16, math.pi/2}, feather=0.3, clip=qN, seed=1071})
print("water re-laid, ripples finer, reflections back")

--@ chunk 543
bankD = pile{{"lead white",0.7},{"raw umber",3.2},{"bone black",2.6},{"green earth",0.40}, medium=0.05}
for p=1,3 do
  work(bankm, {hand="broad", tool="filbert 24", pile=bankD, coverage=2.0, angle=0.0,
               length={150,300}, dips={4,0.95,0.3}, clip=true, fill=true,
               pressure={0.75,1.0}, seed=1080+p})
end
-- a mossy lip catching the light along the bank's edge
lipm = mask(function(x,y) local e = banky(x); return smoothstep(e - 34, e + 6, y) * (1 - smoothstep(e + 14, e + 48, y)) end)
lipP = pile{{"lead white",1.8},{"raw umber",2.2},{"green earth",1.6},{"bone black",0.5}, medium=0.06}
work(lipm, {hand="body", tool="filbert 11", pile=lipP, coverage=1.5, angle=0.0,
            length={40,120}, clip=true, fill=true, edge="found", seed=1090})

-- the fallen birch, with a lit top and a dark underside
logm2 = ribbon({{236,1268},{380,1254},{516,1242},{652,1230},{766,1224}}, {11,12,10,8,5})
work(logm2, {hand="body", tool="filbert 9", pile=logP2, coverage=2.4, angle=0.0,
             length={40,110}, clip=true, fill=true, edge="found", seed=1091})
logtop = ribbon({{240,1263},{382,1249},{518,1237},{654,1225},{766,1220}}, {5,6,5,4,2.5})
logTopP = pile{{"lead white",2.8},{"raw umber",1.2},{"pale smalt",0.7}, medium=0.05}
work(logtop, {hand="body", tool="filbert 6", pile=logTopP, coverage=2.2, angle=0.0,
              length={40,110}, clip=true, fill=true, edge="found", seed=1092})
logsh = ribbon({{238,1278},{380,1264},{516,1252},{652,1240},{766,1234}}, {10,11,9,7,4})
work(logsh, {hand="body", tool="filbert 8", pile=logShP, coverage=2.0, angle=0.0,
             length={40,110}, clip=true, fill=true, edge="found", seed=1093})

-- the figure, again
figD2 = pile{{"lead white",0.4},{"raw umber",2.6},{"bone black",2.4},{"pale smalt",0.40}, medium=0.05}
for p=1,3 do
  work(figm2, {hand="body", tool="filbert 5", pile=figD2, coverage=2.2, edge="found",
               length={7,20}, clip=true, fill=true, seed=1100+p})
end
fb2 = brush("filbert", 2.6)
fb2:load(figSoft, 0.6)
fb2:stroke({{606,1200},{607,1176},{606,1156}}, {pressure={0.4,0.42,0.4}, ramps={0.2,0.25}})
fb2:load(pile{{"lead white",2.8},{"pale smalt",0.6}, medium=0.05}, 0.7)
fb2:stroke({{596,1154},{601,1151},{606,1153}}, {pressure={0.5,0.55,0.5}, ramps={0.2,0.2}})
print("bank, lip, log, figure")

--@ chunk 544
-- a quieter lip on the bank
lipP2 = pile{{"lead white",1.6},{"raw umber",2.6},{"green earth",1.0},{"pale smalt",0.8},{"bone black",0.7}, medium=0.06}
work(lipm, {hand="body", tool="filbert 11", pile=lipP2, coverage=2.0, angle=0.0,
            length={40,120}, clip=true, fill=true, edge="found", seed=1110})

-- the log: a birch lying in shadow, not a white stick
logP3 = pile{{"lead white",1.7},{"raw umber",2.2},{"pale smalt",1.0},{"bone black",0.6}, medium=0.05}
work(logm2, {hand="body", tool="filbert 9", pile=logP3, coverage=2.4, angle=0.0,
             length={40,110}, clip=true, fill=true, edge="found", seed=1111})
logTopP2 = pile{{"lead white",2.2},{"raw umber",1.5},{"pale smalt",0.8}, medium=0.05}
work(logtop, {hand="body", tool="filbert 6", pile=logTopP2, coverage=1.5, angle=0.0,
              length={40,110}, clip=true, fill=true, edge="found", seed=1112})

-- long calm swells drawn by hand across the flood
swellD = pile{{"lead white",1.5},{"pale smalt",1.8},{"bone black",1.60},{"green earth",0.20}, medium=0.05}
swellL = pile{{"lead white",2.6},{"pale smalt",2.2},{"bone black",0.70},{"green earth",0.20}, medium=0.05}
sw = brush("filbert", 26)
local SW = {
 {900, swellD, 0.42}, {968, swellL, 0.40}, {1038, swellD, 0.40},
 {1112, swellL, 0.42}, {1188, swellD, 0.40}, {1262, swellL, 0.40}}
for i=1,#SW do
  local e = SW[i]
  sw:load(e[2], 0.42)
  sw:stroke({{238, e[1]}, {430, e[1]-7}, {640, e[1]+4}, {850, e[1]-4}, {986, e[1]}},
            {pressure={e[3], e[3]*0.8, e[3], e[3]*0.8, e[3]}, ramps={0.4,0.45}})
end

-- stronger reflections under the standing trees
stipple(qM, {pile=rMp, width=5.0, coverage=6.0, drag={13, math.pi/2}, feather=0.25, clip=qM, seed=1120})
stipple(qN, {pile=rNp, width=6.5, coverage=7.0, drag={18, math.pi/2}, feather=0.25, clip=qN, seed=1121})
print("lip, log, swells, reflections")

--@ chunk 545
fx, fy = 600, banky(600) + 8
figbody = ribbon({{fx, fy}, {fx-1, fy-16}, {fx-2, fy-33}, {fx-2, fy-48}, {fx, fy-60}},
                 {15, 12, 10, 13.5, 9})
fighead = ellipse(fx, fy-68, 5.0, 5.6)
figm3 = figbody + fighead + figm2
print(string.format("figure at %.0f,%.0f; %.0f sq units", fx, fy, figm3:area()))
for p=1,3 do
  work(figm3, {hand="body", tool="filbert 5", pile=figD2, coverage=2.2, edge="found",
               length={7,20}, clip=true, fill=true, seed=1130+p})
end
-- the light: her right side, a pale shawl over the shoulders, a warm head
fb3 = brush("filbert", 2.6)
fb3:load(figSoft, 0.6)
fb3:stroke({{fx+6, fy-4}, {fx+6, fy-20}, {fx+5, fy-34}, {fx+5, fy-48}}, {pressure={0.42,0.44,0.42,0.42}, ramps={0.2,0.25}})
shawlP = pile{{"lead white",2.0},{"raw umber",1.8},{"pale smalt",0.8}, medium=0.05}
fb3:load(shawlP, 0.7)
fb3:stroke({{fx-8, fy-56}, {fx-1, fy-60}, {fx+6, fy-55}}, {pressure={0.5,0.58,0.5}, ramps={0.2,0.2}})
fb3:load(pile{{"lead white",2.4},{"yellow ochre",0.9},{"pale smalt",0.5}, medium=0.05}, 0.6)
fb3:stroke({{fx+2, fy-72}, {fx+4, fy-67}}, {pressure={0.5, 0.35}})
print("figure redrawn")

--@ chunk 546
-- a quieter lip: it was reading as a ledge across the bank
lipP3 = pile{{"lead white",1.3},{"raw umber",2.9},{"green earth",0.8},{"pale smalt",0.8},{"bone black",0.9}, medium=0.06}
work(lipm, {hand="body", tool="filbert 11", pile=lipP3, coverage=2.0, angle=0.0,
            length={40,120}, clip=true, fill=true, edge="found", seed=1140})

-- the figure, again: sloping shoulders, a waist, a smaller head on a neck
fx, fy = 600, banky(600) + 6
fig4 = ribbon({{fx, fy}, {fx, fy-10}, {fx-1, fy-25}, {fx-1, fy-39}, {fx, fy-51}, {fx+1, fy-58}},
              {14, 12, 9.5, 11.5, 12.5, 6.5})
fig4 = fig4 + ellipse(fx+1, fy-64, 4.2, 4.8)
print(string.format("new figure %.0f sq units, from y %.0f to %.0f", fig4:area(), fy-69, fy))
for p=1,3 do
  work(fig4, {hand="body", tool="filbert 4", pile=figD2, coverage=2.2, edge="found",
              length={6,18}, clip=true, fill=true, seed=1150+p})
end
-- soft light down the right of her, and a shawl catching it
fb4 = brush("filbert", 3.2)
fb4:load(figSoft, 0.45)
fb4:stroke({{fx+5, fy-3}, {fx+5, fy-18}, {fx+4, fy-33}, {fx+4, fy-46}}, {pressure={0.3,0.32,0.3,0.3}, ramps={0.25,0.3}})
fb4:load(shawlP, 0.55)
fb4:stroke({{fx-7, fy-52}, {fx, fy-56}, {fx+6, fy-51}}, {pressure={0.4,0.46,0.4}, ramps={0.25,0.25}})
fb4:load(pile{{"lead white",2.4},{"yellow ochre",0.9},{"pale smalt",0.5}, medium=0.05}, 0.5)
fb4:stroke({{fx+2, fy-68}, {fx+3, fy-63}}, {pressure={0.42, 0.28}})
print("figure, lip")

--@ chunk 547
-- knock the torn cloud strips back
skypatch = mask(function(x,y) return smoothstep(360, 430, y) * (1 - smoothstep(560, 610, y)) * (1 - smoothstep(0, 40, y)) end)
work(skypatch, {hand="broad", tool="filbert 26", pile=cloudMid, coverage=1.6, edge="found",
                length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                pressure={0.7,0.95}, seed=1160})

-- stronger reflections
rMp2 = pile{{"lead white",0.7},{"pale smalt",1.4},{"bone black",2.8},{"raw umber",1.0}, medium=0.05}
rNp2 = pile{{"lead white",0.3},{"pale smalt",1.0},{"bone black",3.6},{"raw umber",1.2}, medium=0.05}
stipple(qM, {pile=rMp2, width=5.2, coverage=6.5, drag={13, math.pi/2}, feather=0.25, clip=qM, seed=1161})
stipple(qN, {pile=rNp2, width=6.8, coverage=8.0, drag={18, math.pi/2}, feather=0.25, clip=qN, seed=1162})

-- drowned branches lying on the flood, each with a little reflection
debP = pile{{"lead white",0.6},{"raw umber",2.4},{"bone black",2.6}, medium=0.05}
db = brush("filbert", 3.4)
local DEB = {
 {{{300,986},{362,977},{412,981}}, 0.55},
 {{{520,1052},{602,1043},{670,1049}}, 0.6},
 {{{700,942},{762,935},{814,941}}, 0.5},
 {{{340,1128},{422,1119},{488,1125}}, 0.6},
 {{{812,1112},{890,1102},{946,1107}}, 0.55}}
for i=1,#DEB do
  local e = DEB[i]
  db:load(debP, 0.75)
  db:stroke(e[1], {pressure={e[2], e[2]*0.7, e[2]*0.85}, ramps={0.2,0.25}})
  db:load(pile{{"lead white",1.4},{"pale smalt",1.8},{"bone black",1.6}, medium=0.05}, 0.4)
  db:stroke({{e[1][1][1], e[1][1][2]+7}, {e[1][3][1], e[1][3][2]+7}}, {pressure={0.22, 0.16}})
end

-- a clump of reeds at the right
reedP = pile{{"lead white",1.2},{"raw umber",2.2},{"green earth",1.0},{"bone black",1.2}, medium=0.05}
rd = brush("filbert", 2.2)
for i=1,26 do
  local x = 846 + rand(-26, 34)
  local y = 1164 + rand(-10, 8)
  local h = rand(34, 96)
  local lean = rand(-14, 14)
  rd:load(reedP, 0.7)
  rd:stroke({{x, y}, {x + lean*0.4, y - h*0.6}, {x + lean, y - h}},
            {pressure={0.5, 0.35, 0.12}})
end
print("cloud strips, reflections, drowned branches, reeds")

--@ chunk 548
-- the flood deepens toward the viewer: a ramp that never crosses the anchoring threshold
wdrk = mask(function(x,y) return smoothstep(900, 1020, y) * (0.35 + 0.65*smoothstep(1020, 1330, y)) end)
print(string.format("wdrk 900 %.2f 1000 %.2f 1100 %.2f 1200 %.2f 1350 %.2f",
  wdrk:at(400,900), wdrk:at(400,1000), wdrk:at(400,1100), wdrk:at(400,1200), wdrk:at(400,1350)))
wdeepS = pile{{"lead white",1.6},{"pale smalt",2.3},{"bone black",2.2},{"green earth",0.30}, medium=0.05}
print(wdeepS)
for p=1,2 do
  work(wdrk, {hand="broad", tool="filbert 26", pile=wdeepS, coverage=1.8, angle=0.0,
              length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
              pressure={0.75,1.0}, seed=1170+p})
end
-- ripple back over it
for p=1,2 do
  work(wdrk, {hand="body", tool="filbert 6", pile=ripP2, angle=0.0, length={40,150},
              coverage=0.8, clip=true, fill=false, edge="found", broken=0.5, seed=1180+p})
end
for p=1,2 do
  work(wdrk, {hand="body", tool="filbert 5", pile=ripD2, angle=0.0, length={32,120},
              coverage=0.6, clip=true, fill=false, edge="found", broken=0.45, seed=1190+p})
end
stipple(qM, {pile=rMp2, width=5.2, coverage=6.0, drag={13, math.pi/2}, feather=0.25, clip=qM, seed=1200})
stipple(qN, {pile=rNp2, width=6.8, coverage=7.0, drag={18, math.pi/2}, feather=0.25, clip=qN, seed=1201})

-- the cloud strips, knocked back harder
work(skypatch, {hand="broad", tool="filbert 26", pile=cloudMid, coverage=1.8, edge="found",
                length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                pressure={0.7,0.95}, seed=1210})
print("water deepened, ripples back, cloud strips down")

--@ chunk 549
wdrkSafe = wdrk * (-bankm)
print(string.format("wdrkSafe 1000 %.2f 1200 %.2f 1330 %.2f",
  wdrkSafe:at(400,1000), wdrkSafe:at(400,1200), wdrkSafe:at(400,1330)))

for p=1,3 do
  work(bankm, {hand="broad", tool="filbert 24", pile=bankD, coverage=2.0, angle=0.0,
               length={150,300}, dips={4,0.95,0.3}, clip=true, fill=true,
               pressure={0.75,1.0}, seed=1220+p})
end
-- a deeper hollow toward the right of the bank, so it is not one flat mass
nbk = noise{seed=127, octaves=3, period=240}
hollow = mask(function(x,y) return smoothstep(1000, 1330, y) * clamp(0.7*nbk(x, 1300) + 0.45, 0, 1) end)
hollow = hollow * bankm
print(string.format("hollow 1200 %.2f 1330 %.2f", hollow:at(800,1200), hollow:at(800,1330)))
hollowP = pile{{"lead white",0.4},{"raw umber",2.6},{"bone black",3.2},{"green earth",0.5}, medium=0.05}
for p=1,2 do
  work(hollow, {hand="broad", tool="filbert 24", pile=hollowP, coverage=1.6, angle=0.0,
                length={150,300}, dips={4,0.9,0.3}, clip=true, fill=true,
                pressure={0.75,1.0}, seed=1230+p})
end
work(lipm, {hand="body", tool="filbert 11", pile=lipP3, coverage=1.8, angle=0.0,
            length={40,120}, clip=true, fill=true, edge="found", seed=1240})

work(logm2, {hand="body", tool="filbert 9", pile=logP3, coverage=2.4, angle=0.0,
             length={40,110}, clip=true, fill=true, edge="found", seed=1241})
work(logtop, {hand="body", tool="filbert 6", pile=logTopP2, coverage=1.5, angle=0.0,
              length={40,110}, clip=true, fill=true, edge="found", seed=1242})
work(logsh, {hand="body", tool="filbert 8", pile=logShP, coverage=2.0, angle=0.0,
             length={40,110}, clip=true, fill=true, edge="found", seed=1243})

for p=1,3 do
  work(fig4, {hand="body", tool="filbert 4", pile=figD2, coverage=2.2, edge="found",
              length={6,18}, clip=true, fill=true, seed=1250+p})
end
fb4 = brush("filbert", 3.2)
fb4:load(figSoft, 0.45)
fb4:stroke({{fx+5, fy-3}, {fx+5, fy-18}, {fx+4, fy-33}, {fx+4, fy-46}}, {pressure={0.3,0.32,0.3,0.3}, ramps={0.25,0.3}})
fb4:load(shawlP, 0.55)
fb4:stroke({{fx-7, fy-52}, {fx, fy-56}, {fx+6, fy-51}}, {pressure={0.4,0.46,0.4}, ramps={0.25,0.25}})
fb4:load(pile{{"lead white",2.4},{"yellow ochre",0.9},{"pale smalt",0.5}, medium=0.05}, 0.5)
fb4:stroke({{fx+2, fy-68}, {fx+3, fy-63}}, {pressure={0.42, 0.28}})
gb = brush("filbert", 2.4)
for i=1,#TUFT do
  local x, y = TUFT[i][1], TUFT[i][2]
  gb:load(grassP, 0.7)
  for k=1,4 do
    gb:stroke({{x + rand(-9,9), y + 2}, {x + rand(-13,13), y - rand(7,15)}}, {pressure={0.5,0.2}})
  end
end
print("bank, hollow, lip, log, figure, tufts")

--@ chunk 550
-- mist rising into the sky: a stipple whose coverage is a function gives a true gradient,
-- where a mask ramp would only stop at the anchoring threshold
hazeP = pile{{"lead white",3.0},{"yellow ochre",0.30},{"pale smalt",0.80}, medium=0.05}
print(hazeP)
hazeZone = rect(0, 430, 1000, 330)
stipple(hazeZone, {pile=hazeP, width=5, coverage=function(x,y) return 1.15*smoothstep(452, 706, y) end,
                  cluster={0.5, 26}, clip=hazeZone, seed=1300})

-- the last light along the top of the far shore
lipwarm = pile{{"lead white",2.6},{"yellow ochre",0.8},{"pale smalt",0.6}, medium=0.05}
lw2 = brush("filbert", 12)
for k=0,26 do
  local x = -20 + k*40 + rand(-8, 8)
  local h = 800 - 40*nt(x, 780) - 14*nq(x, 790)
  lw2:load(lipwarm, 0.5)
  lw2:stroke({{x, h + 4}, {x + rand(14, 26), h + 1}}, {pressure={0.32, 0.14}})
end

-- warmth along the lip of the near bank
lw3 = brush("filbert", 9)
lw3:load(lipwarm, 0.4)
for k=0,30 do
  local x = -20 + k*36 + rand(-8, 8)
  lw3:stroke({{x, banky(x) + 2}, {x + rand(16, 30), banky(x) + 6}}, {pressure={0.3, 0.12}})
end
print("haze, far-shore light, warm lip")

--@ chunk 551
hazeFixP = pile{{"lead white",2.4},{"yellow ochre",0.25},{"pale smalt",1.4},{"bone black",0.45}, medium=0.05}
print(hazeFixP)
hazeFix = mask(function(x,y)
  return smoothstep(432, 502, y) * (1 - 0.45*smoothstep(700, 800, y)) * (1 - smoothstep(790, 865, y))
end)
print(string.format("hazeFix 440 %.2f 500 %.2f 600 %.2f 700 %.2f 800 %.2f 860 %.2f",
  hazeFix:at(400,440), hazeFix:at(400,500), hazeFix:at(400,600), hazeFix:at(400,700), hazeFix:at(400,800), hazeFix:at(400,860)))
for p=1,2 do
  work(hazeFix, {hand="broad", tool="filbert 26", pile=hazeFixP, coverage=2.0, angle=0.0,
                 length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                 pressure={0.7,1.0}, seed=1310+p})
end
print("transition laid as one film")

--@ chunk 552
-- darker wood
wfarP2 = pile{{"lead white",1.2},{"pale smalt",1.6},{"raw umber",1.6},{"bone black",1.0}, medium=0.05}
wmidP2 = pile{{"lead white",0.7},{"pale smalt",1.0},{"raw umber",2.2},{"bone black",2.2}, medium=0.05}
wnearP2 = pile{{"lead white",0.35},{"pale smalt",0.7},{"raw umber",2.4},{"bone black",3.2}, medium=0.05}
print(wfarP2); print(wmidP2); print(wnearP2)
for p=1,2 do
  work(woodF2, {hand="body", tool="filbert 4", pile=wfarP2, coverage=2.8, edge="found",
                length={16,55}, clip=true, fill=false, seed=1320+p})
end
for p=1,2 do
  work(woodM2, {hand="body", tool="filbert 7", pile=wmidP2, coverage=2.8, edge="found",
                length={18,60}, clip=true, fill=false, seed=1330+p})
end
for p=1,2 do
  work(woodN2, {hand="body", tool="filbert 11", pile=wnearP2, coverage=2.8, edge="found",
                length={20,64}, clip=true, fill=false, seed=1340+p})
end
work(br2, {hand="body", tool="filbert 4", pile=wmidP2, coverage=2.4, edge="found",
           length={14,48}, clip=true, fill=false, seed=1350})

-- the far shore: a soft dark mass, not a row of loops
treel = treeline:blur(7)
print(string.format("treel 770 %.2f 800 %.2f 830 %.2f 860 %.2f",
  treel:at(400,770), treel:at(400,800), treel:at(400,830), treel:at(400,860)))
for p=1,3 do
  work(treel, {hand="body", tool="filbert 15", pile=treeD, coverage=2.4, edge="found",
               length={35,95}, clip=true, fill=true, pressure={0.8,1.0}, seed=1360+p})
end
print("wood darkened, far shore re-laid")

--@ chunk 553
-- the far shore as a wood: clumps of trees, not a hedge
ntr3 = noise{seed=131, octaves=3, period=95}
tl = nil
for i=0,95 do
  local x = -30 + i*11.2
  local h = 14 + 30*rand() + 16*ntr3(x, 790)
  local e = ellipse(x + rand(-4,4), 818 - h*0.55, rand(7,14), h*0.62)
  tl = tl and (tl + e) or e
end
tl = tl:blur(3)
print(string.format("treeline wood %.0f sq units", tl:area()))
treeP3 = pile{{"lead white",1.0},{"pale smalt",1.6},{"raw umber",2.2},{"bone black",1.6}, medium=0.05}
for p=1,2 do
  work(tl, {hand="body", tool="filbert 13", pile=treeP3, coverage=2.4, edge="found",
            length={30,80}, clip=true, fill=true, pressure={0.8,1.0}, seed=1370+p})
end
-- and the last of the light along its top
tlw = brush("filbert", 10)
for k=0,40 do
  local x = -20 + k*26 + rand(-8, 8)
  local y = 818 - 40 - rand(0, 26)
  tlw:load(lipwarm, 0.45)
  tlw:stroke({{x, y + 3}, {x + rand(12, 22), y + 1}}, {pressure={0.28, 0.12}})
end

-- trunks stroked downward, so the marks read as bark and not as rings
for p=1,2 do
  work(woodF2, {hand="body", tool="filbert 4", pile=wfarP2, coverage=2.6, edge="found",
                angle=math.pi/2, length={20,60}, clip=true, fill=false, seed=1380+p})
end
for p=1,2 do
  work(woodM2, {hand="body", tool="filbert 7", pile=wmidP2, coverage=2.6, edge="found",
                angle=math.pi/2, length={[0]=24,22,70}, clip=true, fill=false, seed=1390+p})
end
for p=1,2 do
  work(woodN2, {hand="body", tool="filbert 11", pile=wnearP2, coverage=2.6, edge="found",
                angle=math.pi/2, length={26,80}, clip=true, fill=false, seed=1400+p})
end
print("far shore a wood, trunks stroked down")

--@ chunk 554
for p=1,3 do
  work(birches, {hand="body", tool="filbert 18", pile=birchD, coverage=2.4, edge="found",
                 angle=math.pi/2, length={70,170}, clip=true, fill=false,
                 pressure={0.85,1.0}, seed=1410+p})
end
bs = brush("filbert", 9)
bs:load(barkP, 0.6)
bs:stroke({{36,1400},{50,1050},{60,700},{74,-20}}, {pressure={0.5,0.48,0.5,0.5}, ramps={0.25,0.3}})
bs:stroke({{178,1400},{188,1050},{194,700},{204,-20}}, {pressure={0.5,0.48,0.5,0.5}, ramps={0.25,0.3}})
sc = brush("filbert", 3.2)
for i=1,#SC do
  local x, y = SC[i][1], SC[i][2]
  sc:load(scarP, 0.7)
  sc:stroke({{x - rand(2,6), y}, {x + rand(3,8), y - rand(0,3)}}, {pressure={0.55, 0.35}})
end
work(nbm, {hand="body", tool="filbert 5", pile=birchD, coverage=2.4, edge="found",
           length={16,55}, clip=true, fill=true, seed=1420})

-- only their feet stand in the vapour
mistveilP = pile{{"lead white",2.9},{"yellow ochre",0.35},{"pale smalt",0.60}, medium=0.10}
footm = mask(function(x,y) return smoothstep(672, 800, y) end)
print(string.format("feet 660 %.2f 740 %.2f 820 %.2f", footm:at(60,660), footm:at(60,740), footm:at(60,820)))
work(birches * footm, {hand="body", tool="filbert 20", pile=mistveilP, coverage=1.4, edge="found",
                       angle=math.pi/2, length={80,180}, clip=true, fill=false, seed=1430})
print("near birches dark, feet in the vapour")

--@ chunk 555
-- fill=false left the old pale dabs showing through; close them with one filling pass
for p=1,2 do
  work(birches, {hand="body", tool="filbert 18", pile=birchD, coverage=2.2, edge="found",
                 angle=math.pi/2, length={70,170}, clip=true, fill=true,
                 pressure={0.85,1.0}, seed=1440+p})
end
bs = brush("filbert", 7)
bs:load(barkP, 0.55)
bs:stroke({{36,1400},{50,1050},{60,700},{74,-20}}, {pressure={0.48,0.46,0.48,0.48}, ramps={0.3,0.35}})
bs:stroke({{178,1400},{188,1050},{194,700},{204,-20}}, {pressure={0.48,0.46,0.48,0.48}, ramps={0.3,0.35}})

-- the torn strips in the sky
skystrip = mask(function(x,y) return smoothstep(300, 360, y) * (1 - smoothstep(430, 500, y)) end)
print(string.format("strip 310 %.2f 380 %.2f 460 %.2f", skystrip:at(400,310), skystrip:at(400,380), skystrip:at(400,460)))
for p=1,2 do
  work(skystrip, {hand="broad", tool="filbert 26", pile=cloudMid, coverage=1.9, angle=0.0,
                  length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                  pressure={0.7,0.95}, seed=1450+p})
end

-- the far shore's light carried down onto the flood
warmGlow = pile{{"lead white",2.2},{"yellow ochre",0.6},{"pale smalt",1.2},{"bone black",0.30}, medium=0.08}
glowm = mask(function(x,y) return smoothstep(846, 902, y) * (1 - smoothstep(926, 1015, y)) end)
for p=1,2 do
  work(glowm, {hand="body", tool="filbert 20", pile=warmGlow, coverage=1.3, angle=0.0,
               length={80,220}, clip=true, fill=true, edge="found", seed=1460+p})
end
print("bark closed, sky strips down, far-shore light on the water")

--@ chunk 556
-- put the flood back where that bar of light fell
coversh = mask(function(x,y) return smoothstep(846, 906, y) * (1 - smoothstep(1000, 1060, y)) end)
print(string.format("cover 850 %.2f 920 %.2f 1000 %.2f 1050 %.2f",
  coversh:at(400,850), coversh:at(400,920), coversh:at(400,1000), coversh:at(400,1050)))
for p=1,2 do
  work(coversh, {hand="broad", tool="filbert 26", pile=watS, coverage=2.0, angle=0.0,
                 length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                 pressure={0.75,1.0}, seed=1470+p})
end
for p=1,2 do
  work(wdrkSafe, {hand="broad", tool="filbert 26", pile=wdeepS, coverage=1.6, angle=0.0,
                  length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                  pressure={0.75,1.0}, seed=1480+p})
end
for p=1,2 do
  work(coversh, {hand="body", tool="filbert 6", pile=ripP2, angle=0.0, length={40,150},
                 coverage=0.8, clip=true, fill=false, edge="found", broken=0.5, seed=1490+p})
  work(coversh, {hand="body", tool="filbert 5", pile=ripD2, angle=0.0, length={32,120},
                 coverage=0.6, clip=true, fill=false, edge="found", broken=0.45, seed=1495+p})
  work(wdrkSafe, {hand="body", tool="filbert 6", pile=ripP2, angle=0.0, length={40,150},
                  coverage=0.7, clip=true, fill=false, edge="found", broken=0.5, seed=1500+p})
end
stipple(qM, {pile=rMp2, width=5.2, coverage=6.0, drag={13, math.pi/2}, feather=0.25, clip=qM, seed=1510})
stipple(qN, {pile=rNp2, width=6.8, coverage=7.0, drag={18, math.pi/2}, feather=0.25, clip=qN, seed=1511})
print("flood restored")

--@ chunk 557
print(string.format("banky(80)=%.0f  bankm(80,1240)=%.2f  wdrkSafe(80,1240)=%.2f  coversh(80,1240)=%.2f",
  banky(80), bankm:at(80,1240), wdrkSafe:at(80,1240), coversh:at(80,1240)))
print(string.format("bankm(80,1150)=%.2f bankm(160,1240)=%.2f  wdrkSafe(160,1240)=%.2f",
  bankm:at(80,1150), bankm:at(160,1240), wdrkSafe:at(160,1240)))
print(string.format("rip zone: coversh(80,1000)=%.2f  wdrkSafe(80,1100)=%.2f", coversh:at(80,1000), wdrkSafe:at(80,1100)))

--@ chunk 558
-- the deep flood: put it back, then ripple it with a pile that is only a shade off it
coverdeep = mask(function(x,y) return smoothstep(1055, 1125, y) * (1 - smoothstep(1290, 1375, y)) end) * (-bankm)
print(string.format("coverdeep 1060 %.2f 1150 %.2f 1250 %.2f 1340 %.2f",
  coverdeep:at(400,1060), coverdeep:at(400,1150), coverdeep:at(400,1250), coverdeep:at(400,1340)))
for p=1,2 do
  work(coverdeep, {hand="broad", tool="filbert 26", pile=wdeepS, coverage=2.0, angle=0.0,
                   length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                   pressure={0.75,1.0}, seed=1520+p})
end
ripP3 = pile{{"lead white",1.75},{"pale smalt",2.4},{"bone black",1.90},{"green earth",0.25}, medium=0.05}
ripD3 = pile{{"lead white",1.35},{"pale smalt",2.2},{"bone black",2.60},{"green earth",0.25}, medium=0.05}
for p=1,2 do
  work(coverdeep, {hand="body", tool="filbert 6", pile=ripP3, angle=0.0, length={40,150},
                   coverage=0.7, clip=true, fill=false, edge="found", broken=0.5, seed=1530+p})
end
for p=1,2 do
  work(coverdeep, {hand="body", tool="filbert 5", pile=ripD3, angle=0.0, length={32,120},
                   coverage=0.6, clip=true, fill=false, edge="found", broken=0.45, seed=1540+p})
end

-- the near birches: solid dark body, then a narrow bark edge
for p=1,2 do
  work(birches, {hand="body", tool="filbert 18", pile=birchD, coverage=2.0, edge="found",
                 angle=math.pi/2, length={70,170}, clip=true, fill=true,
                 pressure={0.85,1.0}, seed=1550+p})
end
for p=1,2 do
  work(birches, {hand="body", tool="filbert 16", pile=birchD, coverage=2.2, edge="found",
                 angle=math.pi/2, length={80,190}, clip=true, fill=false,
                 pressure={0.85,1.0}, seed=1560+p})
end
bs = brush("filbert", 4)
bs:load(barkP, 0.55)
bs:stroke({{48,1400},{62,1050},{72,700},{86,-20}}, {pressure={0.5,0.48,0.5,0.5}, ramps={0.3,0.35}})
bs:stroke({{190,1400},{200,1050},{206,700},{216,-20}}, {pressure={0.5,0.48,0.5,0.5}, ramps={0.3,0.35}})
sc = brush("filbert", 2.6)
for i=1,#SC do
  local x, y = SC[i][1], SC[i][2]
  sc:load(scarP, 0.6)
  sc:stroke({{x + 6, y}, {x + 13, y - rand(0,3)}}, {pressure={0.5, 0.3}})
end
print("deep flood calmed, birches dark again")

--@ chunk 559
fx, fy = 600, banky(600) + 6
for p=1,3 do
  work(fig4, {hand="body", tool="filbert 4", pile=figD2, coverage=2.2, edge="found",
              length={6,18}, clip=true, fill=true, seed=1570+p})
end
fb4 = brush("filbert", 3.0)
fb4:load(figSoft, 0.45)
fb4:stroke({{fx+5, fy-3}, {fx+5, fy-18}, {fx+4, fy-33}, {fx+4, fy-46}}, {pressure={0.3,0.32,0.3,0.3}, ramps={0.25,0.3}})
fb4:load(shawlP, 0.55)
fb4:stroke({{fx-7, fy-52}, {fx, fy-56}, {fx+6, fy-51}}, {pressure={0.4,0.46,0.4}, ramps={0.25,0.25}})
fb4:load(pile{{"lead white",2.4},{"yellow ochre",0.9},{"pale smalt",0.5}, medium=0.05}, 0.5)
fb4:stroke({{fx+2, fy-68}, {fx+3, fy-63}}, {pressure={0.42, 0.28}})

-- the lip of the bank under her, so she stands on something
lipm2 = mask(function(x,y) local e = banky(x); return smoothstep(e - 26, e + 10, y) * (1 - smoothstep(e + 22, e + 60, y)) end)
work(lipm2, {hand="body", tool="filbert 9", pile=lipP3, coverage=1.5, angle=0.0,
             length={40,120}, clip=true, fill=true, edge="found", seed=1580})
gb = brush("filbert", 2.4)
for i=1,#TUFT do
  local x, y = TUFT[i][1], TUFT[i][2]
  gb:load(grassP, 0.7)
  for k=1,4 do
    gb:stroke({{x + rand(-9,9), y + 2}, {x + rand(-13,13), y - rand(7,15)}}, {pressure={0.5,0.2}})
  end
end
print("figure back on the bank")

--@ chunk 560
wnearDark = pile{{"lead white",1.1},{"pale smalt",2.3},{"bone black",2.9},{"green earth",0.30}, medium=0.05}
print(wnearDark)
work(coverdeep, {hand="broad", tool="filbert 26", pile=wnearDark, coverage=1.5, angle=0.0,
                 length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                 pressure={0.75,1.0}, seed=1590})
ripP4 = pile{{"lead white",1.5},{"pale smalt",2.3},{"bone black",2.40},{"green earth",0.25}, medium=0.05}
for p=1,2 do
  work(coverdeep, {hand="body", tool="filbert 6", pile=ripP4, angle=0.0, length={40,150},
                   coverage=0.7, clip=true, fill=false, edge="found", broken=0.5, seed=1595+p})
end
work(tl, {hand="body", tool="filbert 13", pile=treeD, coverage=1.3, edge="found",
          length={30,80}, clip=true, fill=true, pressure={0.8,1.0}, seed=1600})
lipP4 = pile{{"lead white",1.2},{"raw umber",3.0},{"green earth",0.9},{"pale smalt",0.9},{"bone black",1.0}, medium=0.06}
work(lipm2, {hand="body", tool="filbert 9", pile=lipP4, coverage=1.7, angle=0.0,
             length={40,120}, clip=true, fill=true, edge="found", seed=1601})
print("last adjustments")

--@ chunk 561
fx, fy = 600, banky(600) + 6
-- a dark pool where she meets the water, so she is anchored
poolm = ellipse(fx+1, fy - 2, 13, 4.6)
poolP = pile{{"lead white",0.4},{"raw umber",2.2},{"bone black",3.4}, medium=0.05}
work(poolm, {hand="body", tool="filbert 9", pile=poolP, coverage=2.0, angle=0.0,
             length={20,60}, clip=true, fill=true, edge="found", seed=1610})
for p=1,3 do
  work(fig4, {hand="body", tool="filbert 4", pile=figD2, coverage=2.2, edge="found",
              length={6,18}, clip=true, fill=true, seed=1611+p})
end
fb4 = brush("filbert", 3.0)
fb4:load(figSoft, 0.45)
fb4:stroke({{fx+5, fy-3}, {fx+5, fy-18}, {fx+4, fy-33}, {fx+4, fy-46}}, {pressure={0.3,0.32,0.3,0.3}, ramps={0.25,0.3}})
fb4:load(shawlP, 0.55)
fb4:stroke({{fx-7, fy-52}, {fx, fy-56}, {fx+6, fy-51}}, {pressure={0.4,0.46,0.4}, ramps={0.25,0.25}})
fb4:load(pile{{"lead white",2.4},{"yellow ochre",0.9},{"pale smalt",0.5}, medium=0.05}, 0.5)
fb4:stroke({{fx+2, fy-68}, {fx+3, fy-63}}, {pressure={0.42, 0.28}})
print("figure anchored")

--@ chunk 562
-- the reflections of the near birches, long and broken, a shade darker than the water
nb = nil
for i=0,9 do
  local t = i/9
  local x1 = 54 + 18*t + 6*math.sin(t*3.0)
  local x2 = 178 + 16*t - 5*math.sin(t*2.6)
  local y = 826 + 250*t
  local r1 = ribbon({{x1, y}, {x1+4, y+60}}, {26 - 8*t, 16 - 5*t})
  local r2 = ribbon({{x2, y}, {x2-3, y+52}}, {17 - 6*t, 10 - 3*t})
  nb = (nb or rect(0,0,0,0)) + r1 + r2
end
nb = nb:blur(5)
print(string.format("near birch reflections %.0f sq units", nb:area()))
nbrP = pile{{"lead white",0.9},{"pale smalt",2.0},{"bone black",2.6},{"raw umber",0.8}, medium=0.05}
stipple(nb, {pile=nbrP, width=7, coverage=4.0, drag={20, math.pi/2}, feather=0.3, clip=nb, seed=1620})
-- and the flood's own ripple across them
for p=1,2 do
  work(nb, {hand="body", tool="filbert 7", pile=ripP4, angle=0.0, length={45,165},
            coverage=0.7, clip=true, fill=false, edge="found", broken=0.5, seed=1625+p})
end

-- the figure's head was catching a shine
fh2 = brush("filbert", 2.2)
fh2:load(figSoft, 0.55)
fh2:stroke({{fx+1.5, fy-70}, {fx+2.5, fy-63}}, {pressure={0.3, 0.2}})
print("near reflections, head")

--@ chunk 563
print(wait(5*24*60))
for _,p in ipairs({{500,300},{400,700},{500,950},{300,1250},{600,1180}}) do
  print(string.format("%d,%d %s", p[1], p[2], drying(p[1], p[2])))
end

--@ chunk 564
print(wait(6*24*60))
for _,p in ipairs({{500,300},{400,700},{500,950},{300,1250},{600,1180},{100,600}}) do
  print(string.format("%d,%d %s", p[1], p[2], drying(p[1], p[2])))
end

--@ chunk 565
wseal = mask(function(x,y) return smoothstep(846, 896, y) * (1 - smoothstep(1160, 1290, y)) end)
sealP = pile{{"lead white",1.6},{"pale smalt",2.4},{"bone black",2.0},{"green earth",0.30}, medium=0.05}
print(sealP)
work(wseal, {hand="scumble", tool="filbert 9", pile=sealP, coverage=1.3, angle=0.0,
              length={24,90}, clip=true, fill=true, pressure={0.5,0.8}, seed=1630})
stipple(wseal, {pile=sealP, width=6, coverage=function(x,y) return 1.2*(1 - smoothstep(1100, 1180, y)) end,
                cluster={0.35, 30}, clip=wseal, seed=1631})
print("flood sealed")

--@ chunk 566
wzone = mask(function(x,y) return smoothstep(846, 898, y) * (1 - smoothstep(1150, 1295, y)) end)
print(string.format("wzone 850 %.2f 950 %.2f 1100 %.2f 1200 %.2f",
  wzone:at(400,850), wzone:at(400,950), wzone:at(400,1100), wzone:at(400,1200)))
work(coverdeep, {hand="broad", tool="filbert 26", pile=wdeepS, coverage=1.6, angle=0.0,
                 length={180,340}, dips={4,0.9,0.3}, clip=true, fill=true,
                 pressure={0.75,1.0}, seed=1640})
for p=1,2 do
  work(wzone, {hand="body", tool="filbert 6", pile=ripP2, angle=0.0, length={40,150},
               coverage=0.75, clip=true, fill=false, edge="found", broken=0.5, seed=1650+p})
  work(wzone, {hand="body", tool="filbert 5", pile=ripD2, angle=0.0, length={32,120},
               coverage=0.55, clip=true, fill=false, edge="found", broken=0.45, seed=1660+p})
end
for p=1,2 do
  work(coverdeep, {hand="body", tool="filbert 6", pile=ripP4, angle=0.0, length={40,150},
                   coverage=0.65, clip=true, fill=false, edge="found", broken=0.5, seed=1670+p})
  work(coverdeep, {hand="body", tool="filbert 5", pile=ripD3, angle=0.0, length={32,120},
                   coverage=0.55, clip=true, fill=false, edge="found", broken=0.45, seed=1675+p})
end
local SWL = {
 {898, swellD, 0.42}, {966, swellL, 0.40}, {1034, swellD, 0.40},
 {1104, swellL, 0.42}, {1178, swellD, 0.40}, {1252, swellL, 0.40}}
sw = brush("filbert", 26)
for i=1,#SWL do
  local e = SWL[i]
  sw:load(e[2], 0.35)
  sw:stroke({{238, e[1]}, {430, e[1]-7}, {640, e[1]+4}, {850, e[1]-4}, {986, e[1]}},
            {pressure={e[3]*0.8, e[3]*0.65, e[3]*0.8, e[3]*0.65, e[3]*0.8}, ramps={0.4,0.45}})
end
print("flood re-rippled")

--@ chunk 567
stipple(qM, {pile=rMp2, width=5.2, coverage=6.0, drag={13, math.pi/2}, feather=0.25, clip=qM, seed=1680})
stipple(qN, {pile=rNp2, width=6.8, coverage=7.0, drag={18, math.pi/2}, feather=0.25, clip=qN, seed=1681})
stipple(nb, {pile=nbrP, width=7, coverage=4.0, drag={20, math.pi/2}, feather=0.3, clip=nb, seed=1682})

local DEB2 = {
 {{{300,986},{362,977},{412,981}}, 0.55},
 {{{520,1052},{602,1043},{670,1049}}, 0.6},
 {{{700,942},{762,935},{814,941}}, 0.5},
 {{{340,1128},{422,1119},{488,1125}}, 0.6},
 {{{812,1112},{890,1102},{946,1107}}, 0.55}}
db = brush("filbert", 3.4)
for i=1,#DEB2 do
  local e = DEB2[i]
  db:load(debP, 0.75)
  db:stroke(e[1], {pressure={e[2], e[2]*0.7, e[2]*0.85}, ramps={0.2,0.25}})
end
rd = brush("filbert", 2.2)
for i=1,30 do
  local x = 846 + rand(-26, 34)
  local y = 1168 + rand(-10, 8)
  local h = rand(34, 96)
  local lean = rand(-14, 14)
  rd:load(reedP, 0.7)
  rd:stroke({{x, y}, {x + lean*0.4, y - h*0.6}, {x + lean, y - h}},
            {pressure={0.5, 0.35, 0.12}})
end

work(lipm2, {hand="body", tool="filbert 9", pile=lipP3, coverage=1.8, angle=0.0,
             length={40,120}, clip=true, fill=true, edge="found", seed=1683})
work(logm2, {hand="body", tool="filbert 9", pile=logP3, coverage=2.4, angle=0.0,
             length={40,110}, clip=true, fill=true, edge="found", seed=1684})
work(logtop, {hand="body", tool="filbert 6", pile=logTopP2, coverage=1.5, angle=0.0,
              length={40,110}, clip=true, fill=true, edge="found", seed=1685})
work(logsh, {hand="body", tool="filbert 8", pile=logShP, coverage=2.0, angle=0.0,
             length={40,110}, clip=true, fill=true, edge="found", seed=1686})
fx, fy = 600, banky(600) + 6
work(poolm, {hand="body", tool="filbert 9", pile=poolP, coverage=2.0, angle=0.0,
             length={20,60}, clip=true, fill=true, edge="found", seed=1687})
for p=1,3 do
  work(fig4, {hand="body", tool="filbert 4", pile=figD2, coverage=2.2, edge="found",
              length={6,18}, clip=true, fill=true, seed=1688+p})
end
fb4 = brush("filbert", 3.0)
fb4:load(figSoft, 0.45)
fb4:stroke({{fx+5, fy-3}, {fx+5, fy-18}, {fx+4, fy-33}, {fx+4, fy-46}}, {pressure={0.3,0.32,0.3,0.3}, ramps={0.25,0.3}})
fb4:load(shawlP, 0.55)
fb4:stroke({{fx-7, fy-52}, {fx, fy-56}, {fx+6, fy-51}}, {pressure={0.4,0.46,0.4}, ramps={0.25,0.25}})
local TUFT2 = {{300,1247},{392,1236},{470,1244},{676,1232},{742,1240},{820,1236},{236,1262},{880,1244}}
gb = brush("filbert", 2.4)
for i=1,#TUFT2 do
  local x, y = TUFT2[i][1], TUFT2[i][2]
  gb:load(grassP, 0.7)
  for k=1,4 do
    gb:stroke({{x + rand(-9,9), y + 2}, {x + rand(-13,13), y - rand(7,15)}}, {pressure={0.5,0.2}})
  end
end
print("reflections, reeds, bank, log, figure")

--@ chunk 568
for p=1,2 do
  work(woodF2, {hand="body", tool="filbert 4", pile=wfarP2, coverage=1.1, edge="found",
                angle=math.pi/2, length={20,60}, clip=true, fill=true, seed=1690+p})
end
for p=1,2 do
  work(woodM2, {hand="body", tool="filbert 7", pile=wmidP2, coverage=1.1, edge="found",
                angle=math.pi/2, length={24,70}, clip=true, fill=true, seed=1695+p})
end
for p=1,2 do
  work(woodN2, {hand="body", tool="filbert 11", pile=wnearP2, coverage=1.1, edge="found",
                angle=math.pi/2, length={26,80}, clip=true, fill=true, seed=1700+p})
end
print("wood sealed")

--@ chunk 569
wfarP3 = pile{{"lead white",1.0},{"pale smalt",1.4},{"raw umber",1.8},{"bone black",1.5}, medium=0.05}
print(wfarP3)
for p=1,3 do
  work(woodF2, {hand="body", tool="filbert 3", pile=wfarP3, coverage=1.7, edge="found",
                angle=math.pi/2, length={18,54}, clip=true, fill=true, seed=1710+p})
end
-- and the near birches
for p=1,2 do
  work(birches, {hand="body", tool="filbert 16", pile=birchD, coverage=0.9, edge="found",
                 angle=math.pi/2, length={80,190}, clip=true, fill=true,
                 pressure={0.85,1.0}, seed=1720+p})
end
print("far rank and near birches")

--@ chunk 570
print(wait(4*24*60))
for _,p in ipairs({{500,300},{400,700},{500,900},{500,1000},{300,1250},{600,1180},{100,600}}) do
  print(string.format("%d,%d %s", p[1], p[2], drying(p[1], p[2])))
end

--@ chunk 571
for p=1,2 do
  work(birches, {hand="body", tool="filbert 16", pile=birchD, coverage=0.9, edge="found",
                 angle=math.pi/2, length={80,190}, clip=true, fill=true,
                 pressure={0.9,1.0}, seed=1730+p})
end
-- one clean bark edge down each, and only a few scars left
bs = brush("filbert", 5)
bs:load(barkP, 0.5)
bs:stroke({{52,1400},{66,1050},{76,700},{90,-20}}, {pressure={0.42,0.4,0.42,0.42}, ramps={0.3,0.35}})
bs:stroke({{194,1400},{204,1050},{210,700},{220,-20}}, {pressure={0.42,0.4,0.42,0.42}, ramps={0.3,0.35}})
sc = brush("filbert", 2.4)
for k=0,7 do
  local x, y = 56, 180 + k*130 + rand(-14, 14)
  sc:load(scarP, 0.55)
  sc:stroke({{x, y}, {x + rand(5, 9), y - rand(0, 3)}}, {pressure={0.45, 0.26}})
end
for k=0,5 do
  local x, y = 196, 260 + k*140 + rand(-14, 14)
  sc:load(scarP, 0.55)
  sc:stroke({{x, y}, {x + rand(4, 8), y - rand(0, 3)}}, {pressure={0.45, 0.26}})
end
print("bark settled")
