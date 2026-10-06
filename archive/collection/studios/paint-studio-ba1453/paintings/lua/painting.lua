-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box tonn
--@ engine 2

--@ chunk 1
canvas{size=762, aspect=4/3, linen={26, 22}, seed=11,
        ground={{pile={{"lead white", 7}, {"raw umber", 3}, {"burnt sienna", 1}},
                 um=140, apply="knife", texture=0.45},
                {pile={{"lead white", 8}, {"raw umber", 2}, {"green earth", 1.4}},
                 um=70, apply="knife", texture=0.30}}}
print(W, H)

--@ chunk 2
h = pencil("2B")
ch = chalk()

-- table's back edge
h:line({{-10, 508}, {170, 501}, {420, 498}, {700, 494}, {1010, 488}}, {pressure=0.30})

-- the jug: lip, neck, shoulder, body, base
h:line({{196, 338}, {204, 392}, {180, 452}, {169, 514}, {177, 570}, {206, 587},
         {258, 589}, {292, 579}, {303, 520}, {296, 456}, {268, 398}, {270, 336}},
        {pressure=0.42})
h:line({{196, 338}, {232, 325}, {270, 336}, {234, 349}, {196, 338}}, {pressure=0.35})

-- the bowl: rim ellipse then the outer wall down to the foot
h:line({{395, 520}, {428, 477}, {560, 462}, {692, 477}, {725, 520}, {690, 562},
         {560, 578}, {430, 562}, {395, 520}}, {pressure=0.40})
h:line({{395, 520}, {401, 563}, {450, 597}, {560, 607}, {670, 593}, {716, 553}, {725, 520}},
        {pressure=0.40})

-- quinces
h:line({{471, 492}, {488, 458}, {524, 450}, {554, 465}, {560, 500}, {538, 532},
         {497, 535}, {474, 518}, {471, 492}}, {pressure=0.38})
h:line({{572, 500}, {590, 470}, {628, 466}, {651, 490}, {645, 522}, {612, 538},
         {580, 528}, {572, 500}}, {pressure=0.34})
h:line({{364, 606}, {372, 570}, {406, 559}, {440, 570}, {453, 602}, {434, 640},
         {398, 650}, {368, 636}, {364, 606}}, {pressure=0.40})

-- the rumpled cloth at the right
h:line({{789, 570}, {842, 546}, {897, 561}, {955, 543}, {996, 566}, {1002, 640},
         {975, 686}, {912, 700}, {845, 688}, {800, 662}, {769, 618}, {789, 570}},
        {pressure=0.34})
h:line({{812, 588}, {846, 612}, {884, 618}, {920, 604}}, {pressure=0.26})
h:line({{826, 640}, {872, 652}, {916, 648}}, {pressure=0.24})

-- a quince left on the cloth's edge
h:line({{704, 600}, {714, 570}, {748, 562}, {780, 578}, {784, 610}, {760, 636},
         {722, 636}, {704, 600}}, {pressure=0.34})

print("drawn")

--@ chunk 3
h2 = pencil("4B")

-- table's back edge
h2:line({{-10, 508}, {170, 501}, {420, 498}, {700, 494}, {1010, 488}}, {pressure=0.55})

-- the jug
h2:line({{196, 338}, {204, 392}, {180, 452}, {169, 514}, {177, 570}, {206, 587},
          {258, 589}, {292, 579}, {303, 520}, {296, 456}, {268, 398}, {270, 336}},
         {pressure=0.65})
h2:line({{196, 338}, {232, 325}, {270, 336}, {234, 349}, {196, 338}}, {pressure=0.5})

-- the bowl
h2:line({{395, 520}, {428, 477}, {560, 462}, {692, 477}, {725, 520}, {690, 562},
          {560, 578}, {430, 562}, {395, 520}}, {pressure=0.6})
h2:line({{395, 520}, {401, 563}, {450, 597}, {560, 607}, {670, 593}, {716, 553}, {725, 520}},
         {pressure=0.6})

-- quinces
h2:line({{471, 492}, {488, 458}, {524, 450}, {554, 465}, {560, 500}, {538, 532},
          {497, 535}, {474, 518}, {471, 492}}, {pressure=0.6})
h2:line({{572, 500}, {590, 470}, {628, 466}, {651, 490}, {645, 522}, {612, 538},
          {580, 528}, {572, 500}}, {pressure=0.5})
h2:line({{364, 606}, {372, 570}, {406, 559}, {440, 570}, {453, 602}, {434, 640},
          {398, 650}, {368, 636}, {364, 606}}, {pressure=0.6})

-- the cloth
h2:line({{789, 570}, {842, 546}, {897, 561}, {955, 543}, {996, 566}, {1002, 640},
          {975, 686}, {912, 700}, {845, 688}, {800, 662}, {769, 618}, {789, 570}},
         {pressure=0.5})
h2:line({{812, 588}, {846, 612}, {884, 618}, {920, 604}}, {pressure=0.38})
h2:line({{826, 640}, {872, 652}, {916, 648}}, {pressure=0.34})

-- the quince at the cloth's edge
h2:line({{704, 600}, {714, 570}, {748, 562}, {780, 578}, {784, 610}, {760, 636},
          {722, 636}, {704, 600}}, {pressure=0.55})
print("drawn heavier")

--@ chunk 4
-- wipe the first drawing out where I want to move things
erase({{-10, 508}, {170, 501}, {420, 498}, {700, 494}, {1010, 488}}, {strength=0.95, width=10})
erase({{196, 338}, {204, 392}, {180, 452}, {169, 514}, {177, 570}, {206, 587},
        {258, 589}, {292, 579}, {303, 520}, {296, 456}, {268, 398}, {270, 336}},
       {strength=0.95, width=10})
erase({{196, 338}, {232, 325}, {270, 336}, {234, 349}, {196, 338}}, {strength=0.95, width=8})
erase({{789, 570}, {842, 546}, {897, 561}, {955, 543}, {996, 566}, {1002, 640},
        {975, 686}, {912, 700}, {845, 688}, {800, 662}, {769, 618}, {789, 570}},
       {strength=0.9, width=12})
erase({{364, 606}, {372, 570}, {406, 559}, {440, 570}, {453, 602}, {434, 640},
        {398, 650}, {368, 636}, {364, 606}}, {strength=0.9, width=10})
erase({{471, 492}, {488, 458}, {524, 450}, {554, 465}, {560, 500}, {538, 532},
        {497, 535}, {474, 518}, {471, 492}}, {strength=0.9, width=10})
erase({{572, 500}, {590, 470}, {628, 466}, {651, 490}, {645, 522}, {612, 538},
        {580, 528}, {572, 500}}, {strength=0.9, width=10})
erase({{812, 588}, {846, 612}, {884, 618}, {920, 604}}, {strength=0.9, width=8})
erase({{826, 640}, {872, 652}, {916, 648}}, {strength=0.9, width=8})
erase({{704, 600}, {714, 570}, {748, 562}, {780, 578}, {784, 610}, {760, 636},
        {722, 636}, {704, 600}}, {strength=0.9, width=10})
print("wiped")

--@ chunk 5
h3 = pencil("4B")

-- the table's back edge, tilting up to the right
h3:line({{-10, 480}, {200, 475}, {480, 471}, {740, 466}, {1010, 460}}, {pressure=0.55})

-- the jug: lip, long neck, high shoulder, heavy base
h3:line({{206, 302}, {216, 352}, {196, 410}, {176, 470}, {180, 528}, {200, 549},
          {258, 551}, {282, 534}, {292, 472}, {282, 412}, {266, 352}, {272, 300}},
         {pressure=0.62})
h3:line({{206, 302}, {238, 291}, {272, 300}, {240, 312}, {206, 302}}, {pressure=0.45})

-- the bowl: an open rim, then a deep wall falling to a foot
h3:line({{367, 492}, {392, 455}, {450, 436}, {525, 432}, {600, 436}, {658, 455},
          {683, 492}, {658, 530}, {600, 549}, {525, 553}, {450, 549}, {392, 530},
          {367, 492}}, {pressure=0.6})
h3:line({{367, 492}, {370, 532}, {392, 574}, {430, 602}, {525, 617}, {620, 607},
          {668, 580}, {681, 540}, {683, 492}}, {pressure=0.6})

-- quinces nested in the bowl
h3:line({{424, 470}, {440, 436}, {474, 424}, {504, 440}, {512, 474}, {492, 508},
          {452, 512}, {428, 496}, {424, 470}}, {pressure=0.6})
h3:line({{545, 478}, {562, 448}, {596, 442}, {622, 464}, {618, 496}, {586, 514},
          {553, 506}, {545, 478}}, {pressure=0.5})

-- the quince out front, in the light
h3:line({{305, 646}, {318, 610}, {352, 594}, {386, 608}, {402, 644}, {384, 682},
          {344, 696}, {310, 678}, {305, 646}}, {pressure=0.62})

-- the folded cloth
h3:line({{745, 562}, {812, 545}, {900, 556}, {1010, 546}, {1014, 638}, {930, 678},
          {856, 676}, {800, 650}, {756, 620}, {745, 562}}, {pressure=0.52})
h3:line({{790, 590}, {830, 628}, {862, 660}}, {pressure=0.38})
h3:line({{858, 556}, {872, 600}, {886, 640}}, {pressure=0.34})
h3:line({{776, 612}, {812, 596}, {846, 578}}, {pressure=0.3})

-- a quince dropped at the cloth's edge
h3:line({{674, 596}, {686, 562}, {720, 552}, {752, 570}, {758, 602}, {734, 630},
          {696, 630}, {674, 596}}, {pressure=0.58})
print("redrawn")

--@ chunk 6
-- clear the old jug and cloth, and a stray ghost circle
erase({{206, 302}, {216, 352}, {196, 410}, {176, 470}, {180, 528}, {200, 549},
        {258, 551}, {282, 534}, {292, 472}, {282, 412}, {266, 352}, {272, 300}},
       {strength=1.0, width=12})
erase({{206, 302}, {238, 291}, {272, 300}, {240, 312}, {206, 302}}, {strength=1.0, width=10})
erase({{745, 562}, {812, 545}, {900, 556}, {1010, 546}, {1014, 638}, {930, 678},
        {856, 676}, {800, 650}, {756, 620}, {745, 562}}, {strength=1.0, width=14})
erase({{790, 590}, {830, 628}, {862, 660}}, {strength=1.0, width=9})
erase({{858, 556}, {872, 600}, {886, 640}}, {strength=1.0, width=9})
erase({{674, 596}, {686, 562}, {720, 552}, {752, 570}, {758, 602}, {734, 630},
        {696, 630}, {674, 596}}, {strength=1.0, width=12})
erase({{364, 606}, {372, 570}, {406, 559}, {440, 570}, {453, 602}, {434, 640},
        {398, 650}, {368, 636}, {364, 606}}, {strength=1.0, width=14})
print("cleared")

--@ chunk 7
h4 = pencil("4B")

-- the jug again, taller: a long-necked bottle
h4:line({{210, 248}, {219, 318}, {201, 388}, {178, 460}, {181, 524}, {201, 549},
          {258, 551}, {281, 532}, {291, 468}, {282, 392}, {267, 320}, {272, 246}},
         {pressure=0.62})
h4:line({{210, 248}, {240, 238}, {272, 246}, {241, 258}, {210, 248}}, {pressure=0.45})

-- the folded cloth, smaller and further right, with straight creases
h4:line({{812, 556}, {880, 548}, {950, 556}, {1012, 550}, {1012, 652}, {950, 668},
          {880, 672}, {830, 660}, {800, 632}, {790, 596}, {812, 556}}, {pressure=0.55})
h4:line({{826, 594}, {856, 620}, {884, 654}}, {pressure=0.4})
h4:line({{898, 554}, {911, 600}, {924, 646}}, {pressure=0.36})
h4:line({{848, 662}, {900, 648}, {952, 646}}, {pressure=0.3})

-- the quince dropped beside the cloth
h4:line({{686, 600}, {700, 560}, {738, 548}, {776, 570}, {782, 606}, {754, 638},
          {710, 636}, {686, 600}}, {pressure=0.6})
print("final drawing")

--@ chunk 8
erase({{395, 520}, {428, 477}, {560, 462}, {692, 477}, {725, 520}, {690, 562},
         {560, 578}, {430, 562}, {395, 520}}, {strength=1.0, width=14})
erase({{395, 520}, {401, 563}, {450, 597}, {560, 607}, {670, 593}, {716, 553}, {725, 520}},
       {strength=1.0, width=14})
erase({{196, 338}, {204, 392}, {180, 452}, {169, 514}, {177, 570}, {206, 587},
        {258, 589}, {292, 579}, {303, 520}, {296, 456}, {268, 398}, {270, 336}},
       {strength=1.0, width=14})
erase({{745, 562}, {812, 545}, {900, 556}, {1010, 546}, {1014, 638}, {930, 678},
        {856, 676}, {800, 650}, {756, 620}, {745, 562}}, {strength=1.0, width=16})
erase({{812, 588}, {846, 612}, {884, 618}, {920, 604}}, {strength=1.0, width=10})
erase({{826, 640}, {872, 652}, {916, 648}}, {strength=1.0, width=10})
erase({{364, 606}, {372, 570}, {406, 559}, {440, 570}, {453, 602}, {434, 640},
        {398, 650}, {368, 636}, {364, 606}}, {strength=1.0, width=16})
erase({{674, 596}, {686, 562}, {720, 552}, {752, 570}, {758, 602}, {734, 630},
        {696, 630}, {674, 596}}, {strength=1.0, width=14})
print("second wipe")

--@ chunk 9
imprim = pile{{"raw umber", 5}, {"bone black", 2.2}, {"burnt sienna", 2},
              {"ultramarine blue", 1.6}, {"lead white", 1}, medium=0.18}

w = work(everywhere(), {hand="glaze", pile=imprim, angle=-0.12, coverage=1.9,
      load_at=function(x, y) return 0.75 - 0.25 * (x / 1000) + 0.1 * math.sin(y / 90) end,
      pressure={0.42, 0.30}, seed=5})
print(w)

--@ chunk 10
base = pile{{"raw umber", 5}, {"bone black", 2.2}, {"burnt sienna", 2},
             {"ultramarine blue", 1.6}, {"lead white", 1}}
work(everywhere(), {hand="broad", pile=base, angle=-0.12, coverage=2.0, fill=true,
      pressure={0.62, 0.5}, tool="filbert 24", dips={7, 0.7, 0.5}, seed=9})

--@ chunk 11
wall_mid = pile{{"lead white", 3}, {"raw umber", 2.4}, {"ultramarine blue", 1.4},
                 {"green earth", 1.1}, {"burnt sienna", 0.6}}

wallm = above(function(x) return 477 - 0.021 * x end):soften(5)
work(wallm, {hand="broad", pile=wall_mid, coverage=1.7, fill=true, angle=-0.07,
      pressure={0.55, 0.44}, tool="filbert 24", edge="soft", seed=3})
print("wall laid")

--@ chunk 12
wall_mid = pile{{"raw umber", 4}, {"green earth", 2.2}, {"ultramarine blue", 1.8},
                 {"burnt sienna", 1.2}, {"lead white", 1.2}}
work(wallm, {hand="broad", pile=wall_mid, coverage=2.2, fill=true, angle=-0.07,
      pressure={0.30, 0.22}, tool="filbert 18", dips={6, 0.55, 0.55}, edge="soft", seed=13})
print("wall redone")

--@ chunk 13
wall_mid = pile{{"raw umber", 4.5}, {"green earth", 3.5}, {"yellow ochre", 1.0},
                 {"ultramarine blue", 0.7}, {"burnt sienna", 0.8}, {"lead white", 0.9}}
work(wallm, {hand="broad", pile=wall_mid, coverage=1.9, fill=true, angle=-0.07,
      pressure={0.50, 0.38}, tool="filbert 22", dips={5, 0.85, 0.3}, edge="soft", seed=21})
print("wall third try")

--@ chunk 14
work(wallm, {hand="broad", pile=wall_mid, coverage=2.1, fill=true, angle=0.06,
      pressure={0.46, 0.36}, tool="filbert 22", dips={3, 0.95, 0.15}, edge="soft", seed=31})
blend(wallm, {angle=0.1})
print("wall smoothed")

--@ chunk 15
tablem = below(function(x) return 477 - 0.021 * x end):soften(6)
table_dark = pile{{"raw umber", 3}, {"burnt sienna", 1.8}, {"bone black", 2.2},
                  {"ultramarine blue", 0.6}, {"lead white", 0.4}}
work(tablem, {hand="broad", pile=table_dark, coverage=1.9, fill=true, angle=-0.05,
      pressure={0.46, 0.36}, tool="filbert 22", dips={4, 0.9, 0.2}, edge="soft", seed=41})
blend(tablem, {angle=-0.08})
print("table laid")

--@ chunk 16
pool = pile{{"raw umber", 2.5}, {"burnt sienna", 1.5}, {"yellow ochre", 1.2},
            {"lead white", 1.6}, {"ultramarine blue", 0.3}}
lamp = ellipse(300, 645, 400, 165):blur(95) + ellipse(215, 575, 180, 80):blur(60)
work(lamp * tablem, {hand="broad", pile=pool, coverage=1.5, fill=true, angle=-0.04,
      pressure={0.44, 0.34}, tool="filbert 22", dips={4, 0.9, 0.25}, edge="lost", seed=57})
blend((lamp * tablem):grow(30), {angle=-0.05})
print("light pool")

--@ chunk 17
knock = pile{{"raw umber", 4}, {"burnt sienna", 2}, {"bone black", 2},
             {"ultramarine blue", 1.2}, medium=0.45}
work(tablem, {hand="glaze", pile=knock, angle=-0.05, coverage=1.15,
      pressure={0.5, 0.4}, tool="filbert 26", dips={3, 0.7, 0.3},
      clip=true, edge="found", seed=63})
print("table knocked back")

--@ chunk 18
print("table", drying(300, 650), drying(800, 600), "wall", drying(500, 200))
work(tablem, {hand="broad", pile=table_dark, coverage=1.6, fill=true, angle=-0.05,
      pressure={0.48, 0.38}, tool="filbert 22", dips={3, 0.95, 0.15}, edge="soft", seed=71})
blend(tablem, {angle=-0.06})
print("table evened")

--@ chunk 19
jugm = poly({{210, 248}, {219, 318}, {201, 388}, {178, 460}, {181, 524}, {201, 549},
              {258, 551}, {281, 532}, {291, 468}, {282, 392}, {267, 320}, {272, 246},
              {240, 238}}, true)
jug_dark = pile{{"bone black", 4}, {"green earth", 3}, {"ultramarine blue", 2.5},
                {"raw umber", 1.5}, {"lead white", 0.6}}
work(jugm, {hand="body", pile=jug_dark, coverage=1.9, fill=true, angle=1.45,
      pressure={0.55, 0.45}, tool="filbert 9", dips={5, 0.8, 0.3},
      clip=true, edge="found", seed=81})
print("jug blocked in")

--@ chunk 20
jug_dark = pile{{"bone black", 5}, {"raw umber", 3}, {"green earth", 2.5},
                {"ultramarine blue", 0.5}, {"lead white", 0.8}}
work(jugm, {hand="body", pile=jug_dark, coverage=2.6, fill=false, angle=1.45,
      pressure={0.62, 0.5}, tool="filbert 9", dips={4, 0.9, 0.25},
      clip=true, edge="found", seed=91})
print("jug repainted")

--@ chunk 21
bowlm = poly({{367, 492}, {370, 532}, {392, 574}, {430, 602}, {525, 617},
              {620, 607}, {668, 580}, {681, 540}, {683, 492}, {658, 455},
              {600, 436}, {525, 432}, {450, 436}, {392, 455}}, true)
bowl_in = ellipse(525, 494, 151, 49)

bowl_mid = pile{{"yellow ochre", 2.5}, {"burnt sienna", 2.5}, {"raw umber", 2}, {"lead white", 0.8}}
bowl_in_dark = pile{{"raw umber", 3}, {"bone black", 2.2}, {"burnt sienna", 1.2},
                    {"ultramarine blue", 0.6}}

work(bowlm, {hand="body", pile=bowl_mid, coverage=2.3, fill=false, angle=-0.25,
      pressure={0.6, 0.48}, tool="filbert 9", dips={4, 0.9, 0.25},
      clip=true, edge="found", seed=101})
work(bowl_in, {hand="body", pile=bowl_in_dark, coverage=2.0, fill=false, angle=-0.25,
      pressure={0.55, 0.45}, tool="filbert 9", dips={4, 0.9, 0.25},
      clip=true, edge="found", seed=103})
print("bowl and its inside")

--@ chunk 22
bowl_mid = pile{{"yellow ochre", 2}, {"burnt sienna", 1.5}, {"raw umber", 3},
                 {"lead white", 1}, {"ultramarine blue", 0.3}}
work(bowlm - bowl_in, {hand="body", pile=bowl_mid, coverage=2.4, fill=false, angle=-0.3,
      pressure={0.6, 0.48}, tool="filbert 9", dips={4, 0.9, 0.25},
      clip=true, edge="found", seed=107})
blend(bowlm, {angle=-0.35})
print("bowl muted and smoothed")

--@ chunk 23
print("bowl", drying(450, 540), drying(600, 470))
bowl_lit = pile{{"yellow ochre", 2.5}, {"lead white", 2.5}, {"raw umber", 1.5},
                {"burnt sienna", 1}, {"ultramarine blue", 0.3}}
bowl_dk  = pile{{"raw umber", 3}, {"burnt sienna", 1.2}, {"bone black", 2},
                {"ultramarine blue", 0.8}, {"lead white", 0.4}}
bowl_in_dark = pile{{"raw umber", 3}, {"bone black", 2.6}, {"burnt sienna", 1},
                    {"ultramarine blue", 0.8}}

litL = ellipse(415, 530, 135, 165):blur(70)
shadR = ellipse(715, 560, 150, 170):blur(75)

work(bowl_in, {hand="body", pile=bowl_in_dark, coverage=2.0, fill=false, angle=-0.3,
      pressure={0.58, 0.48}, tool="filbert 9", dips={4, 0.9, 0.25}, clip=true, edge="found", seed=109})
work(bowlm * litL, {hand="body", pile=bowl_lit, coverage=1.8, fill=false, angle=-0.55,
      pressure={0.55, 0.44}, tool="filbert 9", dips={4, 0.85, 0.3}, clip=true, edge="soft", seed=111})
work(bowlm * shadR, {hand="body", pile=bowl_dk, coverage=2.0, fill=false, angle=-0.4,
      pressure={0.58, 0.48}, tool="filbert 9", dips={4, 0.9, 0.25}, clip=true, edge="soft", seed=113})
print("bowl modelled")

--@ chunk 24
bowl_mid = pile{{"yellow ochre", 1.6}, {"burnt sienna", 1.5}, {"raw umber", 2.6},
                 {"lead white", 1.1}, {"ultramarine blue", 0.4}}
bowl_lit = pile{{"yellow ochre", 3}, {"lead white", 1.6}, {"raw umber", 2},
                {"burnt sienna", 1}, {"ultramarine blue", 0.4}}

work(bowlm, {pile=bowl_mid, hand="broad", tool="filbert 20", length={70, 165},
      coverage=2.2, fill=true, angle=-0.35, pressure={0.52, 0.42}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=121})
work(bowlm * litL, {pile=bowl_lit, hand="broad", tool="filbert 16", length={60, 140},
      coverage=1.5, fill=true, angle=-0.6, pressure={0.5, 0.4}, dips={3, 0.85, 0.25},
      clip=true, edge="soft", seed=123})
work(bowlm * shadR, {pile=bowl_dk, hand="broad", tool="filbert 18", length={60, 150},
      coverage=1.7, fill=true, angle=-0.4, pressure={0.55, 0.45}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=125})
blend(bowlm, {angle=-0.35})
print("bowl, broad strokes")

--@ chunk 25
bowl_mid = pile{{"yellow ochre", 1.2}, {"burnt sienna", 1.4}, {"raw umber", 3},
                 {"lead white", 0.7}, {"ultramarine blue", 0.5}}

work(bowlm, {pile=bowl_mid, hand="broad", tool="filbert 20", length={70, 165},
      coverage=2.0, fill=true, angle=-0.35, pressure={0.52, 0.42}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=131})
work(bowlm * litL, {pile=bowl_lit, hand="broad", tool="filbert 16", length={60, 140},
      coverage=1.5, fill=true, angle=-0.6, pressure={0.5, 0.4}, dips={3, 0.85, 0.25},
      clip=true, edge="soft", seed=133})
work(bowlm * shadR, {pile=bowl_dk, hand="broad", tool="filbert 18", length={60, 150},
      coverage=1.7, fill=true, angle=-0.4, pressure={0.55, 0.45}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=135})
blend(bowlm - bowl_in, {angle=-0.35})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 14", length={50, 110},
      coverage=2.0, fill=true, angle=-0.3, pressure={0.55, 0.45}, dips={3, 0.9, 0.2},
      clip=true, edge="found", seed=137})
print("bowl rebuilt")

--@ chunk 26
wall_dk = pile{{"raw umber", 4}, {"green earth", 2}, {"bone black", 1.6},
                {"ultramarine blue", 1}, {"burnt sienna", 1}}
work(wallm, {pile=wall_dk, hand="broad", tool="filbert 20", length={70, 170},
      coverage=1.7, fill=true, angle=-0.07, pressure={0.52, 0.42}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=141})
blend(wallm, {angle=0.05})
print("wall darkened")

--@ chunk 27
fruit_mid = pile{{"cadmium yellow", 1.8}, {"yellow ochre", 2}, {"transparent oxide yellow", 0.8},
                 {"burnt sienna", 1.6}, {"raw umber", 0.6}, {"lead white", 0.4}}
fruit_shadow = pile{{"burnt sienna", 3}, {"ultramarine blue", 1.5}, {"cadmium red", 1},
                    {"raw umber", 2}, {"lead white", 0.5}}
fruit_light = pile{{"lead white", 3}, {"cadmium yellow", 2}, {"transparent oxide yellow", 1.2},
                   {"yellow ochre", 0.5}}

function paint_fruit(m, cx, cy, r, seed)
  local sh = ellipse(cx + r * 0.5, cy + r * 0.45, r * 1.15, r * 1.15):blur(r * 0.75)
  local lt = ellipse(cx - r * 0.42, cy - r * 0.44, r * 0.62, r * 0.58):blur(r * 0.5)
  work(m, {pile=fruit_mid, hand="broad", tool="filbert 12", length={30, 75},
        coverage=1.9, fill=true, angle=-0.5, pressure={0.55, 0.45}, dips={3, 0.9, 0.2},
        clip=true, edge="soft", seed=seed})
  work(m * sh, {pile=fruit_shadow, hand="broad", tool="filbert 10", length={25, 60},
        coverage=1.4, fill=true, angle=-0.5, pressure={0.55, 0.45}, dips={3, 0.9, 0.2},
        clip=true, edge="soft", seed=seed + 1})
  work(m * lt, {pile=fruit_light, hand="broad", tool="filbert 9", length={20, 50},
        coverage=1.5, fill=true, angle=-0.6, pressure={0.5, 0.4}, dips={3, 0.85, 0.2},
        clip=true, edge="soft", seed=seed + 2})
  blend(m, {angle=-0.5})
end

q1 = poly({{425, 470}, {433, 440}, {455, 424}, {480, 426}, {500, 446}, {506, 476},
           {492, 504}, {460, 512}, {434, 498}}, true)
q2 = poly({{548, 480}, {556, 456}, {576, 444}, {600, 450}, {616, 470}, {612, 496},
           {590, 512}, {562, 506}}, true)
paint_fruit(q1, 466, 468, 42, 151)
paint_fruit(q2, 583, 478, 35, 161)
print("quinces in the bowl")

--@ chunk 28
fruit_light = pile{{"lead white", 1.6}, {"cadmium yellow", 3.4},
                   {"transparent oxide yellow", 1.2}, {"yellow ochre", 0.5}}
fruit_shadow = pile{{"burnt sienna", 3}, {"ultramarine blue", 2}, {"cadmium red", 1.2},
                    {"raw umber", 2.4}, {"lead white", 0.3}}

function paint_fruit(m, cx, cy, r, seed)
  local sh = ellipse(cx + r * 0.42, cy + r * 0.5, r * 1.1, r * 1.1):blur(r * 0.55)
  local lt = ellipse(cx - r * 0.5, cy - r * 0.52, r * 0.5, r * 0.45):blur(r * 0.4)
  work(m, {pile=fruit_mid, hand="broad", tool="filbert 10", length={25, 60},
        coverage=1.8, fill=true, angle=-0.5, pressure={0.58, 0.48}, dips={3, 0.9, 0.2},
        clip=true, edge="soft", seed=seed})
  work(m * sh, {pile=fruit_shadow, hand="broad", tool="filbert 9", length={20, 50},
        coverage=1.7, fill=true, angle=-0.5, pressure={0.58, 0.48}, dips={3, 0.9, 0.2},
        clip=true, edge="soft", seed=seed + 1})
  work(m * lt, {pile=fruit_light, hand="broad", tool="filbert 8", length={18, 45},
        coverage=1.5, fill=true, angle=-0.6, pressure={0.52, 0.42}, dips={3, 0.85, 0.2},
        clip=true, edge="soft", seed=seed + 2})
  blend(m, {angle=-0.5})
end

paint_fruit(q1, 466, 468, 42, 171)
paint_fruit(q2, 583, 478, 35, 181)
print("quinces remodelled")

--@ chunk 29
fruit_mid = pile{{"yellow ochre", 3}, {"cadmium yellow", 1}, {"burnt sienna", 1.8},
                 {"raw umber", 0.8}, {"lead white", 0.4}}
fruit_light = pile{{"lead white", 1.2}, {"cadmium yellow", 4}, {"transparent oxide yellow", 1.2},
                   {"yellow ochre", 0.6}}

function paint_fruit(m, cx, cy, r, seed)
  local sh = ellipse(cx + r * 0.5, cy + r * 0.55, r * 0.95, r * 0.95):blur(r * 0.4)
  local lt = ellipse(cx - r * 0.48, cy - r * 0.5, r * 0.52, r * 0.46):blur(r * 0.42)
  work(m, {pile=fruit_mid, hand="broad", tool="filbert 10", length={25, 60},
        coverage=1.8, fill=true, angle=-0.5, pressure={0.58, 0.48}, dips={3, 0.9, 0.2},
        clip=true, edge="soft", seed=seed})
  work(m * sh, {pile=fruit_shadow, hand="broad", tool="filbert 9", length={20, 50},
        coverage=2.2, fill=true, angle=-0.5, pressure={0.6, 0.5}, dips={3, 0.9, 0.2},
        clip=true, edge="soft", seed=seed + 1})
  work(m * lt, {pile=fruit_light, hand="broad", tool="filbert 8", length={18, 45},
        coverage=1.6, fill=true, angle=-0.6, pressure={0.52, 0.42}, dips={3, 0.85, 0.2},
        clip=true, edge="soft", seed=seed + 2})
  blend(m, {angle=-0.5})
end

paint_fruit(q1, 466, 468, 42, 191)
paint_fruit(q2, 583, 478, 35, 201)
print("quinces, third modelling")

--@ chunk 30
fruit_shadow = pile{{"ultramarine blue", 3}, {"burnt sienna", 2}, {"raw umber", 2.5},
                    {"bone black", 1}}
fruit_base = pile{{"yellow ochre", 3}, {"cadmium yellow", 1.2}, {"burnt sienna", 1.4},
                 {"raw umber", 0.6}, {"lead white", 0.5}}

function paint_fruit(m, cx, cy, r, seed)
  local sh = ellipse(cx + r * 0.4, cy + r * 0.45, r * 1.0, r * 1.0):blur(r * 0.38)
  local lt = ellipse(cx - r * 0.46, cy - r * 0.5, r * 0.5, r * 0.44):blur(r * 0.45)
  work(m, {pile=fruit_base, hand="broad", tool="filbert 12", length={35, 80},
        coverage=3.0, fill=true, angle=-0.5, pressure={0.6, 0.5}, dips={2, 0.95, 0.1},
        clip=true, edge="found", seed=seed})
  work(m * sh, {pile=fruit_shadow, hand="broad", tool="filbert 10", length={25, 60},
        coverage=2.4, fill=true, angle=-0.55, pressure={0.62, 0.52}, dips={2, 0.95, 0.1},
        clip=true, edge="soft", seed=seed + 1})
  work(m * lt, {pile=fruit_light, hand="broad", tool="filbert 9", length={20, 48},
        coverage=1.8, fill=true, angle=-0.6, pressure={0.55, 0.45}, dips={2, 0.9, 0.15},
        clip=true, edge="soft", seed=seed + 2})
end

paint_fruit(q1, 466, 468, 42, 211)
paint_fruit(q2, 583, 478, 35, 221)
print("quinces, clean coat")

--@ chunk 31
fruit_shadow = pile{{"bone black", 3}, {"burnt sienna", 3}, {"ultramarine blue", 1.5},
                    {"lead white", 0.6}}

function paint_fruit(m, cx, cy, r, seed)
  local sh = m * ellipse(cx + r * 0.62, cy + r * 0.6, r * 0.95, r * 0.95):blur(r * 0.4)
  local lt = m * ellipse(cx - r * 0.46, cy - r * 0.5, r * 0.5, r * 0.44):blur(r * 0.45)
  work(sh, {pile=fruit_shadow, hand="broad", tool="filbert 10", length={25, 60},
        coverage=2.2, fill=true, angle=-0.55, pressure={0.62, 0.52}, dips={2, 0.95, 0.1},
        clip=true, edge="soft", seed=seed})
  work(lt, {pile=fruit_light, hand="broad", tool="filbert 9", length={20, 48},
        coverage=1.7, fill=true, angle=-0.6, pressure={0.55, 0.45}, dips={2, 0.9, 0.15},
        clip=true, edge="soft", seed=seed + 1})
end

paint_fruit(q1, 466, 468, 42, 231)
paint_fruit(q2, 583, 478, 35, 241)
print("crescent shadows")

--@ chunk 32
fruit_shadow = pile{{"bone black", 4}, {"burnt sienna", 3}, {"raw umber", 2},
                    {"lead white", 0.5}}
function paint_fruit(m, cx, cy, r, seed)
  local sh = m * ellipse(cx + r * 0.62, cy + r * 0.6, r * 0.95, r * 0.95):blur(r * 0.4)
  work(sh, {pile=fruit_shadow, hand="broad", tool="filbert 10", length={25, 60},
        coverage=2.4, fill=true, angle=-0.55, pressure={0.65, 0.55}, dips={2, 0.95, 0.1},
        clip=true, edge="soft", seed=seed})
end
paint_fruit(q1, 466, 468, 42, 251)
paint_fruit(q2, 583, 478, 35, 261)
print("warm shadow")

--@ chunk 33
function paint_fruit(m, cx, cy, r, seed)
  local sh = m * ellipse(cx + r * 0.62, cy + r * 0.6, r * 0.95, r * 0.95):blur(r * 0.4)
  local lt = m * ellipse(cx - r * 0.46, cy - r * 0.5, r * 0.5, r * 0.44):blur(r * 0.45)
  work(m, {pile=fruit_base, hand="broad", tool="filbert 12", length={35, 80},
        coverage=3.2, fill=true, angle=-0.5, pressure={0.62, 0.52}, dips={2, 0.98, 0.05},
        clip=true, edge="found", seed=seed})
  work(sh, {pile=fruit_shadow, hand="broad", tool="filbert 10", length={25, 60},
        coverage=2.4, fill=true, angle=-0.55, pressure={0.65, 0.55}, dips={2, 0.95, 0.1},
        clip=true, edge="soft", seed=seed + 1})
  work(lt, {pile=fruit_light, hand="broad", tool="filbert 9", length={20, 48},
        coverage=1.7, fill=true, angle=-0.6, pressure={0.55, 0.45}, dips={2, 0.9, 0.15},
        clip=true, edge="soft", seed=seed + 2})
end
paint_fruit(q1, 466, 468, 42, 271)
paint_fruit(q2, 583, 478, 35, 281)
print("full repaint of the quinces")

--@ chunk 34
q3 = poly({{306, 648}, {314, 612}, {340, 592}, {372, 596}, {394, 620}, {394, 654},
           {374, 686}, {338, 694}, {312, 676}}, true)
q4 = poly({{688, 596}, {698, 566}, {722, 552}, {752, 560}, {770, 584}, {766, 612},
           {744, 632}, {712, 630}, {692, 614}}, true)

cast = pile{{"bone black", 3}, {"raw umber", 3}, {"ultramarine blue", 1}, {"burnt sienna", 1.5}}
work(ellipse(392, 672, 60, 17):blur(15), {pile=cast, hand="scumble", pile=cast,
      tool="filbert 14", coverage=1.1, angle=-0.1, pressure={0.6, 0.4}, seed=291})
work(ellipse(772, 614, 52, 16):blur(13), {pile=cast, hand="scumble",
      tool="filbert 14", coverage=1.1, angle=-0.1, pressure={0.6, 0.4}, seed=293})
paint_fruit(q3, 350, 643, 47, 301)
paint_fruit(q4, 730, 592, 41, 311)
print("front quinces and their shadows")

--@ chunk 35
work(ellipse(392, 672, 78, 26):blur(20), {pile=table_dark, hand="broad",
      tool="filbert 20", length={60, 140}, coverage=1.5, fill=true, angle=-0.1,
      pressure={0.55, 0.45}, dips={3, 0.9, 0.2}, seed=321})
work(ellipse(772, 614, 70, 25):blur(18), {pile=table_dark, hand="broad",
      tool="filbert 20", length={60, 140}, coverage=1.5, fill=true, angle=-0.1,
      pressure={0.55, 0.45}, dips={3, 0.9, 0.2}, seed=323})
work(ellipse(400, 676, 66, 18):blur(17), {pile=cast, hand="scumble",
      tool="filbert 16", coverage=0.8, angle=-0.1, pressure={0.55, 0.3}, seed=325})
work(ellipse(778, 616, 58, 17):blur(15), {pile=cast, hand="scumble",
      tool="filbert 16", coverage=0.8, angle=-0.1, pressure={0.55, 0.3}, seed=327})
print("shadows softened")

--@ chunk 36
clothm = poly({{812, 556}, {880, 548}, {950, 556}, {1012, 550}, {1012, 652},
                {950, 668}, {880, 672}, {830, 660}, {800, 632}, {790, 596}}, true)
cloth_mid = pile{{"lead white", 3.5}, {"raw umber", 2.5}, {"ultramarine blue", 0.8},
                 {"yellow ochre", 0.6}}
cloth_light = pile{{"lead white", 5}, {"yellow ochre", 0.8}, {"raw umber", 0.8}}
cloth_dark = pile{{"raw umber", 3}, {"ultramarine blue", 1.5}, {"lead white", 1},
                  {"burnt sienna", 0.8}}

work(clothm, {pile=cloth_mid, hand="broad", tool="filbert 18", length={60, 150},
      coverage=2.4, fill=true, angle=-0.3, pressure={0.55, 0.45}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=331})
work(clothm * ellipse(846, 588, 100, 58):blur(48), {pile=cloth_light, hand="broad",
      tool="filbert 14", length={45, 110}, coverage=1.7, fill=true, angle=-0.4,
      pressure={0.55, 0.45}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=333})
work(clothm * ellipse(958, 642, 115, 62):blur(52), {pile=cloth_dark, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.9, fill=true, angle=-0.35,
      pressure={0.58, 0.48}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=335})
blend(clothm, {angle=-0.35})
print("cloth laid")

--@ chunk 37
cloth_mid = pile{{"lead white", 2.5}, {"raw umber", 3}, {"ultramarine blue", 1.2},
                 {"yellow ochre", 0.5}}
cloth_light = pile{{"lead white", 4}, {"yellow ochre", 1}, {"raw umber", 1.2}}
cloth_dark = pile{{"raw umber", 3.5}, {"ultramarine blue", 2}, {"lead white", 0.8},
                  {"burnt sienna", 1}}

work(clothm, {pile=cloth_mid, hand="broad", tool="filbert 18", length={60, 150},
      coverage=2.6, fill=true, angle=-0.3, pressure={0.55, 0.45}, dips={3, 0.9, 0.2},
      clip=true, edge="found", seed=341})
work(clothm * ribbon({{826, 594}, {856, 620}, {884, 654}}, 6):blur(3),
      {pile=cloth_dark, hand="hatch", tool="round 4", coverage=1.6, angle=-0.9,
       pressure={0.6, 0.45}, clip=true, seed=343})
work(clothm * ribbon({{898, 554}, {911, 600}, {924, 646}}, 5):blur(3),
      {pile=cloth_dark, hand="hatch", tool="round 4", coverage=1.5, angle=-0.9,
       pressure={0.6, 0.45}, clip=true, seed=345})
work(clothm * ribbon({{800, 632}, {840, 652}, {884, 656}}, 6):blur(3),
      {pile=cloth_dark, hand="hatch", tool="round 4", coverage=1.5, angle=-0.9,
       pressure={0.6, 0.45}, clip=true, seed=347})
work(clothm * ellipse(950, 636, 120, 66):blur(50), {pile=cloth_dark, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.8, fill=true, angle=-0.35,
      pressure={0.58, 0.48}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=349})
work(clothm * ellipse(852, 572, 58, 24):blur(22), {pile=cloth_light, hand="broad",
      tool="filbert 12", length={35, 85}, coverage=1.8, fill=true, angle=-0.5,
      pressure={0.52, 0.42}, dips={3, 0.85, 0.2}, clip=true, edge="soft", seed=351})
print("cloth reworked")

--@ chunk 38
work(rect(755, 515, 300, 200):grow(20), {pile=table_dark, hand="broad",
      tool="filbert 22", length={70, 160}, coverage=1.9, fill=true, angle=-0.05,
      pressure={0.55, 0.45}, dips={3, 0.9, 0.2}, edge="soft", seed=361})
blend(rect(755, 515, 300, 200):grow(30), {angle=-0.06})
print("table wiped clean on the right")

--@ chunk 39
clothm = poly({{812, 552}, {900, 536}, {1012, 546}, {1060, 540}, {1060, 706},
                {960, 702}, {880, 690}, {820, 664}, {788, 610}}, true)
cloth = pile{{"lead white", 2}, {"raw umber", 3.5}, {"yellow ochre", 0.8},
             {"ultramarine blue", 0.5}, {"burnt sienna", 1}}
cloth_lt = pile{{"lead white", 3.6}, {"yellow ochre", 1}, {"raw umber", 1}, {"burnt sienna", 0.5}}
cloth_dk = pile{{"raw umber", 4}, {"burnt sienna", 1.5}, {"ultramarine blue", 1.2},
                {"lead white", 0.6}}

work(clothm, {pile=cloth, hand="broad", tool="filbert 18", length={60, 150},
      coverage=2.8, fill=true, angle=-0.28, pressure={0.58, 0.48}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=371})
-- the lit ridge running along the near fold
work(clothm * ribbon({{800, 610}, {860, 630}, {930, 640}, {1000, 648}}, 26):blur(10),
      {pile=cloth_lt, hand="broad", tool="filbert 14", length={45, 110},
       coverage=1.7, fill=true, angle=-0.35, pressure={0.55, 0.45}, clip=true, seed=373})
-- the deep fold beyond it
work(clothm * ribbon({{880, 556}, {930, 590}, {985, 626}, {1040, 652}}, 30):blur(14),
      {pile=cloth_dk, hand="broad", tool="filbert 16", length={50, 120},
       coverage=1.9, fill=true, angle=-0.4, pressure={0.6, 0.5}, clip=true, seed=375})
-- the far side falling into the dark
work(clothm * ellipse(980, 560, 110, 46):blur(40), {pile=cloth_dk, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.8, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=377})
blend(clothm, {angle=-0.35})
print("cloth relaid")

--@ chunk 40
-- crisp fold lines, drawn with a rigger rather than washed in
b = brush{kind="rigger", width=3.5, point=0.9}
b:load(cloth_dk, 0.9)
b:stroke({{834, 606}, {876, 616}, {918, 621}, {958, 618}, {992, 608}},
         {pressure={0.75, 0.7, 0.6, 0.5, 0.3}})
b:stroke({{868, 562}, {912, 584}, {958, 596}, {1004, 594}},
         {pressure={0.6, 0.55, 0.45, 0.25}})
b:stroke({{900, 664}, {944, 662}, {990, 654}}, {pressure={0.5, 0.4, 0.2}})

lb = brush{kind="rigger", width=4, point=0.9}
lb:load(cloth_lt, 0.85)
lb:stroke({{842, 594}, {886, 604}, {928, 608}, {968, 604}}, {pressure={0.6, 0.55, 0.45, 0.25}})
lb:stroke({{876, 550}, {916, 570}, {958, 582}}, {pressure={0.5, 0.45, 0.25}})
print("folds drawn")

--@ chunk 41
work(rect(755, 515, 300, 200):grow(24), {pile=table_dark, hand="broad",
      tool="filbert 22", length={70, 160}, coverage=1.9, fill=true, angle=-0.05,
      pressure={0.55, 0.45}, dips={3, 0.9, 0.2}, edge="soft", seed=381})

clothm = poly({{798, 606}, {812, 566}, {846, 556}, {884, 574}, {938, 548},
               {1004, 558}, {1045, 540}, {1045, 706}, {938, 700}, {874, 676},
               {830, 646}}):soften(2)

work(clothm, {pile=cloth, hand="broad", tool="filbert 18", length={60, 150},
      coverage=2.8, fill=true, angle=-0.28, pressure={0.58, 0.48}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=383})
-- the near ridge, catching the light
work(clothm * ribbon({{806, 600}, {862, 618}, {920, 626}, {978, 618}}, 34):blur(13),
      {pile=cloth_lt, hand="broad", tool="filbert 16", length={50, 120}, coverage=2.0,
       fill=true, angle=-0.35, pressure={0.55, 0.45}, dips={2, 0.95, 0.1}, clip=true, seed=385})
-- the valley in front of it
work(clothm * ribbon({{812, 640}, {870, 656}, {928, 662}, {980, 654}}, 26):blur(11),
      {pile=cloth_dk, hand="broad", tool="filbert 16", length={50, 120}, coverage=2.0,
       fill=true, angle=-0.35, pressure={0.6, 0.5}, dips={2, 0.95, 0.1}, clip=true, seed=387})
-- the far side, turning away
work(clothm * ribbon({{878, 562}, {930, 582}, {986, 596}, {1044, 600}}, 32):blur(13),
      {pile=cloth_dk, hand="broad", tool="filbert 16", length={50, 120}, coverage=2.0,
       fill=true, angle=-0.4, pressure={0.6, 0.5}, dips={2, 0.95, 0.1}, clip=true, seed=389})
blend(clothm, {angle=-0.35})
print("cloth, creased")

--@ chunk 42
knock_c = pile{{"raw umber", 4}, {"lead white", 1.2}, {"ultramarine blue", 1},
               {"burnt sienna", 2}}
work(clothm, {pile=knock_c, hand="broad", tool="filbert 20", length={70, 160},
      coverage=1.9, fill=true, angle=-0.3, pressure={0.58, 0.48}, dips={3, 0.9, 0.2},
      clip=true, edge="found", seed=391})
blend(clothm, {angle=-0.35})
-- the ridge picks the light back up
work(clothm * ribbon({{806, 598}, {862, 616}, {920, 624}, {972, 616}}, 26):blur(10),
      {pile=cloth_lt, hand="broad", tool="filbert 14", length={45, 110}, coverage=1.7,
       fill=true, angle=-0.35, pressure={0.52, 0.42}, dips={3, 0.85, 0.2}, clip=true, seed=393})
-- the far end of the cloth falls into the dark
work(clothm * ellipse(1010, 616, 90, 90):blur(40), {pile=cloth_dk, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.8, fill=true, angle=-0.25,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=395})
print("cloth value brought down")

--@ chunk 43
-- cover the bottle opaquely: a dark olive-black, not cobalt
jug_cover = pile{{"bone black", 5}, {"green earth", 3.5}, {"raw umber", 2.5}, {"lead white", 0.8}}
work(jugm, {pile=jug_cover, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.8, fill=true, angle=1.5, pressure={0.6, 0.5}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=401})
blend(jugm, {angle=1.5})

-- the bowl down to a warmer, deeper wood
bowl_knock = pile{{"raw umber", 3}, {"yellow ochre", 1.5}, {"burnt sienna", 1.5},
                 {"lead white", 0.8}, {"ultramarine blue", 0.4}}
work(bowlm, {pile=bowl_knock, hand="broad", tool="filbert 20", length={70, 160},
      coverage=1.9, fill=true, angle=-0.35, pressure={0.58, 0.48}, dips={3, 0.9, 0.2},
      clip=true, edge="found", seed=403})
blend(bowlm, {angle=-0.35})
print("bottle and bowl re-keyed")

--@ chunk 44
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.6, fill=true, angle=-0.3, pressure={0.6, 0.5}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=411})
paint_fruit(q1, 466, 468, 42, 421)
paint_fruit(q2, 583, 478, 35, 431)
-- the near lip of the bowl comes in front of the fruit
rimfront = ribbon({{380, 506}, {432, 536}, {525, 551}, {620, 544}, {678, 512}}, 15)
work(bowlm * rimfront, {pile=bowl_mid, hand="broad", tool="filbert 14", length={40, 100},
      coverage=2.2, fill=true, angle=-0.3, pressure={0.6, 0.5}, dips={2, 0.95, 0.1},
      clip=true, edge="soft", seed=433})
print("bowl interior and fruit restored")

--@ chunk 45
bowl_lt = pile{{"yellow ochre", 2.6}, {"lead white", 1.6}, {"burnt sienna", 1},
               {"raw umber", 1.2}}
work(bowlm * ellipse(706, 586, 190, 175):blur(75), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.7, fill=true, angle=-0.4,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=441})
work(bowlm * ellipse(505, 622, 210, 75):blur(48), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={60, 140}, coverage=1.3, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=443})
blend(bowlm - bowl_in, {angle=-0.35})
work(bowlm * ribbon({{372, 472}, {428, 442}, {498, 429}, {560, 431}}, 20):blur(7),
      {pile=bowl_lt, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.9,
       fill=true, angle=-0.7, pressure={0.55, 0.45}, dips={2, 0.9, 0.15}, clip=true, seed=445})
print("bowl modelled")

--@ chunk 46
jug_hi = pile{{"green earth", 3}, {"lead white", 3}, {"yellow ochre", 0.8}, {"raw umber", 1}}
jug_refl = pile{{"raw umber", 3}, {"burnt sienna", 1.5}, {"ultramarine blue", 0.8}, {"lead white", 0.8}}
jug_dk2 = pile{{"bone black", 4}, {"green earth", 2}, {"ultramarine blue", 1}, {"lead white", 0.4}}

work(jugm * ribbon({{272, 262}, {278, 340}, {288, 440}, {288, 500}, {266, 538}}, 13):blur(8),
      {pile=jug_refl, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.1,
       fill=true, angle=1.5, pressure={0.55, 0.45}, dips={3, 0.8, 0.3}, clip=true, seed=451})
work(jugm * ribbon({{190, 392}, {174, 452}, {176, 512}}, 11):blur(8),
      {pile=jug_hi, hand="broad", tool="filbert 10", length={35, 85}, coverage=0.9,
       fill=true, angle=1.5, pressure={0.55, 0.45}, dips={3, 0.8, 0.3}, clip=true, seed=453})
work(jugm * ribbon({{214, 256}, {217, 330}, {203, 404}, {187, 468}, {191, 528}}, 15):blur(6),
      {pile=jug_hi, hand="broad", tool="filbert 10", length={35, 85}, coverage=1.7,
       fill=true, angle=1.5, pressure={0.55, 0.45}, dips={3, 0.85, 0.25}, clip=true, seed=455})
-- the mouth: dark, with a lit lip
work(jugm * ellipse(241, 246, 27, 7), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.2, pressure={0.6, 0.5}, clip=true, seed=457})
work(jugm * ribbon({{211, 251}, {241, 240}, {271, 250}}, 5):blur(2),
      {pile=jug_hi, hand="detail", tool="round 4", coverage=1.6, pressure={0.6, 0.5},
       clip=true, seed=459})
print("bottle modelled")

--@ chunk 47
jug_hi = pile{{"green earth", 3.5}, {"lead white", 2}, {"yellow ochre", 1.2}, {"raw umber", 2}}
jug_refl = pile{{"raw umber", 3.5}, {"burnt sienna", 1.5}, {"lead white", 0.5}}
jug_dk2 = pile{{"bone black", 4}, {"green earth", 2}, {"raw umber", 2}, {"lead white", 0.3}}

work(jugm * ribbon({{272, 262}, {278, 340}, {288, 440}, {288, 500}, {266, 538}}, 11):blur(9),
      {pile=jug_refl, hand="broad", tool="filbert 12", length={40, 100}, coverage=0.9,
       fill=true, angle=1.5, pressure={0.5, 0.4}, dips={3, 0.7, 0.35}, clip=true, seed=461})
work(jugm * ribbon({{214, 258}, {217, 330}, {203, 404}, {187, 468}, {191, 528}}, 12):blur(7),
      {pile=jug_hi, hand="broad", tool="filbert 10", length={35, 85}, coverage=0.85,
       fill=true, angle=1.5, pressure={0.5, 0.4}, dips={3, 0.7, 0.35}, clip=true, seed=463})
work(jugm * ellipse(241, 246, 25, 6), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.2, pressure={0.6, 0.5}, clip=true, seed=465})
print("bottle toned down")

--@ chunk 48
-- the wall's light: a thin warm glaze over the left, deepening to the right
wall_lt = pile{{"raw umber", 2.5}, {"green earth", 1.5}, {"yellow ochre", 1.2},
               {"lead white", 1.5}, medium=0.2}
work(wallm * ellipse(130, 370, 470, 340):blur(160), {pile=wall_lt, hand="glaze",
      angle=-0.05, coverage=1.1, pressure={0.5, 0.38}, tool="filbert 26",
      dips={3, 0.7, 0.3}, clip=true, edge="lost", seed=471})
work(wallm * ellipse(1010, 210, 420, 420):blur(170), {pile=wall_dk, hand="glaze",
      angle=-0.05, coverage=1.0, pressure={0.5, 0.4}, tool="filbert 26",
      dips={3, 0.7, 0.3}, clip=true, edge="lost", seed=473})
work(wallm * ellipse(400, -40, 620, 220):blur(130), {pile=wall_dk, hand="glaze",
      angle=-0.05, coverage=0.9, pressure={0.5, 0.4}, tool="filbert 26",
      dips={3, 0.7, 0.3}, clip=true, edge="lost", seed=475})
print("wall graded")

--@ chunk 49
work(wallm, {pile=wall_dk, hand="broad", tool="filbert 22", length={70, 170},
      coverage=2.2, fill=true, angle=-0.07, pressure={0.52, 0.42}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=481})
blend(wallm, {angle=0.05})
work(jugm, {pile=jug_cover, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.8, fill=true, angle=1.5, pressure={0.6, 0.5}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=483})
blend(jugm, {angle=1.5})
work(jugm * ribbon({{214, 258}, {217, 330}, {203, 404}, {187, 468}, {191, 528}}, 12):blur(8),
      {pile=jug_hi, hand="broad", tool="filbert 10", length={35, 85}, coverage=0.7,
       fill=true, angle=1.5, pressure={0.5, 0.4}, dips={3, 0.65, 0.4}, clip=true, seed=485})
work(jugm * ribbon({{272, 262}, {278, 340}, {288, 440}, {288, 500}, {266, 538}}, 10):blur(9),
      {pile=jug_refl, hand="broad", tool="filbert 12", length={40, 100}, coverage=0.7,
       fill=true, angle=1.5, pressure={0.5, 0.4}, dips={3, 0.65, 0.4}, clip=true, seed=487})
work(jugm * ellipse(241, 246, 25, 6), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.2, pressure={0.6, 0.5}, clip=true, seed=489})
print("wall and bottle repaired")

--@ chunk 50
-- clear the haze that crossed the bowl's rim
work(wallm * rect(340, 395, 400, 80), {pile=wall_dk, hand="broad", tool="filbert 20",
      length={70, 160}, coverage=2.0, fill=true, angle=-0.07, pressure={0.52, 0.42},
      dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=491})
blend(wallm * rect(340, 395, 400, 80):grow(25), {angle=0.0})

-- the wall's light, laid thin and broad this time
wall_lt = pile{{"raw umber", 2.5}, {"green earth", 1.5}, {"yellow ochre", 1.2}, {"lead white", 1.5}}
work(wallm * ellipse(150, 380, 480, 350):blur(170), {pile=wall_lt, hand="broad",
      tool="filbert 22", length={80, 180}, coverage=0.95, fill=true, angle=-0.07,
      pressure={0.45, 0.34}, dips={4, 0.7, 0.35}, clip=true, edge="lost", seed=493})
blend(wallm * ellipse(150, 380, 480, 350):blur(170):grow(70), {angle=0.04})

wall_deep = pile{{"bone black", 2.5}, {"raw umber", 3}, {"green earth", 1.5}, {"ultramarine blue", 0.8}}
work(wallm * ellipse(1010, 250, 460, 440):blur(185), {pile=wall_deep, hand="broad",
      tool="filbert 22", length={80, 180}, coverage=0.9, fill=true, angle=-0.07,
      pressure={0.5, 0.4}, dips={4, 0.75, 0.3}, clip=true, edge="lost", seed=495})
blend(wallm * ellipse(1010, 250, 460, 440):blur(185):grow(70), {angle=0.04})

-- the bowl's lit rim again
work(bowlm * ribbon({{372, 472}, {428, 442}, {498, 429}, {560, 431}}, 18):blur(6),
      {pile=bowl_lt, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.8,
       fill=true, angle=-0.7, pressure={0.55, 0.45}, dips={2, 0.9, 0.15}, clip=true, seed=497})
print("wall graded, rim restored")

--@ chunk 51
work(wallm, {pile=wall_dk, hand="broad", tool="filbert 22", length={70, 170},
      coverage=2.4, fill=true, angle=-0.07, pressure={0.52, 0.42}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=501})
blend(wallm, {angle=0.05})
work(jugm, {pile=jug_cover, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.8, fill=true, angle=1.5, pressure={0.6, 0.5}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=503})
blend(jugm, {angle=1.5})
print("repaired")

--@ chunk 52
work(tablem * rect(752, 455, 260, 300), {pile=table_dark, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=2.2, fill=true, angle=-0.05,
      pressure={0.52, 0.42}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=511})
blend(tablem * rect(752, 455, 260, 300):grow(40), {angle=-0.05})

work(clothm, {pile=cloth, hand="broad", tool="filbert 18", length={60, 150},
      coverage=2.6, fill=true, angle=-0.28, pressure={0.55, 0.45}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=513})
work(clothm * ribbon({{806, 598}, {862, 616}, {920, 624}, {972, 616}}, 24):blur(9),
      {pile=cloth_lt, hand="broad", tool="filbert 14", length={45, 110}, coverage=1.6,
       fill=true, angle=-0.35, pressure={0.52, 0.42}, dips={3, 0.85, 0.2}, clip=true, seed=515})
work(clothm * ribbon({{812, 640}, {870, 656}, {928, 662}, {980, 654}}, 22):blur(10),
      {pile=cloth_dk, hand="broad", tool="filbert 16", length={50, 120}, coverage=1.8,
       fill=true, angle=-0.35, pressure={0.58, 0.48}, dips={3, 0.9, 0.2}, clip=true, seed=517})
work(clothm * ellipse(1010, 614, 80, 84):blur(36), {pile=cloth_dk, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.6, fill=true, angle=-0.25,
      pressure={0.58, 0.48}, clip=true, edge="soft", seed=519})
print("cloth relaid cleanly")

--@ chunk 53
cloth_deep = pile{{"raw umber", 4}, {"burnt sienna", 1.5}, {"ultramarine blue", 0.8},
                  {"lead white", 0.5}}
work(clothm, {pile=cloth_deep, hand="broad", tool="filbert 18", length={60, 150},
      coverage=2.6, fill=true, angle=-0.28, pressure={0.55, 0.45}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=521})
blend(clothm, {angle=-0.3})
work(clothm * ribbon({{800, 598}, {860, 616}, {920, 626}, {976, 618}}, 17):blur(5),
      {pile=cloth_lt, hand="broad", tool="filbert 12", length={40, 95}, coverage=1.7,
       fill=true, angle=-0.4, pressure={0.5, 0.4}, dips={3, 0.85, 0.2}, clip=true, seed=523})
work(clothm * ribbon({{818, 574}, {874, 590}, {934, 600}, {988, 600}}, 12):blur(5),
      {pile=cloth_lt, hand="broad", tool="filbert 10", length={35, 80}, coverage=1.4,
       fill=true, angle=-0.4, pressure={0.48, 0.38}, dips={3, 0.8, 0.25}, clip=true, seed=525})
stipple(clothm, {pile=cloth_lt, width=7, tool="stippler 7",
      coverage=function(x, y) return 0.4 + 0.22 * math.sin(x / 26) end,
      pressure={0.32, 0.14}, dips={20, 0.35, 0.5}, cluster={0.6, 40},
      feather=0.4, seed=527})
print("cloth as dark linen with lit folds")

--@ chunk 54
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.4, fill=true, angle=-0.3, pressure={0.6, 0.5}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=531})
paint_fruit(q1, 466, 468, 42, 541)
paint_fruit(q2, 583, 478, 35, 551)
work(bowlm * rimfront, {pile=bowl_mid, hand="broad", tool="filbert 14", length={40, 100},
      coverage=2.0, fill=true, angle=-0.3, pressure={0.6, 0.5}, dips={2, 0.95, 0.1},
      clip=true, edge="soft", seed=553})
paint_fruit(q4, 730, 592, 41, 561)
print("quinces redone")

--@ chunk 55
-- the bowl's right side falls away into the dark
work(bowlm * ellipse(726, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.8, fill=true, angle=-0.4,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=571})
work(bowlm * rimfront * rect(575, 480, 200, 100), {pile=bowl_dk, hand="broad",
      tool="filbert 12", length={35, 85}, coverage=1.6, fill=true, angle=-0.3,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=573})
work(bowlm * ellipse(500, 628, 230, 80):blur(50), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.2, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=575})

-- cast shadows on the table, thrown to the right
work(tablem * ribbon({{248, 536}, {330, 574}, {404, 606}}, 46):blur(22),
      {pile=cast, hand="scumble", tool="filbert 16", coverage=0.95, angle=-0.25,
       pressure={0.6, 0.35}, seed=577})
work(tablem * ellipse(772, 636, 118, 36):blur(30),
      {pile=cast, hand="scumble", tool="filbert 16", coverage=0.9, angle=-0.15,
       pressure={0.6, 0.35}, seed=579})
print("bowl darkened, cast shadows laid")

--@ chunk 56
blob1 = ribbon({{230, 515}, {330, 568}, {432, 622}}, 78):blur(32)
blob2 = ellipse(800, 638, 165, 66):blur(36)
blob3 = ellipse(400, 684, 120, 55):blur(32)
knockm = (blob1 + blob2 + blob3) * tablem
work(knockm, {pile=table_dark, hand="broad", tool="filbert 22", length={80, 170},
      coverage=2.0, fill=true, angle=-0.05, pressure={0.52, 0.42}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=581})
blend(knockm:grow(45), {angle=-0.05})
print("blobs knocked back")

--@ chunk 57
shadowm = (ribbon({{250, 540}, {332, 576}, {406, 608}}, 44):blur(20)
          + ellipse(780, 634, 104, 30):blur(26)
          + ellipse(398, 678, 62, 17):blur(15)
          + ellipse(776, 614, 54, 16):blur(14)) * tablem
work(shadowm, {pile=cast, hand="broad", tool="filbert 24", length={90, 190},
      coverage=0.85, fill=true, angle=-0.22, pressure={0.34, 0.26},
      dips={4, 0.65, 0.4}, clip=true, edge="lost", seed=583})
print("shadows laid thin")

--@ chunk 58
work(bowlm - bowl_in, {pile=bowl_mid, hand="broad", tool="filbert 20", length={70, 160},
      coverage=2.2, fill=true, angle=-0.35, pressure={0.58, 0.48}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=591})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.4, fill=true, angle=-0.3, pressure={0.6, 0.5}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=593})
paint_fruit(q1, 466, 468, 42, 601)
paint_fruit(q2, 583, 478, 35, 611)
work(bowlm * rimfront, {pile=bowl_mid, hand="broad", tool="filbert 14", length={40, 100},
      coverage=2.0, fill=true, angle=-0.3, pressure={0.6, 0.5}, dips={2, 0.95, 0.1},
      clip=true, edge="soft", seed=613})
work(bowlm * ellipse(726, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.6, fill=true, angle=-0.4,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=615})
work(bowlm * ellipse(500, 628, 230, 80):blur(50), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.1, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=617})
work(bowlm * ribbon({{372, 472}, {428, 442}, {498, 429}, {560, 431}}, 18):blur(6),
      {pile=bowl_lt, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.8,
       fill=true, angle=-0.7, pressure={0.55, 0.45}, dips={2, 0.9, 0.15}, clip=true, seed=619})
paint_fruit(q3, 350, 643, 47, 621)
paint_fruit(q4, 730, 592, 41, 631)
print("bowl and all four quinces repaired")

--@ chunk 59
-- give the bowl its value back against the table
work(bowlm * ellipse(436, 516, 185, 195):blur(75), {pile=bowl_lt, hand="broad",
      tool="filbert 18", length={60, 150}, coverage=1.5, fill=true, angle=-0.45,
      pressure={0.55, 0.45}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=641})
work(bowlm * rimfront, {pile=bowl_lt, hand="broad", tool="filbert 14", length={40, 100},
      coverage=1.5, fill=true, angle=-0.3, pressure={0.58, 0.48}, dips={3, 0.9, 0.2},
      clip=true, edge="soft", seed=643})
work(bowlm * ribbon({{372, 472}, {428, 442}, {498, 429}, {560, 431}}, 18):blur(6),
      {pile=bowl_lt, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.8,
       fill=true, angle=-0.7, pressure={0.55, 0.45}, clip=true, seed=645})
blend(bowlm - bowl_in - q1 - q2, {angle=-0.4})

-- the cloth, repainted end to end
work(clothm, {pile=cloth_deep, hand="broad", tool="filbert 18", length={60, 150},
      coverage=2.6, fill=true, angle=-0.28, pressure={0.55, 0.45}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=647})
blend(clothm, {angle=-0.3})
work(clothm * ribbon({{800, 598}, {860, 616}, {920, 626}, {976, 618}}, 17):blur(5),
      {pile=cloth_lt, hand="broad", tool="filbert 12", length={40, 95}, coverage=1.7,
       fill=true, angle=-0.4, pressure={0.5, 0.4}, dips={3, 0.85, 0.2}, clip=true, seed=649})
work(clothm * ribbon({{818, 574}, {874, 590}, {934, 600}, {988, 600}}, 12):blur(5),
      {pile=cloth_lt, hand="broad", tool="filbert 10", length={35, 80}, coverage=1.4,
       fill=true, angle=-0.4, pressure={0.48, 0.38}, clip=true, seed=651})
stipple(clothm, {pile=cloth_lt, width=7, tool="stippler 7",
      coverage=function(x, y) return 0.4 + 0.22 * math.sin(x / 26) end,
      pressure={0.32, 0.14}, dips={20, 0.35, 0.5}, cluster={0.6, 40}, feather=0.4, seed=653})
print("bowl and cloth restored")

--@ chunk 60
bowlfront = bowlm - bowl_in - q1 - q2
work(bowlfront, {pile=bowl_mid, hand="broad", tool="filbert 20", length={70, 160},
      coverage=2.2, fill=true, angle=-0.35, pressure={0.58, 0.48}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=661})
work(bowlfront * ellipse(430, 520, 190, 200):blur(75), {pile=bowl_lt, hand="broad",
      tool="filbert 18", length={60, 150}, coverage=1.5, fill=true, angle=-0.45,
      pressure={0.55, 0.45}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=663})
work(bowlfront * ellipse(726, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.6, fill=true, angle=-0.4,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=665})
work(bowlfront * ellipse(500, 630, 230, 80):blur(50), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.1, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=667})
work(bowlfront * ribbon({{372, 472}, {428, 442}, {498, 429}, {560, 431}}, 18):blur(6),
      {pile=bowl_lt, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.8,
       fill=true, angle=-0.7, pressure={0.55, 0.45}, clip=true, seed=669})
blend(bowlfront, {angle=-0.4})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.4, fill=true, angle=-0.3, pressure={0.6, 0.5}, clip=true, edge="found", seed=671})
paint_fruit(q1, 466, 468, 42, 681)
paint_fruit(q2, 583, 478, 35, 691)
work(bowlfront * rimfront, {pile=bowl_lt, hand="broad", tool="filbert 14", length={40, 100},
      coverage=1.6, fill=true, angle=-0.3, pressure={0.58, 0.48}, clip=true, edge="soft", seed=701})
print("bowl rebuilt in order")

--@ chunk 61
print("wall", drying(300, 200), "table", drying(500, 650), "bowl", drying(450, 550),
      "jug", drying(230, 400), "cloth", drying(880, 610))
print(wait(600))

--@ chunk 62
print("wall", drying(300, 200), "table", drying(500, 650), "bowl", drying(450, 550),
      "jug", drying(230, 400), "cloth", drying(880, 610))

--@ chunk 63
wallfree = wallm - jugm - bowlm
work(wallfree, {pile=wall_dk, hand="broad", tool="filbert 22", length={80, 180},
      coverage=0.85, fill=true, angle=-0.07, pressure={0.42, 0.32}, fill=true,
      dips={4, 0.7, 0.35}, clip=true, edge="soft", seed=711})
work(wallfree * rect(415, 388, 240, 84), {pile=wall_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.9, fill=true, angle=-0.07,
      pressure={0.5, 0.4}, clip=true, edge="soft", seed=713})
print("wall evened, smear cleared")

--@ chunk 64
jug_hi = pile{{"green earth", 3.5}, {"lead white", 1.4}, {"yellow ochre", 1.5}, {"raw umber", 2.5}}
jug_refl = pile{{"raw umber", 3.5}, {"burnt sienna", 1.5}, {"lead white", 0.5}}

fb = brush{kind="filbert", width=20}
fb:load(jug_hi, 0.32)
fb:stroke({{208, 372}, {197, 446}, {204, 508}}, {pressure={0.3, 0.36, 0.24}})
fb:stroke({{219, 300}, {216, 344}}, {pressure={0.3, 0.24}})

jb = brush{kind="rigger", width=6, point=0.85}
jb:load(jug_hi, 0.9)
jb:stroke({{215, 262}, {216, 330}, {204, 402}, {189, 466}, {192, 526}},
          {pressure={0.4, 0.6, 0.55, 0.5, 0.3}})
jb:reload(jug_hi, 0.75)
jb:stroke({{213, 272}, {215, 320}}, {pressure={0.55, 0.4}})
jb:stroke({{196, 430}, {190, 470}, {194, 508}}, {pressure={0.5, 0.45, 0.3}})

rb = brush{kind="rigger", width=7, point=0.85}
rb:load(jug_refl, 0.65)
rb:stroke({{271, 268}, {277, 350}, {286, 442}, {284, 500}, {264, 534}},
          {pressure={0.25, 0.32, 0.38, 0.32, 0.2}})
print("bottle modelled with direct strokes")

--@ chunk 65
tablefree = tablem - jugm - bowlm - q1 - q2 - q3 - q4 - clothm
pool_light = pile{{"raw umber", 3}, {"burnt sienna", 1.5}, {"yellow ochre", 1},
                  {"lead white", 1.2}, medium=0.12}
table_deep = pile{{"bone black", 3}, {"raw umber", 2.5}, {"burnt sienna", 1},
                  {"ultramarine blue", 0.6}}

work(tablefree * ellipse(270, 668, 460, 180):blur(125), {pile=pool_light, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=0.9, fill=true, angle=-0.04,
      pressure={0.4, 0.32}, dips={4, 0.65, 0.4}, clip=true, edge="lost", seed=721})
work(tablefree * ellipse(965, 706, 430, 330):blur(155), {pile=table_deep, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=1.0, fill=true, angle=-0.06,
      pressure={0.42, 0.34}, dips={4, 0.7, 0.35}, clip=true, edge="lost", seed=723})
work(tablefree * rect(-20, 700, 1040, 80):blur(55), {pile=table_deep, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=0.85, fill=true, angle=-0.04,
      pressure={0.42, 0.34}, dips={4, 0.7, 0.35}, clip=true, edge="lost", seed=725})

shadowm = (ribbon({{250, 538}, {332, 574}, {406, 606}}, 42):blur(20)
          + ellipse(784, 632, 100, 28):blur(24)
          + ellipse(398, 674, 58, 15):blur(14)
          + ellipse(778, 610, 50, 15):blur(13)) * tablefree
work(shadowm, {pile=cast, hand="broad", tool="filbert 24", length={90, 190},
      coverage=0.8, fill=true, angle=-0.22, pressure={0.32, 0.24}, dips={4, 0.6, 0.45},
      clip=true, edge="lost", seed=727})
print("light pool, deep right, soft cast shadows")

--@ chunk 66
print("free at fruit centres:", tablefree:at(350, 643), tablefree:at(466, 468),
      tablefree:at(230, 400), tablefree:at(880, 610))
print("open table:", tablefree:at(500, 660))
print(wait(660))

--@ chunk 67
print("table", drying(500, 660), "wall", drying(300, 200), "bowl", drying(450, 570))
-- the wall, laid solid again
work(wallfree, {pile=wall_dk, hand="broad", tool="filbert 22", length={80, 180},
      coverage=2.4, fill=true, angle=-0.07, pressure={0.55, 0.45},
      dips={3, 0.92, 0.15}, clip=true, edge="soft", seed=731})
blend(wallfree, {angle=0.05})
print("wall re-laid")

--@ chunk 68
work(tablefree, {pile=table_dark, hand="broad", tool="filbert 22", length={80, 180},
      coverage=2.4, fill=true, angle=-0.05, pressure={0.55, 0.45},
      dips={3, 0.92, 0.15}, clip=true, edge="soft", seed=741})
blend(tablefree, {angle=-0.05})
work(tablefree * ellipse(270, 668, 460, 180):blur(125), {pile=pool_light, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=1.3, fill=true, angle=-0.04,
      pressure={0.48, 0.38}, dips={4, 0.7, 0.3}, clip=true, edge="firm", seed=743})
work(tablefree * ellipse(965, 706, 430, 330):blur(155), {pile=table_deep, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=1.5, fill=true, angle=-0.06,
      pressure={0.5, 0.4}, dips={4, 0.75, 0.25}, clip=true, edge="firm", seed=745})
work(tablefree * rect(-20, 700, 1040, 80):blur(55), {pile=table_deep, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=1.2, fill=true, angle=-0.04,
      pressure={0.5, 0.4}, dips={4, 0.75, 0.25}, clip=true, edge="firm", seed=747})
shadowm = (ribbon({{250, 538}, {332, 574}, {406, 606}}, 42):blur(20)
          + ellipse(784, 632, 100, 28):blur(24)
          + ellipse(398, 674, 58, 15):blur(14)
          + ellipse(778, 610, 50, 15):blur(13)) * tablefree
work(shadowm, {pile=cast, hand="broad", tool="filbert 24", length={90, 190},
      coverage=1.2, fill=true, angle=-0.22, pressure={0.42, 0.32}, dips={4, 0.7, 0.3},
      clip=true, edge="firm", seed=749})
print("table re-laid with its light")

--@ chunk 69
work(tablefree, {pile=table_dark, hand="broad", tool="filbert 24", length={90, 190},
      coverage=3.2, fill=true, angle=-0.05, pressure={0.64, 0.54},
      dips={2, 0.98, 0.05}, clip=true, edge="soft", seed=751})
blend(tablefree, {angle=-0.05})
work(tablefree * ellipse(270, 668, 460, 180):blur(125), {pile=pool_light, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=1.5, fill=true, angle=-0.04,
      pressure={0.5, 0.4}, dips={4, 0.7, 0.3}, clip=true, edge="firm", seed=753})
work(tablefree * ellipse(965, 706, 430, 330):blur(155), {pile=table_deep, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=1.6, fill=true, angle=-0.06,
      pressure={0.52, 0.42}, dips={4, 0.75, 0.25}, clip=true, edge="firm", seed=755})
print("table re-coated heavy")

--@ chunk 70
work(bowlfront, {pile=bowl_mid, hand="broad", tool="filbert 20", length={70, 160},
      coverage=3.0, fill=true, angle=-0.35, pressure={0.62, 0.52}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=761})
work(bowlfront * ellipse(430, 520, 190, 200):blur(75), {pile=bowl_lt, hand="broad",
      tool="filbert 18", length={60, 150}, coverage=1.5, fill=true, angle=-0.45,
      pressure={0.55, 0.45}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=763})
work(bowlfront * ellipse(726, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.7, fill=true, angle=-0.4,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=765})
work(bowlfront * ellipse(500, 630, 230, 80):blur(50), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.2, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=767})
work(bowlfront * ribbon({{372, 472}, {428, 442}, {498, 429}, {560, 431}}, 18):blur(6),
      {pile=bowl_lt, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.9,
       fill=true, angle=-0.7, pressure={0.55, 0.45}, clip=true, seed=769})
blend(bowlfront, {angle=-0.4})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.6, fill=true, angle=-0.3, pressure={0.6, 0.5}, clip=true, edge="found", seed=771})
paint_fruit(q1, 466, 468, 42, 781)
paint_fruit(q2, 583, 478, 35, 791)
paint_fruit(q3, 350, 643, 47, 801)
paint_fruit(q4, 730, 592, 41, 811)
work(bowlfront * rimfront, {pile=bowl_lt, hand="broad", tool="filbert 14", length={40, 100},
      coverage=1.8, fill=true, angle=-0.3, pressure={0.58, 0.48}, clip=true, edge="soft", seed=821})
print("bowl and fruit re-coated")

--@ chunk 71
work(clothm, {pile=cloth_deep, hand="broad", tool="filbert 18", length={60, 150},
      coverage=3.0, fill=true, angle=-0.28, pressure={0.62, 0.52}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=831})
work(clothm * ellipse(1000, 600, 110, 90):blur(45), {pile=cloth_dk, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.6, fill=true, angle=-0.25,
      pressure={0.58, 0.48}, clip=true, edge="soft", seed=833})
work(clothm * ribbon({{812, 640}, {870, 656}, {928, 662}, {980, 654}}, 22):blur(10),
      {pile=cloth_dk, hand="broad", tool="filbert 16", length={50, 120}, coverage=1.7,
       fill=true, angle=-0.35, pressure={0.58, 0.48}, clip=true, seed=835})
work(clothm * ribbon({{800, 598}, {860, 616}, {920, 626}, {976, 618}}, 17):blur(5),
      {pile=cloth_lt, hand="broad", tool="filbert 12", length={40, 95}, coverage=1.8,
       fill=true, angle=-0.4, pressure={0.5, 0.4}, clip=true, seed=837})
work(clothm * ribbon({{818, 574}, {874, 590}, {934, 600}, {988, 600}}, 12):blur(5),
      {pile=cloth_lt, hand="broad", tool="filbert 10", length={35, 80}, coverage=1.5,
       fill=true, angle=-0.4, pressure={0.48, 0.38}, clip=true, seed=839})
stipple(clothm, {pile=cloth_lt, width=7, tool="stippler 7",
      coverage=function(x, y) return 0.4 + 0.22 * math.sin(x / 26) end,
      pressure={0.32, 0.14}, dips={20, 0.35, 0.5}, cluster={0.6, 40}, feather=0.4, seed=841})
print("cloth re-coated")

--@ chunk 72
bowl_warm = pile{{"yellow ochre", 1.2}, {"burnt sienna", 2}, {"raw umber", 3},
                 {"lead white", 0.5}, {"ultramarine blue", 0.3}}
work(bowlfront, {pile=bowl_warm, hand="broad", tool="filbert 20", length={70, 160},
      coverage=2.8, fill=true, angle=-0.35, pressure={0.62, 0.52}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=851})
work(bowlfront * ellipse(430, 520, 190, 200):blur(75), {pile=bowl_mid, hand="broad",
      tool="filbert 18", length={60, 150}, coverage=1.6, fill=true, angle=-0.45,
      pressure={0.58, 0.48}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=853})
work(bowlfront * ellipse(726, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.8, fill=true, angle=-0.4,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=855})
work(bowlfront * ellipse(500, 630, 230, 80):blur(50), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.3, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=857})
work(bowlfront * ribbon({{372, 472}, {428, 442}, {498, 429}, {560, 431}}, 16):blur(6),
      {pile=bowl_lt, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.6,
       fill=true, angle=-0.7, pressure={0.55, 0.45}, clip=true, seed=859})
blend(bowlfront, {angle=-0.4})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.6, fill=true, angle=-0.3, pressure={0.6, 0.5}, clip=true, edge="found", seed=861})
paint_fruit(q1, 466, 468, 42, 871)
paint_fruit(q2, 583, 478, 35, 881)
work(bowlfront * rimfront, {pile=bowl_mid, hand="broad", tool="filbert 14", length={40, 100},
      coverage=1.7, fill=true, angle=-0.3, pressure={0.58, 0.48}, clip=true, edge="soft", seed=891})
print("bowl warmed and darkened")

--@ chunk 73
bowl_dd = pile{{"burnt sienna", 2.5}, {"raw umber", 3}, {"ultramarine blue", 0.5},
               {"bone black", 1.2}, {"lead white", 0.3}}
work(bowlfront, {pile=bowl_dd, hand="broad", tool="filbert 20", length={70, 160},
      coverage=2.8, fill=true, angle=-0.35, pressure={0.62, 0.52}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=901})
work(bowlfront * ellipse(424, 516, 175, 195):blur(78), {pile=bowl_mid, hand="broad",
      tool="filbert 18", length={60, 150}, coverage=1.5, fill=true, angle=-0.45,
      pressure={0.58, 0.48}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=903})
work(bowlfront * ellipse(726, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.8, fill=true, angle=-0.4,
      pressure={0.6, 0.5}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=905})
work(bowlfront * ellipse(490, 634, 240, 82):blur(52), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.4, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=907})
work(bowlfront * ribbon({{372, 472}, {428, 442}, {498, 429}, {560, 431}}, 16):blur(6),
      {pile=bowl_lt, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.7,
       fill=true, angle=-0.7, pressure={0.55, 0.45}, clip=true, seed=909})
blend(bowlfront, {angle=-0.4})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.6, fill=true, angle=-0.3, pressure={0.6, 0.5}, clip=true, edge="found", seed=911})
paint_fruit(q1, 466, 468, 42, 921)
paint_fruit(q2, 583, 478, 35, 931)
work(bowlfront * rimfront, {pile=bowl_mid, hand="broad", tool="filbert 14", length={40, 100},
      coverage=1.7, fill=true, angle=-0.3, pressure={0.58, 0.48}, clip=true, edge="soft", seed=941})
print("bowl brought down to a mid-dark vessel")

--@ chunk 74
jug_soft = pile{{"green earth", 4}, {"raw umber", 3}, {"lead white", 1}, {"yellow ochre", 1.2}}
work(jugm * ellipse(202, 418, 64, 168):blur(48), {pile=jug_soft, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.3, fill=true, angle=1.5,
      pressure={0.55, 0.45}, dips={3, 0.85, 0.25}, clip=true, edge="soft", seed=951})
work(jugm * ribbon({{272, 262}, {278, 350}, {286, 442}, {284, 500}, {264, 534}}, 12):blur(9),
      {pile=jug_refl, hand="broad", tool="filbert 12", length={40, 100}, coverage=1.0,
       fill=true, angle=1.5, pressure={0.5, 0.4}, clip=true, seed=953})
work(jugm * ribbon({{214, 258}, {217, 330}, {203, 404}, {187, 468}, {191, 528}}, 14):blur(8),
      {pile=jug_hi, hand="broad", tool="filbert 10", length={35, 85}, coverage=1.2,
       fill=true, angle=1.5, pressure={0.52, 0.42}, clip=true, seed=955})
-- the table reflected in the belly, and the dark mouth
work(jugm * ribbon({{182, 508}, {210, 524}, {258, 530}}, 16):blur(9),
      {pile=pile{{"burnt sienna", 3}, {"yellow ochre", 1.2}, {"raw umber", 2}},
       hand="broad", tool="filbert 10", length={30, 70}, coverage=0.9, fill=true,
       angle=0.2, pressure={0.48, 0.38}, clip=true, seed=957})
work(jugm * ellipse(241, 246, 25, 6), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.2, pressure={0.6, 0.5}, clip=true, seed=959})
print("bottle brought back into the light")

--@ chunk 75
work(jugm, {pile=jug_cover, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.9, fill=true, angle=1.5, pressure={0.64, 0.54}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=961})
blend(jugm, {angle=1.5})
work(jugm * ribbon({{213, 262}, {216, 330}, {203, 404}, {188, 466}, {191, 524}}, 11):blur(6),
      {pile=jug_hi, hand="broad", tool="filbert 9", length={30, 70}, coverage=1.0,
       fill=true, angle=1.5, pressure={0.5, 0.4}, clip=true, seed=963})
work(jugm * ribbon({{272, 266}, {278, 350}, {285, 440}, {282, 496}, {263, 530}}, 9):blur(8),
      {pile=jug_refl, hand="broad", tool="filbert 11", length={35, 85}, coverage=0.8,
       fill=true, angle=1.5, pressure={0.46, 0.36}, clip=true, seed=965})
work(jugm * ellipse(241, 246, 25, 6), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.2, pressure={0.6, 0.5}, clip=true, seed=967})
print("bottle back to dark")

--@ chunk 76
deep = pile{{"bone black", 3}, {"raw umber", 3}, {"ultramarine blue", 1},
            {"green earth", 0.8}}
corner = (ellipse(50, 30, 300, 250):blur(120) + ellipse(960, 40, 320, 270):blur(125)
        + ellipse(50, 735, 320, 200):blur(120) + ellipse(950, 730, 330, 210):blur(125))
work(corner, {pile=deep, hand="broad", tool="filbert 24", length={90, 190},
      coverage=1.3, fill=true, angle=-0.1, pressure={0.5, 0.4}, dips={4, 0.72, 0.3},
      clip=true, edge="firm", seed=971})
print("corners deepened")

--@ chunk 77
hot = pile{{"lead white", 3.5}, {"cadmium yellow", 1.5},
           {"transparent oxide yellow", 1}, {"yellow ochre", 0.5}}
ab = brush{kind="filbert", width=7}
ab:load(hot, 0.92)
ab:stroke({{438, 456}, {450, 444}, {466, 439}}, {pressure={0.45, 0.68, 0.28}})
ab:reload(hot, 0.9)
ab:stroke({{558, 470}, {570, 459}, {583, 456}}, {pressure={0.4, 0.62, 0.25}})
ab:reload(hot, 0.92)
ab:stroke({{322, 624}, {336, 610}, {354, 606}}, {pressure={0.5, 0.72, 0.3}})
ab:reload(hot, 0.9)
ab:stroke({{702, 578}, {715, 566}, {731, 563}}, {pressure={0.4, 0.6, 0.25}})

-- the rim of the bowl takes the light
rb2 = brush{kind="filbert", width=9}
rb2:load(pile{{"lead white", 3}, {"yellow ochre", 2.5}, {"burnt sienna", 1},
               {"raw umber", 1.2}}, 0.9)
rb2:stroke({{382, 468}, {412, 452}, {446, 440}, {476, 434}}, {pressure={0.45, 0.6, 0.55, 0.3}})
rb2:reload(pile{{"lead white", 3}, {"yellow ochre", 2.5}, {"burnt sienna", 1}, {"raw umber", 1.2}}, 0.85)
rb2:stroke({{486, 432}, {522, 431}, {556, 434}}, {pressure={0.5, 0.45, 0.25}})

-- the crest of the cloth
cb = brush{kind="filbert", width=8}
cb:load(cloth_lt, 0.9)
cb:stroke({{804, 596}, {840, 608}, {878, 614}}, {pressure={0.45, 0.62, 0.3}})
cb:reload(cloth_lt, 0.85)
cb:stroke({{820, 572}, {856, 584}}, {pressure={0.5, 0.3}})
print("accents laid")

--@ chunk 78
veil_w = pile{{"raw umber", 4}, {"burnt sienna", 1.6}, {"bone black", 1.2},
              {"ultramarine blue", 0.4}, medium=0.2}
work(wallfree, {pile=veil_w, hand="broad", tool="filbert 24", length={90, 190},
      coverage=1.5, fill=true, angle=-0.07, pressure={0.52, 0.42}, fill=true,
      dips={3, 0.8, 0.25}, clip=true, edge="soft", seed=981})
blend(wallfree, {angle=0.05})
veil_t = pile{{"raw umber", 3.4}, {"burnt sienna", 2.2}, {"bone black", 1.4},
              {"ultramarine blue", 0.3}, medium=0.18}
work(tablefree, {pile=veil_t, hand="broad", tool="filbert 24", length={90, 190},
      coverage=1.4, fill=true, angle=-0.05, pressure={0.52, 0.42},
      dips={3, 0.8, 0.25}, clip=true, edge="soft", seed=983})
blend(tablefree, {angle=-0.05})
print("background veiled and blended")

--@ chunk 79
print(wait(560))
print("wall", drying(300, 250), "table", drying(500, 660), "bowl", drying(450, 560))

--@ chunk 80
print(wait(300))
print("wall", drying(300, 250), "table", drying(500, 660), "bowl", drying(450, 560),
      "jug", drying(230, 400), "cloth", drying(880, 610))

--@ chunk 81
work(bowlfront, {pile=bowl_dd, hand="broad", tool="filbert 20", length={70, 160},
      coverage=3.0, fill=true, angle=-0.35, pressure={0.62, 0.52}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=991})
work(bowlfront * ellipse(424, 516, 175, 195):blur(78), {pile=bowl_mid, hand="broad",
      tool="filbert 18", length={60, 150}, coverage=1.6, fill=true, angle=-0.45,
      pressure={0.58, 0.48}, dips={3, 0.9, 0.2}, clip=true, edge="soft", seed=993})
work(bowlfront * ellipse(726, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.8, fill=true, angle=-0.4,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=995})
work(bowlfront * ellipse(490, 634, 240, 82):blur(52), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.4, fill=true, angle=-0.2,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=997})
blend(bowlfront, {angle=-0.4})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.6, fill=true, angle=-0.3, pressure={0.6, 0.5}, clip=true, edge="found", seed=999})
paint_fruit(q1, 466, 468, 42, 1011)
paint_fruit(q2, 583, 478, 35, 1021)
paint_fruit(q3, 350, 643, 47, 1031)
paint_fruit(q4, 730, 592, 41, 1041)
work(bowlfront * rimfront, {pile=bowl_mid, hand="broad", tool="filbert 14", length={40, 100},
      coverage=1.8, fill=true, angle=-0.3, pressure={0.58, 0.48}, clip=true, edge="found", seed=1051})
print("bowl and fruit re-laid crisply")

--@ chunk 82
cloth_deep = pile{{"raw umber", 4}, {"burnt sienna", 1.8}, {"lead white", 0.6}}
cloth_dk = pile{{"raw umber", 4}, {"burnt sienna", 1.5}, {"lead white", 0.4}}
work(clothm, {pile=cloth_deep, hand="broad", tool="filbert 18", length={60, 150},
      coverage=3.0, fill=true, angle=-0.28, pressure={0.62, 0.52}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1061})
work(clothm * ribbon({{812, 640}, {870, 656}, {928, 662}, {980, 654}}, 22):blur(9),
      {pile=cloth_dk, hand="broad", tool="filbert 16", length={50, 120}, coverage=1.8,
       fill=true, angle=-0.35, pressure={0.58, 0.48}, clip=true, seed=1063})
work(clothm * ribbon({{872, 556}, {922, 578}, {978, 592}, {1042, 596}}, 30):blur(12),
      {pile=cloth_dk, hand="broad", tool="filbert 16", length={50, 120}, coverage=1.8,
       fill=true, angle=-0.4, pressure={0.58, 0.48}, clip=true, seed=1065})
work(clothm * ellipse(1010, 606, 95, 80):blur(40), {pile=cloth_dk, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.7, fill=true, angle=-0.25,
      pressure={0.58, 0.48}, clip=true, edge="soft", seed=1067})
work(clothm * ribbon({{800, 598}, {860, 616}, {920, 626}, {976, 618}}, 17):blur(5),
      {pile=cloth_lt, hand="broad", tool="filbert 12", length={40, 95}, coverage=1.9,
       fill=true, angle=-0.4, pressure={0.5, 0.4}, clip=true, seed=1069})
work(clothm * ribbon({{818, 574}, {874, 590}, {934, 600}}, 12):blur(5),
      {pile=cloth_lt, hand="broad", tool="filbert 10", length={35, 80}, coverage=1.6,
       fill=true, angle=-0.4, pressure={0.48, 0.38}, clip=true, seed=1071})

work(jugm, {pile=jug_cover, hand="broad", tool="filbert 16", length={50, 120},
      coverage=3.0, fill=true, angle=1.5, pressure={0.64, 0.54}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1073})
work(jugm * ribbon({{213, 262}, {216, 330}, {203, 404}, {188, 466}, {191, 524}}, 11):blur(6),
      {pile=jug_hi, hand="broad", tool="filbert 9", length={30, 70}, coverage=1.0,
       fill=true, angle=1.5, pressure={0.5, 0.4}, clip=true, seed=1075})
work(jugm * ribbon({{272, 266}, {278, 350}, {285, 440}, {282, 496}, {263, 530}}, 9):blur(8),
      {pile=jug_refl, hand="broad", tool="filbert 11", length={35, 85}, coverage=0.8,
       fill=true, angle=1.5, pressure={0.46, 0.36}, clip=true, seed=1077})
work(jugm * ellipse(241, 246, 25, 6), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.2, pressure={0.6, 0.5}, clip=true, seed=1079})
print("cloth and bottle re-laid")

--@ chunk 83
castm = (ribbon({{268, 536}, {350, 572}, {432, 604}}, 42):blur(19)
        + ribbon({{392, 606}, {502, 642}, {644, 622}}, 36):blur(21)
        + ellipse(704, 618, 92, 24):blur(20)
        + ellipse(398, 680, 58, 15):blur(13)
        + ellipse(778, 614, 54, 15):blur(12)) * tablefree
work(castm, {pile=cast, hand="broad", tool="filbert 24", length={90, 190},
      coverage=1.4, fill=true, angle=-0.22, pressure={0.5, 0.4}, dips={3, 0.8, 0.2},
      clip=true, edge="firm", seed=1081})
contact = (ribbon({{184, 546}, {235, 559}, {286, 549}}, 17):blur(6)
         + ribbon({{388, 612}, {500, 645}, {636, 623}}, 15):blur(8)
         + ellipse(350, 689, 45, 9):blur(5)
         + ellipse(730, 627, 41, 9):blur(5)) * tablefree
work(contact, {pile=jug_dk2, hand="broad", tool="filbert 18", length={60, 140},
      coverage=1.5, fill=true, angle=-0.1, pressure={0.5, 0.42}, clip=true, edge="firm", seed=1083})
print("cast shadows and contacts laid")

--@ chunk 84
upper = wallfree * rect(-10, -20, 1020, 300):blur(100)
work(upper, {pile=wall_deep, hand="broad", tool="filbert 24", length={90, 190},
      coverage=1.8, fill=true, angle=-0.07, pressure={0.54, 0.44}, dips={3, 0.85, 0.2},
      clip=true, edge="firm", seed=1091})
right = wallfree * ellipse(1030, 260, 380, 380):blur(170)
work(right, {pile=wall_deep, hand="broad", tool="filbert 24", length={90, 190},
      coverage=1.7, fill=true, angle=-0.07, pressure={0.54, 0.44}, dips={3, 0.85, 0.2},
      clip=true, edge="firm", seed=1093})
work(tablefree * rect(-20, 650, 1040, 120):blur(75), {pile=table_deep, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=1.5, fill=true, angle=-0.05,
      pressure={0.52, 0.42}, clip=true, edge="firm", seed=1095})
seam = (ribbon({{-10, 476}, {200, 472}, {480, 468}, {740, 463}, {1010, 458}}, 13):blur(7)
      * (everywhere() - jugm - bowlm - q1 - q2 - q3 - q4 - clothm))
work(seam, {pile=jug_dk2, hand="broad", tool="filbert 16", length={60, 140},
      coverage=1.5, fill=true, angle=0.02, pressure={0.5, 0.42},
      clip=true, edge="firm", seed=1097})
print("background structured")

--@ chunk 85
work(wallfree, {pile=wall_dk, hand="broad", tool="filbert 24", length={90, 190},
      coverage=2.6, fill=true, angle=-0.07, pressure={0.56, 0.46}, dips={3, 0.9, 0.15},
      clip=true, edge="soft", seed=1101})
blend(wallfree, {angle=0.05})
work(tablefree * rect(-20, 690, 1040, 90):blur(60), {pile=table_deep, hand="broad",
      tool="filbert 24", length={90, 190}, coverage=1.4, fill=true, angle=-0.05,
      pressure={0.52, 0.42}, clip=true, edge="soft", seed=1103})
print("wall flattened")

--@ chunk 86
work(jugm, {pile=jug_cover, hand="broad", tool="filbert 16", length={50, 120},
      coverage=3.0, fill=true, angle=1.5, pressure={0.64, 0.54}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1111})
work(jugm * ribbon({{213, 262}, {216, 330}, {203, 404}, {188, 466}, {191, 524}}, 12):blur(5),
      {pile=jug_hi, hand="broad", tool="filbert 9", length={30, 70}, coverage=1.2,
       fill=true, angle=1.5, pressure={0.55, 0.45}, clip=true, seed=1113})
work(jugm * ribbon({{272, 266}, {278, 350}, {285, 440}, {282, 496}, {263, 530}}, 9):blur(8),
      {pile=jug_refl, hand="broad", tool="filbert 11", length={35, 85}, coverage=0.8,
       fill=true, angle=1.5, pressure={0.46, 0.36}, clip=true, seed=1115})
work(jugm * ellipse(241, 246, 25, 6), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.2, pressure={0.6, 0.5}, clip=true, seed=1117})

-- the hot lights, last of all
hot = pile{{"lead white", 3.5}, {"cadmium yellow", 1.5},
           {"transparent oxide yellow", 1}, {"yellow ochre", 0.5}}
ab = brush{kind="filbert", width=7}
ab:load(hot, 0.92)
ab:stroke({{438, 456}, {450, 444}, {466, 439}}, {pressure={0.4, 0.68, 0.26}})
ab:reload(hot, 0.9)
ab:stroke({{558, 470}, {570, 459}, {583, 456}}, {pressure={0.36, 0.6, 0.22}})
ab:reload(hot, 0.92)
ab:stroke({{322, 624}, {336, 610}, {354, 606}}, {pressure={0.45, 0.7, 0.28}})
ab:reload(hot, 0.9)
ab:stroke({{702, 578}, {715, 566}, {731, 563}}, {pressure={0.36, 0.58, 0.22}})
rb2 = brush{kind="filbert", width=9}
rb2:load(pile{{"lead white", 3}, {"yellow ochre", 2.5}, {"burnt sienna", 1},
               {"raw umber", 1.2}}, 0.9)
rb2:stroke({{382, 468}, {412, 452}, {446, 440}, {476, 434}},
           {pressure={0.4, 0.58, 0.5, 0.26}})
print("bottle and final lights")

--@ chunk 87
cloth_night = pile{{"raw umber", 4.2}, {"burnt sienna", 1.4}, {"lead white", 0.35},
                   {"ultramarine blue", 0.35}}
cloth_soft = pile{{"lead white", 1.6}, {"raw umber", 2.6}, {"yellow ochre", 1.2},
                  {"ultramarine blue", 0.4}}
work(clothm, {pile=cloth_night, hand="broad", tool="filbert 18", length={60, 150},
      coverage=3.0, fill=true, angle=-0.28, pressure={0.62, 0.52}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1121})
work(clothm * ribbon({{812, 640}, {870, 656}, {928, 662}, {980, 654}}, 22):blur(9),
      {pile=jug_dk2, hand="broad", tool="filbert 16", length={50, 120}, coverage=1.6,
       fill=true, angle=-0.35, pressure={0.58, 0.48}, clip=true, seed=1123})
work(clothm * ribbon({{872, 556}, {922, 578}, {978, 592}, {1042, 596}}, 30):blur(12),
      {pile=jug_dk2, hand="broad", tool="filbert 16", length={50, 120}, coverage=1.6,
       fill=true, angle=-0.4, pressure={0.58, 0.48}, clip=true, seed=1125})
work(clothm * ribbon({{800, 596}, {858, 614}, {916, 624}, {968, 616}}, 15):blur(5),
      {pile=cloth_soft, hand="broad", tool="filbert 11", length={35, 85}, coverage=1.9,
       fill=true, angle=-0.4, pressure={0.5, 0.4}, clip=true, seed=1127})
work(clothm * ribbon({{818, 572}, {872, 588}, {930, 598}}, 10):blur(5),
      {pile=cloth_soft, hand="broad", tool="filbert 9", length={30, 70}, coverage=1.6,
       fill=true, angle=-0.4, pressure={0.48, 0.38}, clip=true, seed=1129})
print("cloth taken down into the dark")

--@ chunk 88
nz = noise{seed=5, octaves=3, period=190, persistence=0.55}
wall_sc = pile{{"raw umber", 3}, {"green earth", 1.5}, {"lead white", 0.6}, {"burnt sienna", 0.8}}
table_sc = pile{{"raw umber", 2.6}, {"burnt sienna", 2}, {"yellow ochre", 0.8}, {"lead white", 0.4}}

stipple(wallfree, {pile=wall_sc, width=11, tool="stippler 11",
      coverage=function(x, y) return 0.14 + 0.20 * nz:at01(x, y) end,
      pressure={0.26, 0.08}, dips={30, 0.3, 0.6}, cluster={0.7, 70},
      feather=0.7, seed=1131})
stipple(tablefree, {pile=table_sc, width=10, tool="stippler 10",
      coverage=function(x, y) return 0.12 + 0.18 * nz:at01(x + 500, y) end,
      pressure={0.26, 0.08}, dips={30, 0.3, 0.6}, cluster={0.7, 60},
      feather=0.7, seed=1133})
print("broken brushwork into wall and table")

--@ chunk 89
rim_lt = pile{{"lead white", 3.4}, {"yellow ochre", 2.4}, {"burnt sienna", 0.8}}
rim_dk = pile{{"burnt sienna", 2.6}, {"raw umber", 3}, {"bone black", 1}}

b1 = brush{kind="filbert", width=7}
b1:load(rim_lt, 0.9)
b1:stroke({{366, 476}, {400, 452}, {444, 436}, {492, 429}},
          {pressure={0.3, 0.55, 0.5, 0.3}})
b1:reload(rim_lt, 0.85)
b1:stroke({{512, 430}, {560, 432}, {604, 438}}, {pressure={0.45, 0.4, 0.2}})
b1:reload(rim_lt, 0.8)
b1:stroke({{624, 444}, {654, 460}}, {pressure={0.35, 0.2}})

b2 = brush{kind="rigger", width=5, point=0.85}
b2:load(rim_dk, 0.8)
b2:stroke({{624, 446}, {660, 464}, {686, 498}}, {pressure={0.35, 0.45, 0.3}})
b2:reload(rim_dk, 0.75)
b2:stroke({{683, 512}, {676, 546}, {660, 574}}, {pressure={0.4, 0.45, 0.25}})

-- the bottle takes a glint on the shoulder and a narrow reflection
b3 = brush{kind="rigger", width=6, point=0.9}
b3:load(pile{{"green earth", 3}, {"lead white", 2.4}, {"yellow ochre", 1}}, 0.85)
b3:stroke({{232, 266}, {226, 320}}, {pressure={0.4, 0.25}})
b3:reload(pile{{"lead white", 2.4}, {"yellow ochre", 1.6}, {"raw umber", 1.6}}, 0.8)
b3:stroke({{271, 300}, {275, 366}, {281, 430}}, {pressure={0.3, 0.45, 0.25}})
b3:reload(pile{{"lead white", 2.4}, {"yellow ochre", 1.6}, {"raw umber", 1.6}}, 0.75)
b3:stroke({{186, 452}, {190, 500}, {199, 528}}, {pressure={0.35, 0.4, 0.2}})

-- a specular on the two loose quinces
b4 = brush{kind="filbert", width=6}
b4:load(hot, 0.85)
b4:stroke({{330, 616}, {342, 606}}, {pressure={0.6, 0.2}})
b4:reload(hot, 0.8)
b4:stroke({{712, 572}, {722, 564}}, {pressure={0.55, 0.18}})
print("drawn marks on bowl and bottle")

--@ chunk 90
work(bowlm * ellipse(672, 468, 115, 95):blur(36), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.8, fill=true,
      angle=-0.4, pressure={0.6, 0.5}, clip=true, edge="soft", seed=1141})
work(bowlm * ribbon({{496, 432}, {560, 434}, {616, 442}}, 13):blur(6),
      {pile=bowl_lt, hand="broad", tool="filbert 10", length={30, 70}, coverage=1.0,
       fill=true, angle=-0.6, pressure={0.48, 0.4}, clip=true, seed=1143})

glz = pile{{"raw umber", 3}, {"burnt sienna", 1.5}, {"yellow ochre", 0.8}, medium=0.35}
work(everywhere(), {pile=glz, hand="broad", tool="filbert 26", length={110, 220},
      coverage=0.8, fill=true, angle=-0.06, pressure={0.44, 0.34},
      dips={4, 0.7, 0.35}, seed=1145})
print("unifying glaze")

--@ chunk 91
work(wallfree, {pile=wall_dk, hand="broad", tool="filbert 24", length={90, 190},
      coverage=3.2, fill=true, angle=-0.07, pressure={0.64, 0.54}, dips={2, 0.98, 0.05},
      clip=true, edge="soft", seed=1151})
work(tablefree, {pile=table_dark, hand="broad", tool="filbert 24", length={90, 190},
      coverage=3.2, fill=true, angle=-0.05, pressure={0.64, 0.54}, dips={2, 0.98, 0.05},
      clip=true, edge="soft", seed=1153})
print("wall and table re-covered")

--@ chunk 92
work(jugm, {pile=jug_cover, hand="broad", tool="filbert 16", length={50, 120},
      coverage=3.2, fill=true, angle=1.5, pressure={0.66, 0.56}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1161})
work(jugm * ribbon({{213, 262}, {216, 330}, {203, 404}, {188, 466}, {191, 524}}, 12):blur(5),
      {pile=jug_hi, hand="broad", tool="filbert 9", length={30, 70}, coverage=1.2,
       fill=true, angle=1.5, pressure={0.55, 0.45}, clip=true, seed=1163})
work(jugm * ribbon({{272, 266}, {278, 350}, {285, 440}, {282, 496}, {263, 530}}, 9):blur(8),
      {pile=jug_refl, hand="broad", tool="filbert 11", length={35, 85}, coverage=0.85,
       fill=true, angle=1.5, pressure={0.48, 0.38}, clip=true, seed=1165})
work(jugm * ellipse(241, 246, 25, 6), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.4, pressure={0.6, 0.5}, clip=true, seed=1167})

work(bowlfront, {pile=bowl_dd, hand="broad", tool="filbert 20", length={70, 160},
      coverage=3.2, fill=true, angle=-0.35, pressure={0.64, 0.54}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1171})
work(bowlfront * ellipse(424, 516, 175, 195):blur(78), {pile=bowl_mid, hand="broad",
      tool="filbert 18", length={60, 150}, coverage=1.7, fill=true, angle=-0.45,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=1173})
work(bowlfront * ellipse(700, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=1.9, fill=true, angle=-0.4,
      pressure={0.62, 0.52}, clip=true, edge="soft", seed=1175})
work(bowlfront * ellipse(490, 634, 240, 82):blur(52), {pile=bowl_dk, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=1.5, fill=true, angle=-0.2,
      pressure={0.62, 0.52}, clip=true, edge="soft", seed=1177})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=2.8, fill=true, angle=-0.3, pressure={0.62, 0.52}, clip=true, edge="found", seed=1179})
print("bottle and bowl re-covered")

--@ chunk 93
paint_fruit(q1, 466, 468, 42, 1181)
paint_fruit(q2, 583, 478, 35, 1191)
paint_fruit(q3, 350, 643, 47, 1201)
paint_fruit(q4, 730, 592, 41, 1211)
work(bowlfront * rimfront, {pile=bowl_mid, hand="broad", tool="filbert 14", length={40, 100},
      coverage=1.9, fill=true, angle=-0.3, pressure={0.6, 0.5}, clip=true, edge="found", seed=1221})
work(bowlfront * ribbon({{368, 476}, {412, 450}, {466, 435}, {520, 431}}, 15):blur(5),
      {pile=bowl_lt, hand="broad", tool="filbert 11", length={35, 85}, coverage=1.7,
       fill=true, angle=-0.65, pressure={0.55, 0.45}, clip=true, seed=1223})

work(clothm, {pile=cloth_night, hand="broad", tool="filbert 18", length={60, 150},
      coverage=3.2, fill=true, angle=-0.28, pressure={0.64, 0.54}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1231})
work(clothm * ribbon({{812, 640}, {870, 656}, {928, 662}, {980, 654}}, 22):blur(9),
      {pile=jug_dk2, hand="broad", tool="filbert 16", length={50, 120}, coverage=1.7,
       fill=true, angle=-0.35, pressure={0.6, 0.5}, clip=true, seed=1233})
work(clothm * ribbon({{800, 596}, {858, 614}, {916, 624}, {968, 616}}, 15):blur(5),
      {pile=cloth_soft, hand="broad", tool="filbert 11", length={35, 85},
       coverage=1.9, fill=true, angle=-0.4, pressure={0.52, 0.42}, clip=true, seed=1235})
work(clothm * ribbon({{818, 572}, {872, 588}, {930, 598}}, 10):blur(5),
      {pile=cloth_soft, hand="broad", tool="filbert 9", length={30, 70}, coverage=1.6,
       fill=true, angle=-0.4, pressure={0.5, 0.4}, clip=true, seed=1237})
print("fruit and cloth re-covered")

--@ chunk 94
work(wallfree, {pile=wall_dk, hand="broad", tool="filbert 26", length={110, 220},
      coverage=4.0, fill=true, angle=-0.07, pressure={0.78, 0.68}, dips={2, 1.0, 0.0},
      clip=true, edge="soft", seed=1241})
work(tablefree, {pile=table_dark, hand="broad", tool="filbert 26", length={110, 220},
      coverage=4.0, fill=true, angle=-0.05, pressure={0.78, 0.68}, dips={2, 1.0, 0.0},
      clip=true, edge="soft", seed=1243})
print("heavy coats")

--@ chunk 95
function paint_fruit(m, cx, cy, r, seed)
  local sh = m * ellipse(cx + r * 0.62, cy + r * 0.6, r * 0.95, r * 0.95):blur(r * 0.4)
  local lt = m * ellipse(cx - r * 0.46, cy - r * 0.5, r * 0.5, r * 0.44):blur(r * 0.45)
  work(m, {pile=fruit_base, hand="broad", tool="filbert 12", length={35, 80},
        coverage=3.4, fill=true, angle=-0.5, pressure={0.66, 0.56}, dips={2, 0.98, 0.05},
        clip=true, edge="found", seed=seed})
  work(sh, {pile=fruit_shadow, hand="broad", tool="filbert 10", length={25, 60},
        coverage=2.6, fill=true, angle=-0.55, pressure={0.66, 0.56}, dips={2, 0.98, 0.05},
        clip=true, edge="found", seed=seed + 1})
  work(lt, {pile=fruit_light, hand="broad", tool="filbert 9", length={20, 48},
        coverage=1.9, fill=true, angle=-0.6, pressure={0.56, 0.46}, dips={2, 0.9, 0.15},
        clip=true, edge="found", seed=seed + 2})
end
work(jugm, {pile=jug_cover, hand="broad", tool="filbert 16", length={50, 120},
      coverage=3.4, fill=true, angle=1.5, pressure={0.68, 0.58}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1251})
work(jugm * ribbon({{213, 262}, {216, 330}, {203, 404}, {188, 466}, {191, 524}}, 12):blur(5),
      {pile=jug_hi, hand="broad", tool="filbert 9", length={30, 70}, coverage=1.2,
       fill=true, angle=1.5, pressure={0.55, 0.45}, clip=true, edge="found", seed=1253})
work(jugm * ellipse(241, 246, 25, 6), {pile=jug_dk2, hand="detail", tool="round 5",
      coverage=2.4, pressure={0.6, 0.5}, clip=true, edge="found", seed=1255})
work(bowlfront, {pile=bowl_dd, hand="broad", tool="filbert 20", length={70, 160},
      coverage=3.4, fill=true, angle=-0.35, pressure={0.66, 0.56}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1261})
work(bowlfront * ellipse(424, 516, 175, 195):blur(78), {pile=bowl_mid, hand="broad",
      tool="filbert 18", length={60, 150}, coverage=1.8, fill=true, angle=-0.45,
      pressure={0.6, 0.5}, clip=true, edge="soft", seed=1263})
work(bowlfront * ellipse(700, 578, 200, 190):blur(80), {pile=bowl_dk, hand="broad",
      tool="filbert 20", length={70, 160}, coverage=2.0, fill=true, angle=-0.4,
      pressure={0.62, 0.52}, clip=true, edge="soft", seed=1265})
work(bowl_in, {pile=bowl_in_dark, hand="broad", tool="filbert 16", length={50, 120},
      coverage=3.0, fill=true, angle=-0.3, pressure={0.64, 0.54}, clip=true, edge="found", seed=1267})
paint_fruit(q1, 466, 468, 42, 1271)
paint_fruit(q2, 583, 478, 35, 1281)
paint_fruit(q3, 350, 643, 47, 1291)
paint_fruit(q4, 730, 592, 41, 1301)
work(bowlfront * rimfront, {pile=bowl_mid, hand="broad", tool="filbert 14", length={40, 100},
      coverage=2.0, fill=true, angle=-0.3, pressure={0.6, 0.5}, clip=true, edge="found", seed=1311})
work(clothm, {pile=cloth_night, hand="broad", tool="filbert 18", length={60, 150},
      coverage=3.4, fill=true, angle=-0.28, pressure={0.66, 0.56}, dips={2, 0.98, 0.05},
      clip=true, edge="found", seed=1321})
work(clothm * ribbon({{800, 596}, {858, 614}, {916, 624}, {968, 616}}, 15):blur(5),
      {pile=cloth_soft, hand="broad", tool="filbert 11", length={35, 85},
       coverage=1.9, fill=true, angle=-0.4, pressure={0.52, 0.42}, clip=true, edge="found", seed=1323})
print("objects re-laid, strictly clipped")

--@ chunk 96
fruit_sh2 = pile{{"burnt sienna", 3}, {"raw umber", 2.5}, {"lead white", 0.6},
                  {"bone black", 0.8}}
function paint_fruit2(m, cx, cy, r, seed)
  local sh = m * ellipse(cx + r * 0.55, cy + r * 0.6, r * 1.0, r * 1.0):blur(r * 0.5)
  local lt = m * ellipse(cx - r * 0.45, cy - r * 0.5, r * 0.62, r * 0.56):blur(r * 0.52)
  work(m, {pile=fruit_base, hand="broad", tool="filbert 14", length={45, 100},
        coverage=3.4, fill=true, angle=-0.5, pressure={0.66, 0.56}, dips={2, 0.98, 0.05},
        clip=true, edge="found", seed=seed})
  work(sh, {pile=fruit_sh2, hand="broad", tool="filbert 14", length={45, 100},
        coverage=1.6, fill=true, angle=-0.55, pressure={0.6, 0.5}, dips={2, 0.9, 0.12},
        clip=true, edge="found", seed=seed + 1})
  work(lt, {pile=fruit_light, hand="broad", tool="filbert 12", length={40, 90},
        coverage=1.7, fill=true, angle=-0.6, pressure={0.58, 0.48}, dips={2, 0.9, 0.12},
        clip=true, edge="found", seed=seed + 2})
  blend(m, {angle=-0.5})
end
paint_fruit2(q1, 466, 468, 42, 1331)
paint_fruit2(q2, 583, 478, 35, 1341)
paint_fruit2(q3, 350, 643, 47, 1351)
paint_fruit2(q4, 730, 592, 41, 1361)
print("quinces turned smoothly")

--@ chunk 97
fruit_sh3 = pile{{"burnt sienna", 3.2}, {"raw umber", 2.6}, {"bone black", 1},
                  {"ultramarine blue", 0.4}}
function shade_fruit(m, cx, cy, r, seed)
  local sh = m * ellipse(cx + r * 0.62, cy + r * 0.62, r * 0.92, r * 0.92):blur(r * 0.34)
  local core = m * ellipse(cx + r * 0.5, cy + r * 0.85, r * 0.75, r * 0.5):blur(r * 0.3)
  work(sh, {pile=fruit_sh3, hand="broad", tool="filbert 12", length={35, 85},
        coverage=2.2, fill=true, angle=-0.55, pressure={0.62, 0.52}, dips={2, 0.95, 0.1},
        clip=true, edge="found", seed=seed})
  work(core, {pile=jug_dk2, hand="broad", tool="filbert 10", length={30, 70},
        coverage=1.4, fill=true, angle=-0.5, pressure={0.58, 0.5}, dips={2, 0.9, 0.15},
        clip=true, edge="found", seed=seed + 1})
end
shade_fruit(q1, 466, 468, 42, 1371)
shade_fruit(q2, 583, 478, 35, 1381)
shade_fruit(q3, 350, 643, 47, 1391)
shade_fruit(q4, 730, 592, 41, 1401)

contact = (ellipse(352, 689, 44, 11):blur(9) + ellipse(732, 630, 40, 10):blur(8)) * tablefree
work(contact, {pile=jug_dk2, hand="broad", tool="filbert 16", length={50, 120},
      coverage=1.6, fill=true, angle=-0.1, pressure={0.52, 0.44},
      clip=true, edge="firm", seed=1411})
print("shadows on the fruit")

--@ chunk 98
fruit_sh4 = pile{{"burnt sienna", 3}, {"raw umber", 3}, {"bone black", 1.2}}
function shade_fruit2(m, cx, cy, r, seed)
  local sh = m * ellipse(cx + r * 0.5, cy + r * 0.55, r * 1.05, r * 1.05):blur(r * 0.55)
  work(sh, {pile=fruit_sh4, hand="broad", tool="filbert 12", length={35, 85},
        coverage=1.7, fill=true, angle=-0.55, pressure={0.6, 0.5}, dips={2, 0.92, 0.12},
        clip=true, edge="found", seed=seed})
end
shade_fruit2(q1, 466, 468, 42, 1421)
shade_fruit2(q2, 583, 478, 35, 1431)
shade_fruit2(q3, 350, 643, 47, 1441)
shade_fruit2(q4, 730, 592, 41, 1451)
print("warm shadows on the fruit")

--@ chunk 99
function fruit_soft(m, cx, cy, r, seed)
  local sh = m * ellipse(cx + r * 0.52, cy + r * 0.58, r * 1.05, r * 1.05):blur(r * 0.72)
  local lt = m * ellipse(cx - r * 0.44, cy - r * 0.5, r * 0.66, r * 0.6):blur(r * 0.66)
  work(m, {pile=fruit_base, hand="broad", tool="filbert 16", length={60, 140},
        coverage=3.6, fill=true, angle=-0.5, pressure={0.66, 0.56}, dips={2, 0.98, 0.05},
        clip=true, edge="found", seed=seed})
  work(sh, {pile=fruit_sh4, hand="broad", tool="filbert 18", length={60, 140},
        coverage=1.9, fill=true, angle=-0.55, pressure={0.6, 0.5}, dips={2, 0.92, 0.12},
        clip=true, edge="found", seed=seed + 1})
  work(lt, {pile=fruit_light, hand="broad", tool="filbert 16", length={55, 125},
        coverage=1.8, fill=true, angle=-0.6, pressure={0.58, 0.48}, dips={2, 0.9, 0.12},
        clip=true, edge="found", seed=seed + 2})
end
fruit_soft(q1, 466, 468, 42, 1461)
fruit_soft(q2, 583, 478, 35, 1471)
fruit_soft(q3, 350, 643, 47, 1481)
fruit_soft(q4, 730, 592, 41, 1491)
print("fruit, soft turns")

--@ chunk 100
work(bowl_in - q1 - q2, {pile=bowl_in_dark, hand="broad", tool="filbert 18",
      length={60, 140}, coverage=2.4, fill=true, angle=-0.3, pressure={0.62, 0.52},
      clip=true, edge="found", seed=1501})
work(clothm * rect(880, 600, 170, 110), {pile=cloth_night, hand="broad",
      tool="filbert 18", length={60, 140}, coverage=2.6, fill=true, angle=-0.28,
      pressure={0.62, 0.52}, clip=true, edge="found", seed=1503})
work(clothm * ribbon({{800, 596}, {858, 614}, {916, 624}, {968, 616}}, 15):blur(5),
      {pile=cloth_soft, hand="broad", tool="filbert 11", length={35, 85},
       coverage=1.8, fill=true, angle=-0.4, pressure={0.52, 0.42}, clip=true, edge="found", seed=1505})
print("bowl interior and cloth tidied")

--@ chunk 101
lightpool = poly({{-10, 522}, {210, 514}, {420, 500}, {505, 566}, {486, 760},
                   {-10, 760}}, true)
work(lightpool * tablefree, {pile=pool_light, hand="broad", tool="filbert 22",
      length={90, 190}, coverage=1.15, fill=true, angle=-0.04, pressure={0.52, 0.42},
      dips={4, 0.75, 0.28}, clip=true, edge="soft", seed=1511})
print("light on the table")

--@ chunk 102
work(lightpool * tablefree, {pile=table_dark, hand="broad", tool="filbert 24",
      length={90, 190}, coverage=3.4, fill=true, angle=-0.05, pressure={0.7, 0.6},
      dips={2, 0.98, 0.05}, clip=true, edge="soft", seed=1521})
pool2 = pile{{"raw umber", 3}, {"burnt sienna", 1.5}, {"yellow ochre", 1}, {"lead white", 1.2}}
work(lightpool * tablefree, {pile=pool2, hand="broad", tool="filbert 24", length={90, 190},
      coverage=2.4, fill=true, angle=-0.04, pressure={0.68, 0.58}, dips={2, 0.95, 0.08},
      clip=true, edge="soft", seed=1523})
print("light re-laid stiff")

--@ chunk 103
work(lightpool * tablefree, {pile=table_dark, hand="broad", tool="filbert 26",
      length={110, 220}, coverage=4.5, fill=true, angle=-0.05, pressure={0.82, 0.72},
      dips={2, 1.0, 0.0}, clip=true, edge="soft", seed=1531})
fruit_soft(q3, 350, 643, 47, 1541)
work(ellipse(352, 689, 44, 11):blur(9) * tablefree, {pile=jug_dk2, hand="broad",
      tool="filbert 16", length={50, 120}, coverage=1.5, fill=true, angle=-0.1,
      pressure={0.52, 0.44}, clip=true, edge="firm", seed=1551})
print("table evened, front quince restored")

--@ chunk 104
band = rect(420, 486, 170, 280) * tablefree
work(band, {pile=table_dark, hand="broad", tool="filbert 26", length={110, 220},
      coverage=3.0, fill=true, angle=-0.05, pressure={0.76, 0.66}, dips={2, 0.98, 0.05},
      clip=true, edge="soft", seed=1561})
print("table step evened out")

--@ chunk 105
local function stat(name, m)
  local x0,y0,x1,y1,s = 1e9,1e9,-1e9,-1e9,0
  for x = 0, W, 2 do for y = 0, H, 2 do
    local v = m:at(x, y)
    if v > 0.4 then
      if x < x0 then x0 = x end if x > x1 then x1 = x end
      if y < y0 then y0 = y end if y > y1 then y1 = y end
      s = s + 1
    end
  end end
  print(string.format("%-8s x %3d..%3d  y %3d..%3d  n=%d", name, x0, x1, y0, y1, s))
end
stat("q1", q1); stat("q2", q2); stat("q3", q3); stat("q4", q4)
stat("clothm", clothm); stat("bowlm", bowlm); stat("bowlfront", bowlfront)
stat("jugm", jugm)
print("drying 460,440 =", drying(460,440), "| 900,600 =", drying(900,600), "| 500,600 =", drying(500,600), "| 700,80 =", drying(700,80))

--@ chunk 106
cloth_sh = pile{{"raw umber", 3.4}, {"burnt sienna", 1.6}, {"lead white", 0.9}, {"ultramarine blue", 0.3}, medium=0}
cloth_md = pile{{"lead white", 1.8}, {"raw umber", 2.6}, {"yellow ochre", 1.1}, {"ultramarine blue", 0.3}, medium=0}
cloth_hi = pile{{"lead white", 3}, {"yellow ochre", 1.3}, {"raw umber", 1.5}, {"burnt sienna", 0.4}, medium=0}
print(cloth_sh) print(cloth_md) print(cloth_hi)
work(clothm, {hand="broad", tool="filbert 20", pile=cloth_sh, length={60,150},
               coverage=3.4, angle=0.12, pressure={0.55,0.4}, edge="found", clip=true})

--@ chunk 107
-- hand-drawn fold structure for the linen
crestA = ribbon({{808,594},{850,582},{900,588},{948,574},{1004,565}}, {5,15,16,12,4}) * clothm
crestB = ribbon({{828,644},{872,653},{928,656},{990,642}}, {4,12,13,5}) * clothm
valley = ribbon({{812,616},{858,610},{910,616},{958,606},{1004,600}}, {8,16,16,12,6}) * clothm
print(crestA:area(), crestB:area(), valley:area())
work(clothm, {hand="broad", tool="filbert 20", pile=cloth_md, length={60,140},
               coverage=2.1, angle=0.1, pressure={0.5,0.35}, edge="found", clip=true})
work(valley:soften(5), {hand="broad", tool="filbert 18", pile=cloth_sh, length={50,120},
               coverage=1.8, angle=0.08, pressure={0.45,0.3}, edge="lost", clip=true})

--@ chunk 108
work(crestA:soften(7), {hand="broad", tool="filbert 18", pile=cloth_hi, length={45,110},
            coverage=1.9, angle=0.06, pressure={0.45,0.3}, edge="lost", clip=true})
work(crestB:soften(6), {hand="broad", tool="filbert 16", pile=cloth_md, length={40,100},
            coverage=1.5, angle=0.06, pressure={0.4,0.28}, edge="lost", clip=true})
-- crisp small lights where the crests turn
hl = brush("rigger", 5, 0.8)
hl:load(cloth_hi, 0.7)
hl:stroke({{828,586},{858,578},{884,581}}, {pressure={0.55,0.15}, clip=crestA:soften(2)})
hl:stroke({{918,580},{944,570},{968,566}}, {pressure={0.5,0.12}, clip=crestA:soften(2)})
blend(clothm, {})

--@ chunk 109
-- crisp fold edges: linen turns on a line, not in a blur
fold1 = outline{{806,606},{850,599},{898,605},{946,595},{1004,586}, char="firm", seed=11}:mask()
fold2 = outline{{818,650},{864,661},{916,664},{968,652},{1004,642}, char="firm", seed=5}:mask()
tipL  = poly({{800,566},{824,570},{834,592},{828,620},{806,618},{798,592}}, true):grow(2) * clothm
print(fold1:area(), fold2:area(), tipL:area())
work(fold1, {hand="detail", tool="rigger 4", pile=cloth_sh, coverage=1.5,
             pressure={0.5,0.25}, edge="found", clip=true})
work(fold2, {hand="detail", tool="rigger 4", pile=cloth_sh, coverage=1.3,
             pressure={0.45,0.2}, edge="found", clip=true})
-- the plane that turns the light, just above each fold line
lit1 = ribbon({{812,600},{852,593},{898,599},{946,589},{1004,580}}, {6,12,12,9,4}) * clothm - fold1
work(lit1:soften(3), {hand="broad", tool="filbert 16", pile=cloth_hi, length={40,100},
            coverage=1.3, angle=0.05, pressure={0.42,0.26}, edge="soft", clip=true})
work(tipL, {hand="body", tool="filbert 8", pile=cloth_md, length={25,55},
            coverage=1.6, angle=1.2, edge="found", clip=true})

--@ chunk 110
-- the tip pass went wormy; cover and smooth it, and soften the streak edges
work(tipL, {hand="broad", tool="filbert 18", pile=cloth_md, length={50,120},
            coverage=2.4, angle=0.9, pressure={0.5,0.35}, edge="soft", clip=true})
work(tipL, {hand="broad", tool="filbert 18", pile=cloth_sh, length={40,90},
            coverage=1.2, angle=0.3, pressure={0.4,0.25}, edge="soft", clip=true})
blend(lit1:grow(6), {})
blend(tipL:grow(4), {})

--@ chunk 111
clothf = poly({{1000,536},{955,545},{922,538},{893,553},{858,546},{830,557},
                {810,572},{800,592},{806,612},{826,634},{852,652},{884,666},
                {922,674},{958,672},{1000,656}}, true):roughen(2.5, 46, 7, 0.4)
-- wipe the halo back to the table
work((clothm:grow(16) - clothf) - q4 - bowlm - jugm, {hand="broad", tool="filbert 20",
      pile=table_sc, length={60,140}, coverage=2.6, angle=0.05, pressure={0.5,0.35},
      edge="found", clip=true})
work(clothf, {hand="broad", tool="filbert 20", pile=cloth_md, length={60,140},
      coverage=2.3, angle=0.08, pressure={0.5,0.35}, edge="found", clip=true})
clothm = clothf
print("clothf area", clothf:area())
work(clothf, {hand="broad", tool="filbert 18", pile=cloth_sh, length={50,120},
      coverage=1.3, angle=0.1, pressure={0.42,0.26}, edge="lost", clip=true})

--@ chunk 112
vB = ribbon({{818,650},{864,661},{916,664},{968,652},{1004,642}}, {4,12,13,9,3}) * clothm
vA = ribbon({{808,604},{850,596},{898,602},{946,592},{1004,582}}, {6,12,12,9,3}) * clothm
cA = ribbon({{812,592},{852,585},{898,591},{946,581},{1004,572}}, {4,12,12,9,3}) * clothm
work(vB:soften(6), {hand="broad", tool="filbert 16", pile=cloth_md, length={40,100},
     coverage=1.5, angle=0.06, pressure={0.4,0.28}, edge="lost", clip=true})
work(vA:soften(7), {hand="broad", tool="filbert 16", pile=cloth_sh, length={45,110},
     coverage=1.7, angle=0.06, pressure={0.45,0.3}, edge="lost", clip=true})
work(cA:soften(6), {hand="broad", tool="filbert 16", pile=cloth_hi, length={40,100},
     coverage=1.6, angle=0.05, pressure={0.42,0.26}, edge="soft", clip=true})
hl:load(cloth_hi, 0.6)
hl:stroke({{836,581},{862,574}}, {pressure={0.5,0.12}, clip=clothm})
hl:stroke({{922,576},{946,568}}, {pressure={0.45,0.1}, clip=clothm})
blend(clothm, {})

--@ chunk 113
cloth_m2 = pile{{"lead white", 1.2}, {"raw umber", 3.2}, {"yellow ochre", 1.0}, {"ultramarine blue", 0.3}, medium=0}
cloth_h2 = pile{{"lead white", 2.2}, {"yellow ochre", 1.2}, {"raw umber", 1.8}, {"burnt sienna", 0.3}, medium=0}
cloth_d2 = pile{{"raw umber", 4.0}, {"burnt sienna", 1.8}, {"lead white", 0.5}, {"ultramarine blue", 0.4}, medium=0}
-- clear the whole zone back to the table
work(clothm:grow(24) - q4 - bowlm - jugm - q3, {hand="broad", tool="filbert 22",
     pile=table_sc, length={70,160}, coverage=2.8, angle=0.04, pressure={0.55,0.4},
     edge="found", clip=true})
work(clothf, {hand="broad", tool="filbert 20", pile=cloth_m2, length={60,140},
     coverage=2.2, angle=0.07, pressure={0.5,0.35}, edge="found", clip=true})
print("cleared")

--@ chunk 114
cloth_d3 = pile{{"raw umber", 3.6}, {"burnt sienna", 1.7}, {"lead white", 0.5}, {"ultramarine blue", 0.45}, medium=0}
cloth_m3 = pile{{"lead white", 0.9}, {"raw umber", 3.4}, {"yellow ochre", 1.1}, {"ultramarine blue", 0.3}, medium=0}
cloth_h3 = pile{{"lead white", 1.9}, {"yellow ochre", 1.2}, {"raw umber", 2.0}, {"burnt sienna", 0.4}, medium=0}
zone = (clothf:grow(30) - q4 - bowlm - jugm - q3) + (clothf - q4 - bowlm)
work(zone, {hand="broad", tool="filbert 22", pile=cloth_d3, length={80,180},
     coverage=2.6, angle=0.04, pressure={0.55,0.4}, edge="found", clip=true})
blend(clothf:grow(30) - q4 - bowlm - jugm - q3, {})

--@ chunk 115
cA = poly({{780,520},{1020,520},{1020,578},{922,596},{858,592},{780,608}}, true) * clothf
cB = poly({{780,608},{858,592},{922,596},{1020,578},{1020,610},{934,628},{868,624},{780,634}}, true) * clothf
cC = poly({{780,634},{868,624},{934,628},{1020,610},{1020,720},{780,720}}, true) * clothf
print("A",cA:area(),"B",cB:area(),"C",cC:area())
pl_back  = pile{{"lead white", 0.8}, {"raw umber", 3.2}, {"yellow ochre", 1.0}, {"ultramarine blue", 0.3}, medium=0}
pl_crest = pile{{"lead white", 1.8}, {"yellow ochre", 1.1}, {"raw umber", 2.0}, {"burnt sienna", 0.3}, medium=0}
pl_front = pile{{"raw umber", 4.0}, {"burnt sienna", 1.6}, {"ultramarine blue", 0.4}, {"lead white", 0.3}, medium=0}
work(cC:soften(5), {hand="broad", tool="filbert 20", pile=pl_front, length={60,140},
     coverage=2.4, angle=0.08, pressure={0.5,0.35}, edge="found", clip=true})
work(cA:soften(5), {hand="broad", tool="filbert 20", pile=pl_back, length={60,140},
     coverage=2.2, angle=0.08, pressure={0.5,0.35}, edge="found", clip=true})
work(cB:soften(4), {hand="broad", tool="filbert 18", pile=pl_crest, length={50,120},
     coverage=2.4, angle=0.06, pressure={0.5,0.35}, edge="found", clip=true})

--@ chunk 116
tbl_rt = pile{{"raw umber", 3.6}, {"burnt sienna", 1.8}, {"bone black", 1.4}, {"ultramarine blue", 0.4}, {"lead white", 0.3}, medium=0}
wall_rt = pile{{"raw umber", 4.0}, {"burnt sienna", 2.0}, {"bone black", 2.0}, {"lead white", 0.4}, medium=0}
clear = rect(760, 462, 244, 258) - q4:grow(5)
wallb = rect(760, 462, 244, 40) - q4:grow(5)
print(clear:area(), wallb:area())
work(clear, {hand="broad", tool="filbert 22", pile=tbl_rt, length={80,180},
     coverage=3.0, angle=0.03, pressure={0.55,0.4}, edge="found", clip=true})
work(wallb, {hand="broad", tool="filbert 22", pile=wall_rt, length={80,180},
     coverage=2.8, angle=0.03, pressure={0.55,0.4}, edge="found", clip=true})
blend(rect(752, 452, 252, 274) - q4:grow(6), {})

--@ chunk 117
rt_mid = pile{{"raw umber", 3.9}, {"burnt sienna", 1.8}, {"bone black", 2.0}, {"ultramarine blue", 0.45}, {"lead white", 0.25}, medium=0}
rt_dark = pile{{"raw umber", 4.4}, {"burnt sienna", 1.4}, {"bone black", 3.0}, {"ultramarine blue", 0.5}, {"lead white", 0.15}, medium=0}
g1 = (rect(756, 468, 250, 258):soften(30)) - q4:grow(5)
g2 = (rect(856, 480, 152, 240):soften(38)) - q4:grow(5)
work(g1, {hand="broad", tool="filbert 22", pile=rt_mid, length={80,180},
     coverage=2.2, angle=0.03, pressure={0.5,0.38}, edge="found", clip=true})
work(g2, {hand="broad", tool="filbert 22", pile=rt_dark, length={70,160},
     coverage=2.4, angle=0.03, pressure={0.5,0.38}, edge="found", clip=true})

--@ chunk 118
gz_rt = pile{{"raw umber", 4.0}, {"burnt sienna", 2.0}, {"bone black", 2.2}, {"ultramarine blue", 0.5}, {"lead white", 0.2}, medium=0.42}
work(rect(694, 424, 320, 306):soften(46) - q4:grow(6) - bowlm, {hand="glaze", pile=gz_rt,
     coverage=2.4, angle=0.03, pressure={0.4,0.3}, edge="found", clip=true})

--@ chunk 119
blend(rect(700, 430, 300, 300):soften(48) - q4:grow(8) - bowlm, {})

--@ chunk 120
-- knock all four fruit back to a dark warm olive, then build the light up again
fruit_dk = pile{{"bone black", 3.0}, {"raw umber", 3.0}, {"yellow ochre", 1.2}, {"green earth", 0.6}, {"lead white", 0.2}, medium=0}
allq = q1 + q2 + q3 + q4
print(allq:area())
work(allq, {hand="broad", tool="filbert 18", pile=fruit_dk, length={40,110},
     coverage=3.0, angle=1.2, pressure={0.55,0.4}, edge="found", clip=true})

--@ chunk 121
work(q3, {hand="broad", tool="filbert 9", pile=fruit_dk, length={22,50},
     coverage=6, angle=1.1, pressure={0.85,0.75}, dips={9, 0.9, 0.3}, edge="found", clip=true})

--@ chunk 122
tb = brush("filbert", 14, 1.0)
tb:load(fruit_dk, 1.0)
tb:stroke({{312,600},{400,610},{400,700},{310,690}}, {pressure=0.9, clip=q3})
print(tb:fullness())

--@ chunk 123
print("fullness before", tb:fullness())
tb:load(fruit_dk, 1.0)
tb:stroke({{300,600},{380,596},{410,640},{350,700},{306,660}}, {pressure=0.95})
tb:load(fruit_dk, 1.0)
tb:stroke({{430,600},{520,610},{540,660},{460,690}}, {pressure=0.95})
print("done")

--@ chunk 124
print(wait(3 * 24 * 60))
print("fruit", drying(350, 640), "| table", drying(500, 600), "| wall", drying(700, 120),
      "| right", drying(900, 600), "| bowl", drying(500, 500))

--@ chunk 125
work(q3, {hand="broad", tool="filbert 9", pile=fruit_dk, length={22,50},
     coverage=4, angle=1.1, pressure={0.8,0.65}, dips={10, 0.95, 0.25}, edge="found", clip=true})
work(q1, {hand="broad", tool="filbert 8", pile=fruit_dk, length={20,44},
     coverage=4, angle=1.1, pressure={0.8,0.65}, dips={10, 0.95, 0.25}, edge="found", clip=true})
work(q2, {hand="broad", tool="filbert 8", pile=fruit_dk, length={18,40},
     coverage=4, angle=1.1, pressure={0.8,0.65}, dips={10, 0.95, 0.25}, edge="found", clip=true})
work(q4, {hand="broad", tool="filbert 9", pile=fruit_dk, length={22,50},
     coverage=4, angle=1.1, pressure={0.8,0.65}, dips={10, 0.95, 0.25}, edge="found", clip=true})

--@ chunk 126
for i = 1, 3 do
  work(q1, {hand="broad", tool="filbert 8", pile=fruit_dk, length={20,44}, coverage=3.5,
            angle=0.3 + i, pressure={0.85,0.7}, dips={10, 0.95, 0.2}, edge="found", clip=true})
  work(q2, {hand="broad", tool="filbert 8", pile=fruit_dk, length={18,40}, coverage=3.5,
            angle=0.3 + i, pressure={0.85,0.7}, dips={10, 0.95, 0.2}, edge="found", clip=true})
  work(q3, {hand="broad", tool="filbert 9", pile=fruit_dk, length={22,50}, coverage=3.5,
            angle=0.3 + i, pressure={0.85,0.7}, dips={10, 0.95, 0.2}, edge="found", clip=true})
  work(q4, {hand="broad", tool="filbert 9", pile=fruit_dk, length={22,50}, coverage=3.5,
            angle=0.3 + i, pressure={0.85,0.7}, dips={10, 0.95, 0.2}, edge="found", clip=true})
end
print("dark done")

--@ chunk 127
-- cover the leftover yellow halo at the silhouettes
allq = (q1:grow(4) + q2:grow(4) + q3:grow(4) + q4:grow(4))
work(allq, {hand="broad", tool="filbert 9", pile=fruit_dk, length={22,48},
     coverage=3, angle=0.8, pressure={0.85,0.7}, edge="found", clip=q1:grow(4)+q2:grow(4)+q3:grow(4)+q4:grow(4)})

q1 = q1:grow(4); q2 = q2:grow(4); q3 = q3:grow(4); q4 = q4:grow(4)
core = pile{{"burnt sienna", 3.0}, {"raw umber", 2.6}, {"bone black", 1.3}, {"transparent oxide yellow", 0.4}, medium=0}
c1 = ellipse(482, 488, 30, 28) * q1
c2 = ellipse(596, 494, 26, 25) * q2
c3 = ellipse(370, 664, 34, 31) * q3
c4 = ellipse(746, 612, 28, 26) * q4
print(c1:area(), c2:area(), c3:area(), c4:area())
for _, m in ipairs({c1, c2, c3, c4}) do
  work(m:soften(12), {hand="broad", tool="filbert 9", pile=core, length={20,50},
       coverage=2.2, angle=1.2, pressure={0.7,0.55}, edge="found", clip=true})
end

--@ chunk 128
print(drying(466,468))
litm = pile{{"yellow ochre", 4.0}, {"cadmium yellow", 1.6}, {"transparent oxide yellow", 0.8}, {"lead white", 0.8}, medium=0}
l1 = ellipse(452, 451, 32, 30) * q1
l2 = ellipse(570, 463, 26, 25) * q2
l3 = ellipse(334, 624, 36, 34) * q3
l4 = ellipse(714, 575, 31, 29) * q4
for _, m in ipairs({l1, l2, l3, l4}) do
  work(m:soften(11), {hand="broad", tool="filbert 9", pile=litm, length={20,50},
       coverage=2.4, angle=1.1, pressure={0.7,0.55}, edge="found", clip=true})
end

--@ chunk 129
blend(q1, {}); blend(q2, {}); blend(q3, {}); blend(q4, {})
core2 = pile{{"raw umber", 3.0}, {"bone black", 2.2}, {"green earth", 1.2}, {"burnt sienna", 1.0}, medium=0}
litm2 = pile{{"yellow ochre", 4.2}, {"cadmium yellow", 1.4}, {"transparent oxide yellow", 0.7}, {"lead white", 0.7}, medium=0.12}
for _, m in ipairs({c1, c2, c3, c4}) do
  work(m:soften(14), {hand="glaze", pile=core2, coverage=2.2, angle=1.2,
       pressure={0.45,0.3}, edge="found", clip=true})
end
for _, m in ipairs({l1, l2, l3, l4}) do
  work(m:soften(13), {hand="glaze", pile=litm2, coverage=2.0, angle=1.1,
       pressure={0.45,0.3}, edge="found", clip=true})
end
print(drying(466,468))

--@ chunk 130
pz1 = pile{{"yellow ochre", 4.0}, {"raw umber", 1.7}, {"cadmium yellow", 0.8}, {"transparent oxide yellow", 0.4}, {"lead white", 0.3}, medium=0}
pz2 = pile{{"yellow ochre", 3.6}, {"cadmium yellow", 1.4}, {"transparent oxide yellow", 0.6}, {"lead white", 0.7}, medium=0}
pz3 = pile{{"yellow ochre", 2.6}, {"cadmium yellow", 2.2}, {"transparent oxide yellow", 0.5}, {"lead white", 1.2}, medium=0}
pz4 = pile{{"lead white", 2.2}, {"cadmium yellow", 3.0}, {"transparent oxide yellow", 0.4}, {"yellow ochre", 0.8}, medium=0}
pc1 = pile{{"raw umber", 3.0}, {"green earth", 1.4}, {"bone black", 1.4}, {"yellow ochre", 1.2}, medium=0}
pc2 = pile{{"bone black", 2.4}, {"raw umber", 2.6}, {"green earth", 1.2}, medium=0}
-- core shadow on q3
work((ellipse(370, 664, 34, 31) * q3):soften(13), {hand="broad", tool="filbert 7",
     pile=pc1, length={14,32}, coverage=3, angle=0.9, curve={5,3}, orient="across",
     pressure={0.5,0.38}, edge="found", clip=true})
work((ellipse(378, 672, 20, 18) * q3):soften(10), {hand="broad", tool="filbert 7",
     pile=pc2, length={12,26}, coverage=2.6, angle=0.9, curve={5,3}, orient="across",
     pressure={0.5,0.38}, edge="found", clip=true})
blend(q3, {})
-- light on q3, four soft steps
work((ellipse(334, 624, 38, 34) * q3):soften(12), {hand="broad", tool="filbert 7",
     pile=pz1, length={14,32}, coverage=3, angle=0.9, curve={5,3}, orient="across",
     pressure={0.5,0.38}, edge="found", clip=true})
work((ellipse(336, 626, 29, 26) * q3):soften(11), {hand="broad", tool="filbert 7",
     pile=pz2, length={14,30}, coverage=2.8, angle=0.9, curve={5,3}, orient="across",
     pressure={0.5,0.38}, edge="found", clip=true})
blend(q3, {})
work((ellipse(338, 628, 20, 18) * q3):soften(9), {hand="broad", tool="filbert 6",
     pile=pz3, length={12,26}, coverage=2.6, angle=0.9, curve={4,3}, orient="across",
     pressure={0.5,0.38}, edge="found", clip=true})
work((ellipse(340, 630, 11, 10) * q3):soften(7), {hand="broad", tool="filbert 6",
     pile=pz4, length={10,22}, coverage=2.4, angle=0.9, curve={4,3}, orient="across",
     pressure={0.5,0.38}, edge="found", clip=true})
blend(q3, {})

--@ chunk 131
psh = pile{{"raw umber", 3.4}, {"green earth", 1.5}, {"bone black", 1.6}, {"yellow ochre", 1.0}, medium=0}
pco = pile{{"raw umber", 2.6}, {"green earth", 1.2}, {"bone black", 2.4}, medium=0}
local F = {
 {q=q1, cx=466, cy=468, sx=18, sy=22, sr=36, cr=20, lx=-15, ly=-17, lr={32,23,15,8}},
 {q=q2, cx=582, cy=478, sx=16, sy=19, sr=31, cr=17, lx=-13, ly=-15, lr={27,20,13,7}},
 {q=q3, cx=351, cy=643, sx=21, sy=25, sr=40, cr=22, lx=-17, ly=-20, lr={34,25,16,9}},
 {q=q4, cx=729, cy=592, sx=18, sy=21, sr=36, cr=20, lx=-15, ly=-17, lr={31,22,15,8}},
}
local function pass(m, pile, cov, ln, sd, w)
  work(m:soften(sd), {hand="broad", tool=w, pile=pile, length={ln, ln*2.3}, coverage=cov,
       angle=0.9, curve={5,3}, orient="across", pressure={0.5,0.38},
       edge="found", clip=true})
end
for i, f in ipairs(F) do
  pass(ellipse(f.cx+f.sx, f.cy+f.sy, f.sr, f.sr*0.92) * f.q, psh, 3, 14, 15, "filbert 7")
  pass(ellipse(f.cx+f.sx+8, f.cy+f.sy+8, f.cr, f.cr*0.9) * f.q, pco, 2.6, 12, 10, "filbert 7")
  blend(f.q, {})
  pass(ellipse(f.cx+f.lx, f.cy+f.ly, f.lr[1], f.lr[1]*0.95) * f.q, pz1, 3, 14, 13, "filbert 7")
  pass(ellipse(f.cx+f.lx+1, f.cy+f.ly+2, f.lr[2], f.lr[2]*0.95) * f.q, pz2, 2.8, 14, 12, "filbert 7")
  blend(f.q, {})
  pass(ellipse(f.cx+f.lx+2, f.cy+f.ly+4, f.lr[3], f.lr[3]*0.95) * f.q, pz3, 2.6, 12, 9, "filbert 6")
  pass(ellipse(f.cx+f.lx+3, f.cy+f.ly+6, f.lr[4], f.lr[4]*0.95) * f.q, pz4, 2.2, 10, 7, "filbert 6")
  blend(f.q, {})
end
print("turned")

--@ chunk 132
gz_f = pile{{"raw umber", 4.0}, {"burnt sienna", 2.0}, {"bone black", 2.5}, {"green earth", 1.0}, {"lead white", 0.1}, medium=0.4}
work(q1 + q2 + q3 + q4, {hand="glaze", pile=gz_f, coverage=1.7, angle=1.1,
     pressure={0.4,0.28}, angle_jitter=1.2, edge="found", clip=true})

--@ chunk 133
bowl_ring = pile{{"raw umber", 3.2}, {"green earth", 1.6}, {"bone black", 2.0}, {"burnt sienna", 1.0}, {"lead white", 0.2}, medium=0}
work(((q1 + q2):grow(11) - q1 - q2) * bowl_in:grow(24), {hand="broad", tool="filbert 7",
     pile=bowl_ring, length={14,34}, coverage=3, angle=0.4, curve={5,3}, pressure={0.5,0.38},
     edge="found", clip=true})
work((q3:grow(12) - q3) - bowlm, {hand="broad", tool="filbert 8", pile=table_sc,
     length={18,40}, coverage=3, angle=0.3, curve={5,3}, pressure={0.5,0.38},
     edge="found", clip=true})
work((q4:grow(12) - q4) - bowlm, {hand="broad", tool="filbert 8", pile=table_sc,
     length={18,40}, coverage=3, angle=0.3, curve={5,3}, pressure={0.5,0.38},
     edge="found", clip=true})
blend((q3 + q4):grow(14), {})

--@ chunk 134
for i, f in ipairs({q1, q2, q3, q4}) do
  work(f, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.6 + i * 0.5, curve={6,3}, orient="across", pressure={0.8,0.7},
       dips={10, 0.95, 0.2}, edge="found", clip=true})
end
for i, f in ipairs({q1, q2, q3, q4}) do blend(f, {}) end
print("base laid")

--@ chunk 135
ringb = ((q1 + q2):grow(34) - q1 - q2) * bowlm:grow(14)
ringt = ((q3:grow(34) - q3) - bowlm) + ((q4:grow(34) - q4) - bowlm)
print(ringb:area(), ringt:area())
work(ringb, {hand="broad", tool="filbert 8", pile=bowl_ring, length={16,38}, coverage=3.4,
     angle=0.4, curve={6,3}, pressure={0.55,0.42}, edge="found", clip=true})
work(ringt, {hand="broad", tool="filbert 9", pile=table_sc, length={20,46}, coverage=3.4,
     angle=0.3, curve={6,3}, pressure={0.55,0.42}, edge="found", clip=true})
blend(ringb, {})
blend(ringt, {})

--@ chunk 136
bowl_fill = pile{{"bone black", 3.0}, {"raw umber", 3.0}, {"green earth", 1.5}, {"burnt sienna", 0.9}, medium=0}
tbl_dk = pile{{"raw umber", 4.2}, {"burnt sienna", 1.4}, {"bone black", 2.4}, {"yellow ochre", 0.5}, medium=0}
work(bowl_in, {hand="broad", tool="filbert 10", pile=bowl_fill, length={20,46}, coverage=4,
     angle=0.5, curve={6,3}, orient="across", pressure={0.8,0.68}, dips={10,0.95,0.2},
     edge="found", clip=true})
work(ringt, {hand="broad", tool="filbert 10", pile=tbl_dk, length={22,50}, coverage=4,
     angle=0.3, curve={6,3}, orient="across", pressure={0.8,0.68}, edge="found", clip=true})
blend(bowl_in, {})
blend(ringt, {})

--@ chunk 137
local function stat(n, m)
  local x0,y0,x1,y1 = 1e9,1e9,-1e9,-1e9
  for x=0,W,3 do for y=0,H,3 do if m:at(x,y) > 0.4 then
    if x<x0 then x0=x end if x>x1 then x1=x end if y<y0 then y0=y end if y>y1 then y1=y end end end end
  print(string.format("%-12s area %8.0f  x %3d..%3d y %3d..%3d", n, m:area(), x0,x1,y0,y1))
end
stat("bowlm", bowlm); stat("bowl_in", bowl_in); stat("bowlfront", bowlfront)
stat("rimfront", rimfront); stat("q1", q1); stat("q2", q2)
print("q1 in bowl_in:", (q1*bowl_in):area(), " q1 in bowlm:", (q1*bowlm):area())
print("q2 in bowl_in:", (q2*bowl_in):area(), " q2 in bowlm:", (q2*bowlm):area())
print("bowl_in inside bowlm:", (bowl_in*bowlm):area())
-- where is the bowl's top rim? sample a vertical line at x=520
for _, y in ipairs({420,430,440,450,460,470,480,500,520,540,560,570,580,590,600,610,620}) do
  print(string.format("y=%3d  bowlm %.2f  bowl_in %.2f  bowlfront %.2f", y,
    bowlm:at(520,y), bowl_in:at(520,y), bowlfront:at(520,y)))
end

--@ chunk 138
bowl_wall = pile{{"raw umber", 3.4}, {"burnt sienna", 1.6}, {"bone black", 1.8}, {"yellow ochre", 0.6}, medium=0}
bowl_lit = pile{{"yellow ochre", 2.4}, {"burnt sienna", 1.6}, {"raw umber", 2.4}, {"lead white", 0.8}, medium=0}
inw = bowl_in - q1 - q2
wallw = bowlfront - q1 - q2
work(inw:grow(3), {hand="broad", tool="filbert 10", pile=bowl_fill, length={20,46},
     coverage=4, angle=0.5, curve={6,3}, orient="across", pressure={0.8,0.68},
     dips={10,0.95,0.2}, edge="found", clip=true})
work(wallw, {hand="broad", tool="filbert 12", pile=bowl_wall, length={24,54},
     coverage=3.4, angle=0.1, curve={6,3}, orient="across", pressure={0.75,0.62},
     edge="found", clip=true})
-- the bowl wall catches light on its left, falls away to the right
bl = ellipse(430, 580, 62, 30) * wallw
work(bl:soften(16), {hand="broad", tool="filbert 9", pile=bowl_lit, length={18,42},
     coverage=2, angle=0.1, pressure={0.55,0.42}, edge="found", clip=true})
blend(inw:grow(3), {})
blend(wallw, {})

--@ chunk 139
pz1 = pile{{"yellow ochre", 4.0}, {"raw umber", 2.2}, {"cadmium yellow", 0.7}, {"transparent oxide yellow", 0.4}, {"lead white", 0.2}, medium=0}
pz2 = pile{{"yellow ochre", 3.4}, {"cadmium yellow", 1.3}, {"transparent oxide yellow", 0.5}, {"lead white", 0.6}, medium=0}
pz3 = pile{{"yellow ochre", 2.6}, {"cadmium yellow", 2.0}, {"transparent oxide yellow", 0.5}, {"lead white", 1.0}, medium=0}
pz4 = pile{{"lead white", 2.0}, {"cadmium yellow", 2.8}, {"transparent oxide yellow", 0.4}, {"yellow ochre", 0.7}, medium=0}
local function steps(q, cx, cy, lx, ly, r)
  work(q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7, curve={6,3}, orient="across", pressure={0.8,0.7}, dips={10,0.95,0.2},
       edge="found", clip=true})
  blend(q, {})
  local d = {r[1], r[2], r[3], r[4]}
  for i = 1, 4 do
    local p = (i == 1 and pz1) or (i == 2 and pz2) or (i == 3 and pz3) or pz4
    work((ellipse(cx+lx+i, cy+ly+i*1.4, d[i], d[i]*0.95) * q):soften(13 - i*1.5),
         {hand="broad", tool=(i < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - i, 32 - i*4}, coverage=3 - i*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.5,0.38}, edge="found", clip=true})
    if i == 2 then blend(q, {}) end
  end
  blend(q, {})
end
steps(q1, 466, 468, -15, -17, {27, 20, 13, 7.5})
steps(q2, 582, 478, -13, -15, {22, 16, 11, 6.5})

--@ chunk 140
bowl_front = pile{{"raw umber", 3.6}, {"burnt sienna", 1.8}, {"bone black", 2.4}, {"yellow ochre", 0.5}, medium=0}
bowl_flit = pile{{"yellow ochre", 2.6}, {"burnt sienna", 1.8}, {"raw umber", 2.6}, {"lead white", 0.6}, medium=0}
rimline = ribbon({{375,498},{420,514},{465,526},{520,534},{575,528},{620,514},{665,498}},
                 {2,4,5,5,5,4,2}) * bowlm
wallw = bowlfront - q1 - q2
work(wallw, {hand="broad", tool="filbert 12", pile=bowl_front, length={24,54},
     coverage=4, angle=0.08, curve={6,3}, orient="across", pressure={0.8,0.68},
     dips={10,0.95,0.2}, edge="found", clip=true})
work((ellipse(432, 578, 66, 34) * wallw):soften(20), {hand="broad", tool="filbert 9",
     pile=bowl_flit, length={18,42}, coverage=1.8, angle=0.06, pressure={0.5,0.4},
     edge="found", clip=true})
work((ribbon({{372,472},{424,446},{492,434},{560,442},{622,462},{670,484}}, {2,4,5,5,4,2}) * bowlm):soften(2),
     {hand="detail", tool="rigger 4", pile=bowl_flit, coverage=1.4, pressure={0.45,0.2},
      edge="found", clip=true})
blend(wallw, {})

--@ chunk 141
sA = pile{{"raw umber", 3.6}, {"burnt sienna", 1.8}, {"bone black", 2.4}, {"yellow ochre", 0.5}, medium=0}
sB = pile{{"bone black", 4.0}, {"raw umber", 4.0}, {"burnt sienna", 1.0}, medium=0}
sC = pile{{"bone black", 6.0}, {"raw umber", 2.0}, medium=0}
sD = pile{{"bone black", 4.0}, {"raw umber", 2.0}, {"green earth", 2.0}, medium=0}
local function sw(n, m, p)
  work(m, {hand="broad", tool="filbert 10", pile=p, length={20,44}, coverage=4,
       angle=0.2, curve={5,3}, orient="across", pressure={0.8,0.68}, edge="found", clip=true})
end
sw("A", rect(400, 664, 52, 36), sA)
sw("B", rect(462, 664, 52, 36), sB)
sw("C", rect(524, 664, 52, 36), sC)
sw("D", rect(586, 664, 52, 36), sD)

--@ chunk 142
glaze_bowl = pile{{"bone black", 4.0}, {"raw umber", 3.0}, {"green earth", 1.2}, {"burnt sienna", 0.6}, medium=0}
bowl_rimlit = pile{{"yellow ochre", 2.2}, {"burnt sienna", 1.4}, {"raw umber", 2.6}, {"lead white", 1.0}, medium=0}
work(wallw, {hand="broad", tool="filbert 12", pile=glaze_bowl, length={24,54},
     coverage=4.5, angle=0.08, curve={6,3}, orient="across", pressure={0.8,0.68},
     dips={10,0.95,0.2}, edge="found", clip=true})
-- a soft sheen down the left of the bowl
work((ellipse(408, 545, 46, 62) * wallw):soften(22), {hand="broad", tool="filbert 9",
     pile=bowl_rimlit, coverage=1.4, angle=1.2, pressure={0.4,0.3}, edge="found", clip=true})
-- and the lit rim along the top-left of the bowl
work((ribbon({{374,496},{398,468},{430,450},{470,440}}, {3,6,7,6}) * bowlm):soften(3),
     {hand="detail", tool="rigger 4", pile=bowl_rimlit, coverage=1.3,
      pressure={0.5,0.15}, edge="found", clip=true})
blend(wallw, {})

--@ chunk 143
dark_warm = pile{{"raw umber", 3.6}, {"burnt sienna", 1.8}, {"bone black", 2.4}, {"yellow ochre", 0.5}, medium=0}
-- the test swatches go back into the dark foreground
work(rect(388, 650, 264, 60):soften(16), {hand="broad", tool="filbert 14",
     pile=dark_warm, length={30,70}, coverage=3.5, angle=0.1, pressure={0.75,0.6},
     edge="found", clip=true})
blend(rect(370, 636, 300, 84):soften(30), {})
-- soft shadow haloes under the two loose fruit, feathered into the table
halo = ((q3:grow(46) - q3:shrink(1)) + (q4:grow(46) - q4:shrink(1))):soften(26)
work(halo, {hand="broad", tool="filbert 16", pile=dark_warm, length={24,56},
     coverage=2.2, angle=0.1, curve={8,4}, pressure={0.6,0.45}, edge="found", clip=true})
-- directional cast shadows, away from the light
cs3 = (ellipse(398, 668, 52, 24) - q3:grow(2)):soften(16)
cs4 = (ellipse(776, 616, 46, 21) - q4:grow(2)):soften(16)
for _, m in ipairs({cs3, cs4}) do
  work(m, {hand="broad", tool="filbert 10", pile=dark_warm, length={20,44},
       coverage=1.8, angle=0.1, pressure={0.55,0.42}, edge="found", clip=true})
end
blend((q3 + q4):grow(40), {})

--@ chunk 144
tbl_lt = pile{{"raw umber", 3.2}, {"burnt sienna", 2.4}, {"yellow ochre", 1.0}, {"lead white", 0.8}, {"bone black", 0.6}, medium=0}
lift = ((q3:grow(34) - q3:shrink(2)) + (q4:grow(34) - q4:shrink(2))):soften(20)
work(lift, {hand="broad", tool="filbert 16", pile=tbl_lt, length={26,60},
     coverage=3, angle=0.1, curve={8,4}, pressure={0.7,0.55}, edge="found", clip=true})
sh_pile = pile{{"raw umber", 3.6}, {"burnt sienna", 1.6}, {"bone black", 3.0}, {"yellow ochre", 0.4}, medium=0}
-- tight contact shadow hugging the base, then a short cast shadow away from the light
ct3 = ellipse(360, 664, 44, 16) - q3:grow(1)
ct4 = ellipse(740, 612, 40, 14) - q4:grow(1)
for _, m in ipairs({ct3, ct4}) do
  work(m:soften(7), {hand="broad", tool="filbert 9", pile=sh_pile, length={16,36},
       coverage=2.6, angle=0.1, pressure={0.65,0.5}, edge="found", clip=true})
end
blend(lift, {})

--@ chunk 145
ringz = ((q3:grow(34) - q3:shrink(2)) + (q4:grow(34) - q4:shrink(2))):soften(9)
work(ringz, {hand="broad", tool="filbert 16", pile=dark_warm, length={26,60},
     coverage=2.6, angle=0.1, curve={8,4}, pressure={0.7,0.55}, edge="found", clip=true})
blend((q3 + q4):grow(46):soften(14), {})

--@ chunk 146
tabfree2 = tablem - bowlm - jugm - q1 - q2 - q3 - q4
print("tablem", tablem:area(), "tabfree2", tabfree2:area())
print("leftover tablefree not in tabfree2:", (tablefree - tabfree2):area())
fg = tabfree2 * rect(0, 560, 1000, 190):soften(40)
print("fg", fg:area())

--@ chunk 147
warm_tab = pile{{"raw umber", 3.4}, {"burnt sienna", 2.2}, {"bone black", 1.8}, {"yellow ochre", 0.6}, medium=0}
work(fg, {hand="broad", tool="filbert 20", pile=warm_tab, length={50,120},
     coverage=3.2, angle=0.06, curve={8,4}, orient="across", pressure={0.75,0.6},
     dips={10,0.95,0.2}, edge="found", clip=true})
blend(fg:grow(30):soften(20), {})

--@ chunk 148
local function steps(q, cx, cy, lx, ly, r)
  work(q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7, curve={6,3}, orient="across", pressure={0.8,0.7}, dips={10,0.95,0.2},
       edge="found", clip=true})
  local d = r
  for i = 1, 4 do
    local p = (i == 1 and pz1) or (i == 2 and pz2) or (i == 3 and pz3) or pz4
    work((ellipse(cx+lx+i, cy+ly+i*1.4, d[i], d[i]*0.95) * q):soften(13 - i*1.5),
         {hand="broad", tool=(i < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - i, 32 - i*4}, coverage=3 - i*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.5,0.38}, edge="found", clip=true})
    if i == 2 then blend(q, {}) end
  end
end
steps(q3, 351, 643, -17, -20, {28, 21, 14, 8})
steps(q4, 729, 592, -15, -17, {26, 19, 13, 7})
blend(q3, {}); blend(q4, {})

--@ chunk 149
print(wait(24 * 60))
print("wall", drying(700, 200), "| table", drying(450, 690), "| bowl", drying(520, 570),
      "| fruit", drying(351, 643), "| bottle", drying(230, 400))

--@ chunk 150
gz_w = pile{{"raw umber", 4.0}, {"burnt sienna", 1.8}, {"bone black", 2.2}, {"ultramarine blue", 0.4}, {"lead white", 0.15}, medium=0.4}
gz_t = pile{{"raw umber", 3.6}, {"burnt sienna", 2.0}, {"bone black", 1.4}, {"ultramarine blue", 0.3}, {"lead white", 0.3}, medium=0.35}
work(wallm - jugm, {hand="glaze", pile=gz_w, coverage=1.7, angle=0.05,
     angle_jitter=1.4, pressure={0.42,0.3}, edge="found", clip=true})
work(tabfree2, {hand="glaze", pile=gz_t, coverage=1.5, angle=0.05,
     angle_jitter=1.4, pressure={0.42,0.3}, edge="found", clip=true})

--@ chunk 151
wall_p = pile{{"raw umber", 3.4}, {"burnt sienna", 1.4}, {"bone black", 4.0}, {"ultramarine blue", 0.4}, medium=0}
w = wallm - jugm:grow(8)
work(w, {hand="broad", tool="filbert 22", pile=wall_p, length={80,180}, coverage=5,
     angle=0.05, angle_jitter=1.6, curve={10,5}, orient="across", pressure={0.75,0.6},
     dips={10,0.95,0.2}, edge="found", clip=true})
work(w, {hand="broad", tool="filbert 22", pile=wall_p, length={80,180}, coverage=4,
     angle=1.35, angle_jitter=1.6, curve={10,5}, orient="across", pressure={0.75,0.6},
     dips={10,0.95,0.2}, edge="found", clip=true})

--@ chunk 152
bigb = {kind="filbert", width=34}
work(w, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=4.5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(w, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=3.5,
     angle=1.42, curve={0,0}, angle_jitter=0.4, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})

--@ chunk 153
table_p = pile{{"raw umber", 3.6}, {"burnt sienna", 2.4}, {"bone black", 1.6}, {"yellow ochre", 0.8}, medium=0}
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=4.5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=3.5,
     angle=1.45, curve={0,0}, angle_jitter=0.4, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})

--@ chunk 154
local out = {}
for x = 20, 980, 80 do
  local y
  for yy = 300, 620 do if tablem:at(x, yy) > 0.5 then y = yy break end end
  out[#out+1] = string.format("%d:%s", x, tostring(y))
end
print(table.concat(out, "  "))

--@ chunk 155
pool_p = pile{{"raw umber", 3.0}, {"burnt sienna", 2.2}, {"yellow ochre", 1.2}, {"lead white", 1.0}, medium=0}
dark_j = pile{{"raw umber", 3.4}, {"burnt sienna", 1.4}, {"bone black", 3.4}, {"yellow ochre", 0.3}, medium=0}
-- shadow where the wall meets the table
junc = tabfree2 * (ribbon({{0,468},{250,464},{500,460},{750,456},{1000,452}}, {16,16,16,16,16}))
work(junc:soften(14), {hand="broad", tool=bigb, pile=dark_j, length={90,190},
     coverage=2.6, angle=0.03, curve={0,0}, angle_jitter=0.5, pressure={0.72,0.6},
     edge="found", clip=true})
-- the light comes in from the left and falls away to the right
for _, e in ipairs({ellipse(330, 630, 330, 150), ellipse(300, 615, 230, 108),
                    ellipse(272, 600, 150, 72)}) do
  work((e * tabfree2):soften(54), {hand="broad", tool=bigb, pile=pool_p, length={90,190},
       coverage=1.5, angle=0.03, curve={0,0}, angle_jitter=0.5, pressure={0.6,0.45},
       edge="found", clip=true})
end

--@ chunk 156
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=4.5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=3.5,
     angle=1.45, curve={0,0}, angle_jitter=0.4, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
print("table flattened")

--@ chunk 157
pm = (ellipse(330, 622, 430, 200):soften(72)) * tabfree2
work(pm, {hand="broad", tool=bigb, pile=pool_p, length={90,190}, coverage=2.2,
     angle=0.04, curve={0,0}, angle_jitter=0.5, pressure={0.6,0.45},
     load_at=function(x, y)
       local d = math.sqrt(((x - 330) / 430) ^ 2 + ((y - 622) / 200) ^ 2)
       return clamp(1.0 - d * 1.05, 0.1, 1.0)
     end,
     dips={8, 0.95, 0.15}, edge="found", clip=true})

--@ chunk 158
local function cast(obj, dx, dy, gr, bl)
  local m = obj:offset(dx, dy)
  return (m:grow(gr):blur(bl) - obj) * (tabfree2 + bowl_in - q1 - q2 - q3 - q4)
end
sh_bottle = cast(jugm, 30, 13, 5, 8)
sh_bowl = cast(bowlm, 34, 15, 4, 9)
sh_q3 = cast(q3, 22, 11, 3, 6)
sh_q4 = cast(q4, 20, 10, 3, 6)
print(sh_bottle:area(), sh_bowl:area(), sh_q3:area(), sh_q4:area())
local allsh = sh_bottle + sh_bowl + sh_q3 + sh_q4
work(allsh, {hand="broad", tool=bigb, pile=dark_j, length={60,140}, coverage=2.4,
     angle=0.05, curve={6,4}, pressure={0.7,0.55}, edge="found", clip=true})
-- wall meets table: a soft dark seam
junc = tabfree2 * ribbon({{0,470},{250,466},{500,462},{750,458},{1000,454}}, {22,22,22,22,22})
work(junc:soften(18), {hand="broad", tool=bigb, pile=dark_j, length={90,190},
     coverage=2.0, angle=0.03, curve={0,0}, pressure={0.62,0.5}, edge="found", clip=true})

--@ chunk 159
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=4.5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=3.5,
     angle=1.45, curve={0,0}, angle_jitter=0.4, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
work((ellipse(330, 622, 430, 200):soften(72)) * tabfree2, {hand="broad", tool=bigb,
     pile=pool_p, length={90,190}, coverage=2.2, angle=0.04, curve={0,0},
     angle_jitter=0.5, pressure={0.6,0.45},
     load_at=function(x, y)
       local d = math.sqrt(((x - 330) / 430) ^ 2 + ((y - 622) / 200) ^ 2)
       return clamp(1.0 - d * 1.05, 0.1, 1.0)
     end, dips={8, 0.95, 0.15}, edge="found", clip=true})
print("table reset")

--@ chunk 160
-- soft contact shadows that hug each base and die away to the right
local function foot(e)
  return (e * tabfree2):soften(13)
end
shadows = foot(ellipse(252, 552, 58, 15)) + foot(ellipse(516, 604, 160, 19))
        + foot(ellipse(374, 680, 46, 13)) + foot(ellipse(750, 621, 42, 12))
print(shadows:area())
work(shadows, {hand="broad", tool=bigb, pile=dark_j, length={70,150}, coverage=2.6,
     angle=0.06, curve={6,4}, pressure={0.68,0.55},
     load_at=function(x, y) return clamp(0.55 + 0.5 * math.exp(-((x - 500) / 260) ^ 2), 0.5, 1.0) end,
     edge="found", clip=true})
-- and the wall/table seam
junc = tabfree2 * ribbon({{0,470},{250,466},{500,462},{750,458},{1000,454}}, {22,22,22,22,22})
work(junc:soften(18), {hand="broad", tool=bigb, pile=dark_j, length={90,190},
     coverage=1.8, angle=0.03, curve={0,0}, pressure={0.6,0.48}, edge="found", clip=true})

--@ chunk 161
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=4,
     angle=0.5, curve={4,3}, orient="across", pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
work((ellipse(330, 622, 430, 200):soften(72)) * tabfree2, {hand="broad", tool=bigb,
     pile=pool_p, length={90,190}, coverage=2.0, angle=0.04, curve={0,0},
     angle_jitter=0.5, pressure={0.6,0.45},
     load_at=function(x, y)
       local d = math.sqrt(((x - 330) / 430) ^ 2 + ((y - 622) / 200) ^ 2)
       return clamp(1.0 - d * 1.05, 0.1, 1.0)
     end, dips={8, 0.95, 0.15}, edge="found", clip=true})
shadows = ((ellipse(250, 556, 66, 15) + ellipse(516, 606, 162, 19)
        + ellipse(374, 680, 46, 13) + ellipse(750, 622, 42, 12)):soften(9)) * tabfree2
work(shadows, {hand="broad", tool=bigb, pile=dark_j, length={60,130}, coverage=3.4,
     angle=0.06, curve={6,4}, pressure={0.7,0.58}, edge="found", clip=true})

--@ chunk 162
-- put the table back under the shadow band
work(shadows, {hand="broad", tool=bigb, pile=table_p, length={110,220}, coverage=4,
     angle=0.05, curve={0,0}, angle_jitter=0.5, pressure={0.8,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
-- soft shadows laid with long sweeping strokes
work(shadows, {hand="broad", tool="filbert 18", pile=dark_j, length={150,280},
     coverage=2.8, angle=0.05, curve={12,7}, pressure={0.5,0.36}, dips={6,0.9,0.3},
     edge="found", clip=true})

--@ chunk 163
blend(tabfree2:shrink(3), {})
work(shadows:soften(30), {hand="broad", tool="filbert 18", pile=dark_j, length={150,280},
     coverage=1.3, angle=0.05, curve={12,7}, pressure={0.42,0.3}, dips={6,0.9,0.3},
     edge="found", clip=true})

--@ chunk 164
repair = ((ellipse(560, 614, 140, 28) + ellipse(430, 596, 90, 18)):soften(16)) * tabfree2
work(repair, {hand="broad", tool="filbert 20", pile=table_p, length={120,230},
     coverage=3.4, angle=0.05, curve={12,7}, angle_jitter=1.2, pressure={0.7,0.58},
     dips={8,0.95,0.2}, edge="found", clip=true})
blend(repair:grow(20):soften(14), {})

--@ chunk 165
-- warmer, more golden fruit piles
ws1 = pile{{"yellow ochre", 4.0}, {"cadmium yellow", 1.2}, {"transparent oxide yellow", 1.4}, {"raw umber", 1.6}, {"lead white", 0.3}, medium=0}
ws2 = pile{{"yellow ochre", 3.2}, {"cadmium yellow", 2.0}, {"transparent oxide yellow", 1.0}, {"lead white", 0.7}, medium=0}
ws3 = pile{{"yellow ochre", 2.2}, {"cadmium red", 0.4}, {"cadmium yellow", 2.4}, {"transparent oxide yellow", 0.6}, {"lead white", 1.0}, medium=0}
ws4 = pile{{"lead white", 2.0}, {"cadmium yellow", 2.4}, {"transparent oxide yellow", 0.8}, {"burnt sienna", 0.4}, medium=0}
wsh = pile{{"raw umber", 3.0}, {"burnt sienna", 2.0}, {"green earth", 1.0}, {"bone black", 1.6}, medium=0}
local F2 = {
 {q=q1, cx=466, cy=468, r=43, lx=-15, ly=-17, sx=18, sy=21, lr={27,20,13,7.5}},
 {q=q2, cx=582, cy=478, r=38, lx=-13, ly=-15, sx=16, sy=18, lr={24,17,11,6.5}},
 {q=q3, cx=351, cy=643, r=46, lx=-16, ly=-18, sx=19, sy=22, lr={29,21,14,8}},
 {q=q4, cx=729, cy=592, r=42, lx=-15, ly=-17, sx=18, sy=20, lr={26,19,13,7}},
}
for i, f in ipairs(F2) do
  work(f.q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7 + i * 0.4, curve={6,3}, orient="across", pressure={0.8,0.7},
       dips={10,0.95,0.2}, edge="found", clip=true})
  work((ellipse(f.cx + f.sx, f.cy + f.sy, f.r*0.86, f.r*0.8) * f.q):soften(15),
       {hand="broad", tool="filbert 7", pile=wsh, length={14,32}, coverage=3,
        angle=0.7, curve={5,3}, orient="across", pressure={0.55,0.42},
        edge="found", clip=true})
end
print("fruit base + shadow")

--@ chunk 166
F2 = {
 {q=q1, cx=466, cy=468, r=43, lx=-15, ly=-17, sx=18, sy=21, lr={27,20,13,7.5}},
 {q=q2, cx=582, cy=478, r=38, lx=-13, ly=-15, sx=16, sy=18, lr={24,17,11,6.5}},
 {q=q3, cx=351, cy=643, r=46, lx=-16, ly=-18, sx=19, sy=22, lr={29,21,14,8}},
 {q=q4, cx=729, cy=592, r=42, lx=-15, ly=-17, sx=18, sy=20, lr={26,19,13,7}},
}
for i, f in ipairs(F2) do
  for k = 1, 4 do
    local p = (k == 1 and ws1) or (k == 2 and ws2) or (k == 3 and ws3) or ws4
    work((ellipse(f.cx + f.lx + k, f.cy + f.ly + k*1.4, f.lr[k], f.lr[k]*0.95) * f.q):soften(13 - k*1.5),
         {hand="broad", tool=(k < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - k, 32 - k*4}, coverage=3 - k*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.5,0.38}, edge="found", clip=true})
    if k == 2 then blend(f.q, {}) end
  end
  blend(f.q, {})
end
print("fruit light")

--@ chunk 167
ws1 = pile{{"yellow ochre", 4.0}, {"raw umber", 2.0}, {"cadmium yellow", 0.9}, {"cadmium red", 0.35}, {"lead white", 0.3}, medium=0}
ws2 = pile{{"yellow ochre", 3.4}, {"cadmium yellow", 1.6}, {"cadmium red", 0.3}, {"transparent oxide yellow", 0.3}, {"lead white", 0.7}, medium=0}
ws3 = pile{{"yellow ochre", 2.6}, {"cadmium yellow", 2.2}, {"cadmium red", 0.35}, {"transparent oxide yellow", 0.3}, {"lead white", 1.1}, medium=0}
ws4 = pile{{"lead white", 2.2}, {"cadmium yellow", 2.8}, {"cadmium red", 0.25}, {"transparent oxide yellow", 0.3}, {"yellow ochre", 0.7}, medium=0}
for i, f in ipairs(F2) do
  for k = 1, 4 do
    local p = (k == 1 and ws1) or (k == 2 and ws2) or (k == 3 and ws3) or ws4
    work((ellipse(f.cx + f.lx + k, f.cy + f.ly + k*1.4, f.lr[k], f.lr[k]*0.95) * f.q):soften(13 - k*1.5),
         {hand="broad", tool=(k < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - k, 32 - k*4}, coverage=3.2 - k*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.55,0.42}, edge="found", clip=true})
    if k == 2 then blend(f.q, {}) end
  end
  blend(f.q, {})
end
print("ok")

--@ chunk 168
bowl_sheen = pile{{"yellow ochre", 2.2}, {"burnt sienna", 1.6}, {"raw umber", 2.8}, {"lead white", 0.8}, medium=0}
bowl_deep = pile{{"raw umber", 3.4}, {"burnt sienna", 1.4}, {"bone black", 3.4}, {"yellow ochre", 0.4}, medium=0}
work(wallw, {hand="broad", tool="filbert 18", pile=glaze_bowl, length={120,220},
     coverage=3.4, angle=0.05, curve={10,6}, angle_jitter=1.2, pressure={0.7,0.58},
     dips={8,0.95,0.2}, edge="found", clip=true})
-- the bowl turns: light down its left shoulder, dark under its right
work((ellipse(412, 552, 52, 56) * wallw):soften(26), {hand="broad", tool="filbert 14",
     pile=bowl_sheen, coverage=1.6, angle=1.2, curve={8,5}, pressure={0.45,0.35},
     edge="found", clip=true})
work((ellipse(636, 578, 46, 34) * wallw):soften(24), {hand="broad", tool="filbert 14",
     pile=bowl_deep, coverage=2, angle=1.2, curve={8,5}, pressure={0.55,0.42},
     edge="found", clip=true})
blend(wallw:shrink(5), {})

--@ chunk 169
bowl_warm2 = pile{{"bone black", 4.0}, {"raw umber", 3.4}, {"burnt sienna", 1.8}, {"transparent oxide yellow", 0.6}, medium=0}
work(wallw, {hand="broad", tool="filbert 18", pile=bowl_warm2, length={120,220},
     coverage=3.6, angle=0.05, curve={10,6}, angle_jitter=1.2, pressure={0.72,0.6},
     dips={8,0.95,0.2}, edge="found", clip=true})
work((ellipse(410, 552, 54, 58) * wallw):soften(28), {hand="broad", tool="filbert 14",
     pile=bowl_sheen, coverage=1.7, angle=1.2, curve={8,5}, pressure={0.45,0.35},
     edge="found", clip=true})
work((ellipse(640, 580, 48, 34) * wallw):soften(26), {hand="broad", tool="filbert 14",
     pile=bowl_deep, coverage=2, angle=1.2, curve={8,5}, pressure={0.55,0.42},
     edge="found", clip=true})
blend(wallw:shrink(5), {})

--@ chunk 170
fall_p = pile{{"raw umber", 4.0}, {"burnt sienna", 1.8}, {"bone black", 2.6}, {"yellow ochre", 0.3}, medium=0.1}
work(tabfree2, {hand="broad", tool="filbert 20", pile=fall_p, length={120,230},
     coverage=2.0, angle=0.05, curve={12,7}, angle_jitter=1.2, pressure={0.6,0.45},
     load_at=function(x, y)
       local t = 0.55 * (1 - x / 1000) + 0.45 * clamp((y - 430) / 320, 0, 1)
       return clamp(t * 0.95, 0.06, 0.85)
     end,
     dips={8, 0.95, 0.2}, edge="found", clip=true})

--@ chunk 171
halo_j = jugm:grow(16) - jugm:shrink(2)
ht = halo_j * tabfree2
hw = halo_j * wallm
print(ht:area(), hw:area())
work(ht, {hand="broad", tool="filbert 18", pile=table_p, length={110,210},
     coverage=4, angle=0.05, curve={10,6}, angle_jitter=1.2, pressure={0.75,0.62},
     edge="found", clip=true})
work(hw, {hand="broad", tool="filbert 18", pile=wall_p, length={110,210},
     coverage=3.6, angle=0.05, curve={10,6}, angle_jitter=1.2, pressure={0.75,0.62},
     edge="found", clip=true})
-- crisp the bottle's own edge
work(jugm:rim(9), {hand="body", tool="filbert 7", pile=jug_cover, length={16,34},
     coverage=3, angle=1.4, curve={4,3}, pressure={0.6,0.45}, edge="found", clip=true})

--@ chunk 172
lowb = jugm * rect(0, 486, 1000, 80)
work(lowb, {hand="broad", tool="filbert 10", pile=jug_dk2, length={16,38}, coverage=4.5,
     angle=1.45, curve={5,3}, orient="across", pressure={0.8,0.7}, dips={10,0.95,0.2},
     edge="found", clip=true})
-- a soft reflected light up the bottle's left, and the shoulder turning away at the right
work((ellipse(196, 500, 26, 70) * jugm):soften(16), {hand="broad", tool="filbert 7",
     pile=jug_soft, coverage=1.6, angle=1.5, curve={5,3}, pressure={0.45,0.34},
     edge="found", clip=true})
blend((jugm:shrink(5)) * rect(0, 480, 1000, 90), {})

--@ chunk 173
work(jugm, {hand="broad", tool="filbert 12", pile=jug_dk2, length={22,50}, coverage=5,
     angle=1.45, curve={6,4}, orient="across", pressure={0.85,0.75}, dips={10,0.98,0.15},
     edge="found", clip=true})
lit_edge = (ribbon({{200,272},{192,330},{190,400},{193,470},{200,532}}, {6,12,14,13,8}) * jugm):soften(5)
work(lit_edge, {hand="broad", tool="filbert 7", pile=jug_hi, length={16,34}, coverage=2.4,
     angle=1.5, curve={4,3}, pressure={0.5,0.36}, edge="found", clip=true})
blend(jugm:shrink(7), {})
print("bottle reset")

--@ chunk 174
jug_c = jugm:band(0.55, 1.0, 1.5)
print(jug_c:area(), jugm:area())
work(jug_c:rim(10), {hand="body", tool="filbert 7", pile=jug_dk2, length={16,34},
     coverage=3.2, angle=1.45, curve={4,3}, pressure={0.62,0.48}, edge="found", clip=jug_c})
work((ribbon({{206,258},{196,320},{192,392},{192,452},{197,522}}, {5,10,12,12,7}) * jug_c):soften(5),
     {hand="broad", tool="filbert 6", pile=jug_hi, length={14,30}, coverage=2.6,
      angle=1.5, curve={4,3}, pressure={0.5,0.34}, edge="found", clip=jug_c})
-- warm reflected light climbing the right of the bottle off the table
work((ribbon({{276,512},{280,470},{278,420},{272,368}}, {5,9,10,6}) * jug_c):soften(6),
     {hand="broad", tool="filbert 6", pile=jug_refl, length={14,28}, coverage=1.5,
      angle=1.5, curve={4,3}, pressure={0.42,0.3}, edge="found", clip=jug_c})
blend(jug_c:shrink(8), {})

--@ chunk 175
for i, f in ipairs(F2) do
  work(f.q:rim(6), {hand="body", tool="filbert 5", pile=wsh, length={10,22},
       coverage=1.5, angle=1.2, curve={3,2}, pressure={0.5,0.34},
       edge="found", clip=f.q})
end
for i, f in ipairs(F2) do
  local p = {{f.cx + f.lx - 16, f.cy + f.ly - 3}, {f.cx + f.lx - 10, f.cy + f.ly - 15},
             {f.cx + f.lx + 2, f.cy + f.ly - 20}}
  work((ribbon(p, {4,5,3}) * f.q:rim(9)):soften(3),
       {hand="detail", tool="rigger 4", pile=ws4, coverage=1.5,
        pressure={0.5,0.2}, edge="found", clip=f.q})
end
print("rims")

--@ chunk 176
for i, f in ipairs(F2) do
  work(f.q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7 + i * 0.4, curve={6,3}, orient="across", pressure={0.8,0.7},
       dips={10,0.95,0.2}, edge="found", clip=true})
  work((ellipse(f.cx + f.sx, f.cy + f.sy, f.r*0.86, f.r*0.8) * f.q):soften(15),
       {hand="broad", tool="filbert 7", pile=wsh, length={14,32}, coverage=3,
        angle=0.7, curve={5,3}, orient="across", pressure={0.55,0.42},
        edge="found", clip=true})
  blend(f.q, {})
  for k = 1, 4 do
    local p = (k == 1 and ws1) or (k == 2 and ws2) or (k == 3 and ws3) or ws4
    work((ellipse(f.cx + f.lx + k, f.cy + f.ly + k*1.4, f.lr[k], f.lr[k]*0.95) * f.q):soften(13 - k*1.5),
         {hand="broad", tool=(k < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - k, 32 - k*4}, coverage=3.2 - k*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.55,0.42}, edge="found", clip=true})
    if k == 2 then blend(f.q, {}) end
  end
  blend(f.q, {})
end
print("fruit rebuilt")

--@ chunk 177
wall_glow = pile{{"raw umber", 3.4}, {"burnt sienna", 2.0}, {"bone black", 2.4}, {"yellow ochre", 0.7}, medium=0}
work((ellipse(210, 400, 330, 250):soften(80)) * (wallm - jugm:grow(10)),
     {hand="broad", tool="filbert 20", pile=wall_glow, length={130,240}, coverage=1.8,
      angle=0.05, curve={12,7}, angle_jitter=1.2, pressure={0.55,0.42},
      load_at=function(x, y)
        local d = math.sqrt(((x - 210) / 330) ^ 2 + ((y - 400) / 250) ^ 2)
        return clamp(1.0 - d * 1.1, 0.08, 0.95)
      end, dips={8, 0.95, 0.2}, edge="found", clip=true})

--@ chunk 178
w = wallm - jugm:grow(10)
work(w, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(w, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=4,
     angle=1.42, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=true})

--@ chunk 179
work((ellipse(210, 400, 330, 250):soften(84)) * w, {hand="broad", tool="filbert 20",
     pile=wall_glow, length={140,250}, coverage=1.5, angle=0.05, curve={12,7},
     angle_jitter=1.2, pressure={0.5,0.38},
     load_at=function(x, y)
       local d = math.sqrt(((x - 210) / 330) ^ 2 + ((y - 400) / 250) ^ 2)
       return clamp(0.9 - d * 1.0, 0.06, 0.8)
     end, dips={8, 0.95, 0.2}, edge="found", clip=true})

--@ chunk 180
work(wallw, {hand="broad", tool="filbert 16", pile=bowl_warm2, length={120,220},
     coverage=5, angle=0.05, curve={10,6}, angle_jitter=1.2, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=true})
work((ellipse(408, 552, 56, 60) * wallw):soften(30), {hand="broad", tool="filbert 14",
     pile=bowl_sheen, coverage=1.8, angle=1.25, curve={8,5}, pressure={0.45,0.35},
     edge="found", clip=true})
work((ellipse(642, 582, 50, 36) * wallw):soften(28), {hand="broad", tool="filbert 14",
     pile=bowl_deep, coverage=2.2, angle=1.25, curve={8,5}, pressure={0.55,0.42},
     edge="found", clip=true})
blend(wallw:shrink(6), {})

--@ chunk 181
jpts = {{0,477},{120,469},{210,482},{300,465},{380,474},{470,462},
        {560,473},{650,457},{740,468},{830,451},{920,463},{1000,449}}
newtable = below(jpts)
rise = (newtable - tablem) * (wallm - jugm:grow(10))
dip = (tablem - newtable)
print("rise", rise:area(), "dip", dip:area())
work(rise, {hand="broad", tool="filbert 16", pile=table_p, length={110,210},
     coverage=4, angle=0.05, curve={10,6}, angle_jitter=1.2, pressure={0.78,0.64},
     edge="found", clip=true})
work(dip, {hand="broad", tool="filbert 16", pile=wall_p, length={110,210},
     coverage=4, angle=0.05, curve={10,6}, angle_jitter=1.2, pressure={0.78,0.64},
     edge="found", clip=true})

--@ chunk 182
work(ribbon({{0,474},{250,469},{500,464},{750,458},{1000,452}}, {26,26,26,26,26}):soften(12),
     {hand="broad", tool="filbert 18", pile=dark_j, length={130,230}, coverage=2.2,
      angle=0.03, curve={8,6}, angle_jitter=1.0, pressure={0.6,0.46},
      edge="found", clip=true})

--@ chunk 183
refl_p = pile{{"raw umber", 3.0}, {"burnt sienna", 2.0}, {"yellow ochre", 1.4}, {"cadmium yellow", 0.5}, {"lead white", 0.4}, medium=0.1}
refl = ((ellipse(330, 682, 60, 20) + ellipse(712, 626, 52, 17)
       + ellipse(400, 640, 70, 20) + ellipse(250, 566, 44, 14)):soften(16)) * tabfree2
work(refl, {hand="broad", tool="filbert 16", pile=refl_p, length={110,210}, coverage=1.8,
     angle=0.05, curve={10,6}, angle_jitter=1.2, pressure={0.55,0.42}, edge="found", clip=true})
work((shadows:soften(9)), {hand="broad", tool="filbert 14", pile=bowl_deep, length={80,170},
     coverage=1.6, angle=0.06, curve={8,5}, pressure={0.6,0.46}, edge="found", clip=true})

--@ chunk 184
work(refl:grow(12), {hand="broad", tool=bigb, pile=table_p, length={110,220},
     coverage=4.5, angle=0.05, curve={8,5}, angle_jitter=0.8, pressure={0.8,0.68},
     dips={8,0.98,0.15}, edge="found", clip=true})
work((refl:soften(34)), {hand="broad", tool="filbert 18", pile=refl_p, length={130,240},
     coverage=1.2, angle=0.05, curve={12,7}, angle_jitter=1.2, pressure={0.45,0.34},
     load_at=function(x, y) return clamp(0.95 - math.abs(y - 660) / 90, 0.15, 0.9) end,
     edge="found", clip=true})

--@ chunk 185
arq = (((q3:grow(74) + q4:grow(74)) - q3 - q4) - bowlm) * tabfree2
print(arq:area())
work(arq, {hand="broad", tool=bigb, pile=table_p, length={110,220}, coverage=5,
     angle=0.05, curve={8,5}, angle_jitter=0.8, pressure={0.82,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(arq:grow(18):soften(16), {hand="broad", tool="filbert 20", pile=table_p, length={130,240},
     coverage=2, angle=0.05, curve={12,7}, angle_jitter=1.2, pressure={0.5,0.4},
     edge="found", clip=true})
blend(arq:shrink(10), {})

--@ chunk 186
for i = 3, 4 do
  local f = F2[i]
  work(f.q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7 + i * 0.4, curve={6,3}, orient="across", pressure={0.8,0.7},
       dips={10,0.95,0.2}, edge="found", clip=true})
  work((ellipse(f.cx + f.sx, f.cy + f.sy, f.r*0.86, f.r*0.8) * f.q):soften(15),
       {hand="broad", tool="filbert 7", pile=wsh, length={14,32}, coverage=3,
        angle=0.7, curve={5,3}, orient="across", pressure={0.55,0.42},
        edge="found", clip=true})
  blend(f.q, {})
  for k = 1, 4 do
    local p = (k == 1 and ws1) or (k == 2 and ws2) or (k == 3 and ws3) or ws4
    work((ellipse(f.cx + f.lx + k, f.cy + f.ly + k*1.4, f.lr[k], f.lr[k]*0.95) * f.q):soften(13 - k*1.5),
         {hand="broad", tool=(k < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - k, 32 - k*4}, coverage=3.2 - k*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.55,0.42}, edge="found", clip=true})
    if k == 2 then blend(f.q, {}) end
  end
  blend(f.q, {})
end
print("q3 q4 rebuilt")

--@ chunk 187
local cs = ellipse(244, 550, 46, 8) + ellipse(288, 555, 48, 7)
        + ellipse(520, 612, 148, 9) + ellipse(612, 617, 118, 7)
        + ellipse(362, 690, 40, 8) + ellipse(404, 696, 42, 7)
        + ellipse(740, 630, 37, 8) + ellipse(778, 636, 38, 6)
work((cs:soften(5)) * tabfree2, {hand="broad", tool="filbert 12", pile=bowl_deep,
     length={40,90}, coverage=2.6, angle=0.05, curve={6,4}, pressure={0.62,0.48},
     dips={8,0.92,0.25}, edge="found", clip=true})

--@ chunk 188
cs = ellipse(244, 550, 46, 8) + ellipse(288, 555, 48, 7)
    + ellipse(520, 612, 148, 9) + ellipse(612, 617, 118, 7)
    + ellipse(362, 690, 40, 8) + ellipse(404, 696, 42, 7)
    + ellipse(740, 630, 37, 8) + ellipse(778, 636, 38, 6)
work((cs:grow(8)) * tabfree2, {hand="broad", tool=bigb, pile=table_p,
     length={110,220}, coverage=4.5, angle=0.05, curve={8,5}, angle_jitter=0.8,
     pressure={0.82,0.7}, dips={8,0.98,0.15}, edge="found", clip=true})
work((cs:grow(4):soften(11)) * tabfree2, {hand="broad", tool="filbert 18", pile=bowl_deep,
     length={130,230}, coverage=1.7, angle=0.05, curve={10,6}, pressure={0.5,0.38},
     dips={8,0.92,0.25}, edge="found", clip=true})

--@ chunk 189
work((ellipse(215, 390, 340, 260):soften(90)) * (wallm - jugm:grow(12)),
     {hand="broad", tool="filbert 22", pile=wall_glow, length={150,260}, coverage=1.2,
      angle=0.05, curve={12,7}, angle_jitter=1.2, pressure={0.45,0.34},
      load_at=function(x, y)
        local d = math.sqrt(((x - 215) / 340) ^ 2 + ((y - 390) / 260) ^ 2)
        return clamp(0.85 - d * 0.95, 0.05, 0.7)
      end, dips={8, 0.95, 0.2}, edge="found", clip=true})

--@ chunk 190
-- the table: a broad warm passage in the light, and a cooler darker foreground
tbl_warm = pile{{"raw umber", 3.2}, {"burnt sienna", 2.2}, {"yellow ochre", 1.2}, {"lead white", 0.7}, medium=0}
tbl_cool = pile{{"raw umber", 3.6}, {"burnt sienna", 1.5}, {"bone black", 2.0}, {"ultramarine blue", 0.5}, medium=0.08}
work((ellipse(300, 600, 300, 130):soften(60)) * tabfree2, {hand="broad", tool="filbert 20",
     pile=tbl_warm, length={130,240}, coverage=1.3, angle=0.05, curve={12,7},
     angle_jitter=1.2, pressure={0.48,0.36},
     load_at=function(x, y)
       local d = math.sqrt(((x - 300) / 300) ^ 2 + ((y - 600) / 130) ^ 2)
       return clamp(0.9 - d * 1.0, 0.05, 0.75)
     end, dips={8,0.95,0.2}, edge="found", clip=true})
work((rect(0, 640, 1000, 110):soften(50)) * tabfree2, {hand="broad", tool="filbert 20",
     pile=tbl_cool, length={130,240}, coverage=1.4, angle=0.05, curve={12,7},
     angle_jitter=1.2, pressure={0.5,0.38},
     load_at=function(x, y) return clamp(0.2 + (y - 620) / 140, 0.15, 0.85) end,
     dips={8,0.95,0.2}, edge="found", clip=true})

--@ chunk 191
fgr = (rect(0, 618, 1000, 132):soften(40)) * tabfree2
work(fgr, {hand="broad", tool=bigb, pile=table_p, length={110,220}, coverage=5,
     angle=0.05, curve={8,5}, angle_jitter=0.8, pressure={0.82,0.7},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(fgr, {hand="broad", tool=bigb, pile=tbl_cool, length={110,220}, coverage=0.9,
     angle=0.05, curve={8,5}, angle_jitter=0.8, pressure={0.55,0.42},
     load_at=function(x, y) return clamp(0.1 + (y - 600) / 150, 0.08, 0.7) end,
     dips={8,0.95,0.2}, edge="found", clip=true})

--@ chunk 192
dimple = pile{{"raw umber", 2.4}, {"bone black", 1.6}, {"green earth", 1.4}, medium=0}
sb = brush("rigger", 3.5, 0.8)
sb:load(dimple, 0.85)
sb:touch(474, 431, {pressure=0.55, drag={0, 4}})
sb:touch(589, 446, {pressure=0.5, drag={0, 3.5}})
sb:touch(360, 603, {pressure=0.55, drag={0, 4}})
sb:touch(737, 556, {pressure=0.5, drag={0, 3.5}})
hb = brush("rigger", 3, 0.8)
hb:load(ws3, 0.7)
hb:touch(469, 436, {pressure=0.4})
hb:touch(584, 451, {pressure=0.35})
hb:touch(355, 608, {pressure=0.4})
hb:touch(732, 561, {pressure=0.35})

--@ chunk 193
rim_hi = pile{{"lead white", 2.4}, {"yellow ochre", 2.0}, {"burnt sienna", 0.8}, medium=0}
rb3 = brush("rigger", 4, 0.75)
rb3:load(rim_hi, 0.8)
rb3:stroke({{378,492},{404,466},{438,448},{476,440}}, {pressure={0.5,0.15,0.45,0.1}})
rb3:stroke({{392,510},{412,492},{436,478}}, {pressure={0.4,0.1}})
gl_hi = pile{{"lead white", 2.6}, {"green earth", 2.2}, {"yellow ochre", 1.6}, medium=0}
rb3:reload(gl_hi, 0.7)
rb3:stroke({{196,404},{198,440}}, {pressure={0.5,0.15}})
rb3:stroke({{196,314},{200,348}}, {pressure={0.45,0.12}})

--@ chunk 194
inw2 = (bowl_in - q1 - q2)
work(inw2:grow(2), {hand="broad", tool="filbert 12", pile=bowl_warm2, length={60,130},
     coverage=4.5, angle=0.5, curve={8,5}, orient="across", pressure={0.82,0.7},
     dips={8,0.98,0.15}, edge="found", clip=bowl_in})
work(inw2:grow(2), {hand="broad", tool="filbert 12", pile=bowl_fill, length={50,110},
     coverage=3, angle=1.4, curve={8,5}, orient="across", pressure={0.72,0.58},
     edge="found", clip=bowl_in})

--@ chunk 195
inw2 = bowl_in - q1 - q2
work(inw2, {hand="broad", tool=bigb, pile=bowl_fill, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=inw2})
work(inw2, {hand="broad", tool=bigb, pile=bowl_fill, length={90,200}, coverage=4,
     angle=1.4, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=inw2})

--@ chunk 196
unify = pile{{"raw umber", 4.0}, {"burnt sienna", 2.2}, {"transparent oxide yellow", 1.0}, {"lead white", 0.5}, medium=0.28}
work(everywhere(), {hand="broad", tool="filbert 24", pile=unify, length={160,290},
     coverage=0.9, angle=0.04, curve={14,8}, angle_jitter=1.4,
     pressure={0.38,0.28}, dips={6,0.9,0.3}, edge="found", clip=true})

--@ chunk 197
blend(everywhere(), {})

--@ chunk 198
w = wallm - jugm:grow(12)
work(w, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(w, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=4,
     angle=1.42, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=true})
work(tabfree2, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=4,
     angle=1.45, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=true})

--@ chunk 199
work(jugm, {hand="broad", tool="filbert 12", pile=jug_dk2, length={22,50}, coverage=5,
     angle=1.45, curve={6,4}, orient="across", pressure={0.85,0.75}, dips={10,0.98,0.15},
     edge="found", clip=true})
work(inw2, {hand="broad", tool=bigb, pile=bowl_fill, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=inw2})
work(wallw, {hand="broad", tool=bigb, pile=bowl_warm2, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.85,0.75},
     dips={8,0.98,0.15}, edge="found", clip=wallw})
for i, f in ipairs(F2) do
  work(f.q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7 + i * 0.4, curve={6,3}, orient="across", pressure={0.8,0.7},
       dips={10,0.95,0.2}, edge="found", clip=true})
end
print("objects flattened")

--@ chunk 200
-- bottle: lit shoulder down the left, warm bounce up the right
work((ribbon({{204,256},{194,318},{190,390},{191,452},{197,524}}, {5,10,12,12,7}) * jugm):soften(5),
     {hand="broad", tool="filbert 6", pile=jug_hi, length={14,30}, coverage=2.6,
      angle=1.5, curve={4,3}, pressure={0.5,0.34}, edge="found", clip=true})
work((ribbon({{276,514},{281,468},{278,418},{272,364}}, {5,9,10,6}) * jugm):soften(6),
     {hand="broad", tool="filbert 6", pile=jug_refl, length={14,28}, coverage=1.6,
      angle=1.5, curve={4,3}, pressure={0.42,0.3}, edge="found", clip=true})
blend(jugm:shrink(8), {})
-- bowl: light down the left shoulder, dark falling away to the right
work((ellipse(410, 552, 56, 60) * wallw):soften(30), {hand="broad", tool="filbert 14",
     pile=bowl_sheen, coverage=1.8, angle=1.25, curve={8,5}, pressure={0.45,0.35},
     edge="found", clip=true})
work((ellipse(642, 582, 50, 36) * wallw):soften(28), {hand="broad", tool="filbert 14",
     pile=bowl_deep, coverage=2.2, angle=1.25, curve={8,5}, pressure={0.55,0.42},
     edge="found", clip=true})
blend(wallw:shrink(6), {})

--@ chunk 201
for i, f in ipairs(F2) do
  work((ellipse(f.cx + f.sx, f.cy + f.sy, f.r*0.86, f.r*0.8) * f.q):soften(15),
       {hand="broad", tool="filbert 7", pile=wsh, length={14,32}, coverage=3,
        angle=0.7, curve={5,3}, orient="across", pressure={0.55,0.42},
        edge="found", clip=true})
  blend(f.q, {})
  for k = 1, 4 do
    local p = (k == 1 and ws1) or (k == 2 and ws2) or (k == 3 and ws3) or ws4
    work((ellipse(f.cx + f.lx + k, f.cy + f.ly + k*1.4, f.lr[k], f.lr[k]*0.95) * f.q):soften(13 - k*1.5),
         {hand="broad", tool=(k < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - k, 32 - k*4}, coverage=3.2 - k*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.55,0.42}, edge="found", clip=true})
    if k == 2 then blend(f.q, {}) end
  end
  blend(f.q, {})
end
print("fruit modelled")

--@ chunk 202
patch = (rect(300, 90, 400, 280):soften(20)) * w
work(patch, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=6,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(patch, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=5,
     angle=1.42, curve={0,0}, angle_jitter=0.4, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(jugm, {hand="broad", tool=bigb, pile=jug_dk2, length={80,180}, coverage=6,
     angle=1.44, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(wallw, {hand="broad", tool=bigb, pile=bowl_warm2, length={80,180}, coverage=6,
     angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=wallw})

--@ chunk 203
work((ribbon({{204,256},{194,318},{190,390},{191,452},{197,524}}, {5,10,12,12,7}) * jugm):soften(5),
     {hand="broad", tool="filbert 6", pile=jug_hi, length={14,30}, coverage=2.8,
      angle=1.5, curve={4,3}, pressure={0.5,0.34}, edge="found", clip=true})
work((ellipse(410, 552, 56, 60) * wallw):soften(30), {hand="broad", tool="filbert 14",
     pile=bowl_sheen, coverage=1.8, angle=1.25, curve={8,5}, pressure={0.45,0.35},
     edge="found", clip=true})
work((ellipse(642, 582, 50, 36) * wallw):soften(28), {hand="broad", tool="filbert 14",
     pile=bowl_deep, coverage=2.2, angle=1.25, curve={8,5}, pressure={0.55,0.42},
     edge="found", clip=true})

--@ chunk 204
sp_j = (jugm:grow(11) - jugm)
work(sp_j * wallm, {hand="broad", tool="filbert 12", pile=wall_p, length={80,160},
     coverage=5, angle=0.05, curve={8,5}, angle_jitter=0.6, pressure={0.85,0.72},
     edge="found", clip=true})
work(sp_j * tabfree2, {hand="broad", tool="filbert 12", pile=table_p, length={80,160},
     coverage=5, angle=0.05, curve={8,5}, angle_jitter=0.6, pressure={0.85,0.72},
     edge="found", clip=true})
sp_b = (wallw:grow(11) - wallw) - q3 - q4
work(sp_b * tabfree2, {hand="broad", tool="filbert 12", pile=table_p, length={80,160},
     coverage=5, angle=0.05, curve={8,5}, angle_jitter=0.6, pressure={0.85,0.72},
     edge="found", clip=true})
work(sp_b * wallm, {hand="broad", tool="filbert 12", pile=wall_p, length={80,160},
     coverage=5, angle=0.05, curve={8,5}, angle_jitter=0.6, pressure={0.85,0.72},
     edge="found", clip=true})

--@ chunk 205
work(jugm:grow(7), {hand="broad", tool=bigb, pile=jug_dk2, length={80,180},
     coverage=6, angle=1.44, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=jugm:grow(7)})
work(bowl_in:grow(7), {hand="broad", tool=bigb, pile=bowl_fill, length={80,180},
     coverage=6, angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=bowl_in:grow(7)})
work(wallw:grow(7), {hand="broad", tool=bigb, pile=bowl_warm2, length={80,180},
     coverage=6, angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=wallw:grow(7)})

--@ chunk 206
work((ribbon({{204,256},{194,318},{190,390},{191,452},{197,524}}, {5,10,12,12,7}) * jugm):soften(5),
     {hand="broad", tool="filbert 6", pile=jug_hi, length={14,30}, coverage=2.8,
      angle=1.5, curve={4,3}, pressure={0.5,0.34}, edge="found", clip=true})
work((ellipse(410, 552, 56, 60) * wallw):soften(30), {hand="broad", tool="filbert 14",
     pile=bowl_sheen, coverage=1.8, angle=1.25, curve={8,5}, pressure={0.45,0.35},
     edge="found", clip=true})
work((ellipse(642, 582, 50, 36) * wallw):soften(28), {hand="broad", tool="filbert 14",
     pile=bowl_deep, coverage=2.2, angle=1.25, curve={8,5}, pressure={0.55,0.42},
     edge="found", clip=true})

--@ chunk 207
for i = 1, 2 do
  local f = F2[i]
  work(f.q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7 + i * 0.4, curve={6,3}, orient="across", pressure={0.8,0.7},
       dips={10,0.95,0.2}, edge="found", clip=true})
  work((ellipse(f.cx + f.sx, f.cy + f.sy, f.r*0.86, f.r*0.8) * f.q):soften(15),
       {hand="broad", tool="filbert 7", pile=wsh, length={14,32}, coverage=3,
        angle=0.7, curve={5,3}, orient="across", pressure={0.55,0.42},
        edge="found", clip=true})
  blend(f.q, {})
  for k = 1, 4 do
    local p = (k == 1 and ws1) or (k == 2 and ws2) or (k == 3 and ws3) or ws4
    work((ellipse(f.cx + f.lx + k, f.cy + f.ly + k*1.4, f.lr[k], f.lr[k]*0.95) * f.q):soften(13 - k*1.5),
         {hand="broad", tool=(k < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - k, 32 - k*4}, coverage=3.2 - k*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.55,0.42}, edge="found", clip=true})
    if k == 2 then blend(f.q, {}) end
  end
  blend(f.q, {})
end
print("q1 q2 back")

--@ chunk 208
-- the bottle's lit band was reading as a string of beads: wider and softer
work(jugm, {hand="broad", tool=bigb, pile=jug_dk2, length={80,180}, coverage=5,
     angle=1.44, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work((ribbon({{206,254},{196,318},{191,392},{192,454},{198,526}}, {12,20,24,23,14}) * jugm):soften(9),
     {hand="broad", tool="filbert 10", pile=jug_hi, length={24,52}, coverage=2.6,
      angle=1.5, curve={6,4}, pressure={0.45,0.32}, edge="found", clip=true})
work((ribbon({{276,514},{281,468},{278,418},{272,362}}, {10,16,18,12}) * jugm):soften(8),
     {hand="broad", tool="filbert 10", pile=jug_refl, length={24,50}, coverage=1.6,
      angle=1.5, curve={6,4}, pressure={0.4,0.28}, edge="found", clip=true})

--@ chunk 209
inw3 = (bowl_in - q1 - q2):grow(4)
wallw3 = (bowlfront - q1 - q2):grow(4)
work(jugm, {hand="broad", tool=bigb, pile=jug_dk2, length={80,180}, coverage=6,
     angle=1.44, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(inw3, {hand="broad", tool=bigb, pile=bowl_fill, length={80,180}, coverage=6,
     angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=inw3})
work(wallw3, {hand="broad", tool=bigb, pile=bowl_warm2, length={80,180}, coverage=6,
     angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=wallw3})

--@ chunk 210
work((ellipse(208, 398, 58, 165) * jugm):soften(32), {hand="broad", tool="filbert 16",
     pile=jug_hi, length={40,90}, coverage=2.0, angle=1.5, curve={8,5}, orient="across",
     pressure={0.42,0.3}, edge="found", clip=true})
work((ellipse(272, 440, 40, 140) * jugm):soften(30), {hand="broad", tool="filbert 16",
     pile=jug_refl, length={40,90}, coverage=1.3, angle=1.5, curve={8,5}, orient="across",
     pressure={0.38,0.28}, edge="found", clip=true})
work((ellipse(408, 552, 62, 62) * wallw3):soften(34), {hand="broad", tool="filbert 16",
     pile=bowl_sheen, length={44,96}, coverage=1.8, angle=1.25, curve={8,5}, orient="across",
     pressure={0.4,0.3}, edge="found", clip=true})
work((ellipse(646, 584, 56, 40) * wallw3):soften(30), {hand="broad", tool="filbert 16",
     pile=bowl_deep, length={44,96}, coverage=2.0, angle=1.25, curve={8,5}, orient="across",
     pressure={0.5,0.4}, edge="found", clip=true})

--@ chunk 211
work(jugm, {hand="broad", tool=bigb, pile=jug_dk2, length={80,180}, coverage=6,
     angle=1.44, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(jugm, {hand="broad", tool=bigb, pile=jug_dk2, length={80,180}, coverage=5,
     angle=0.46, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
blend(jugm:shrink(9), {})
work((ellipse(206, 400, 52, 150) * jugm):soften(30), {hand="broad", tool="filbert 20",
     pile=jug_hi, length={60,120}, coverage=1.0, angle=1.5, curve={10,6},
     pressure={0.35,0.26}, edge="found", clip=true})
work((ellipse(274, 445, 34, 130) * jugm):soften(26), {hand="broad", tool="filbert 20",
     pile=jug_refl, length={60,120}, coverage=0.7, angle=1.5, curve={10,6},
     pressure={0.32,0.24}, edge="found", clip=true})
blend(jugm:shrink(9), {})

--@ chunk 212
work(inw3, {hand="broad", tool=bigb, pile=bowl_fill, length={80,180}, coverage=6,
     angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=inw3})
work(wallw3, {hand="broad", tool=bigb, pile=bowl_warm2, length={80,180}, coverage=6,
     angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=wallw3})
blend(bowlm:shrink(10), {})
work((ellipse(404, 550, 66, 66) * wallw3):soften(36), {hand="broad", tool="filbert 20",
     pile=bowl_sheen, length={60,120}, coverage=1.1, angle=1.25, curve={10,6},
     pressure={0.36,0.27}, edge="found", clip=true})
work((ellipse(650, 586, 58, 42) * wallw3):soften(32), {hand="broad", tool="filbert 20",
     pile=bowl_deep, length={60,120}, coverage=1.2, angle=1.25, curve={10,6},
     pressure={0.4,0.3}, edge="found", clip=true})
blend(bowlm:shrink(10), {})

--@ chunk 213
for i = 1, 2 do
  local f = F2[i]
  work(f.q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7 + i * 0.4, curve={6,3}, orient="across", pressure={0.8,0.7},
       dips={10,0.95,0.2}, edge="found", clip=true})
  work((ellipse(f.cx + f.sx, f.cy + f.sy, f.r*0.86, f.r*0.8) * f.q):soften(15),
       {hand="broad", tool="filbert 7", pile=wsh, length={14,32}, coverage=3,
        angle=0.7, curve={5,3}, orient="across", pressure={0.55,0.42},
        edge="found", clip=true})
  blend(f.q, {})
  for k = 1, 4 do
    local p = (k == 1 and ws1) or (k == 2 and ws2) or (k == 3 and ws3) or ws4
    work((ellipse(f.cx + f.lx + k, f.cy + f.ly + k*1.4, f.lr[k], f.lr[k]*0.95) * f.q):soften(13 - k*1.5),
         {hand="broad", tool=(k < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - k, 32 - k*4}, coverage=3.2 - k*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.55,0.42}, edge="found", clip=true})
    if k == 2 then blend(f.q, {}) end
  end
  blend(f.q, {})
end
print("ok")

--@ chunk 214
work(tabfree2, {hand="broad", tool="filbert 20", pile=fall_p, length={130,240},
     coverage=1.8, angle=0.05, curve={12,7}, angle_jitter=1.2, pressure={0.55,0.42},
     load_at=function(x, y)
       local t = 0.55 * (1 - x / 1000) + 0.45 * clamp((y - 430) / 320, 0, 1)
       return clamp(t * 0.95, 0.06, 0.85)
     end, dips={8, 0.95, 0.2}, edge="found", clip=true})
work((ellipse(215, 390, 340, 260):soften(90)) * (wallm - jugm:grow(14)),
     {hand="broad", tool="filbert 22", pile=wall_glow, length={150,260}, coverage=1.2,
      angle=0.05, curve={12,7}, angle_jitter=1.2, pressure={0.42,0.32},
      load_at=function(x, y)
        local d = math.sqrt(((x - 215) / 340) ^ 2 + ((y - 390) / 260) ^ 2)
        return clamp(0.85 - d * 0.95, 0.05, 0.7)
      end, dips={8, 0.95, 0.2}, edge="found", clip=true})
work(ribbon({{0,474},{250,469},{500,464},{750,458},{1000,452}}, {26,26,26,26,26}):soften(12),
     {hand="broad", tool="filbert 18", pile=dark_j, length={130,230}, coverage=2.0,
      angle=0.03, curve={8,6}, angle_jitter=1.0, pressure={0.58,0.44},
      edge="found", clip=true})

--@ chunk 215
wl = rect(0, 60, 780, 400) * w
jb = rect(0, 420, 1000, 92):soften(10) * tabfree2
work(wl, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(wl, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=4,
     angle=1.42, curve={0,0}, angle_jitter=0.4, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(jb, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(jb, {hand="broad", tool=bigb, pile=table_p, length={90,200}, coverage=4,
     angle=1.45, curve={0,0}, angle_jitter=0.4, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})

--@ chunk 216
w2 = wallm - bowlm - q1 - q2 - jugm:grow(10)
work(w2, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=5,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(w2, {hand="broad", tool=bigb, pile=wall_p, length={90,200}, coverage=4,
     angle=1.42, curve={0,0}, angle_jitter=0.4, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=true})
work(inw3, {hand="broad", tool=bigb, pile=bowl_fill, length={80,180}, coverage=6,
     angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=inw3})
work(wallw3, {hand="broad", tool=bigb, pile=bowl_warm2, length={80,180}, coverage=6,
     angle=0.04, curve={0,0}, angle_jitter=0.3, pressure={0.88,0.78},
     dips={8,0.99,0.12}, edge="found", clip=wallw3})

--@ chunk 217
for i = 1, 2 do
  local f = F2[i]
  work(f.q, {hand="broad", tool="filbert 7", pile=fruit_dk, length={14,34}, coverage=4.5,
       angle=0.7 + i * 0.4, curve={6,3}, orient="across", pressure={0.8,0.7},
       dips={10,0.95,0.2}, edge="found", clip=true})
  work((ellipse(f.cx + f.sx, f.cy + f.sy, f.r*0.86, f.r*0.8) * f.q):soften(15),
       {hand="broad", tool="filbert 7", pile=wsh, length={14,32}, coverage=3,
        angle=0.7, curve={5,3}, orient="across", pressure={0.55,0.42},
        edge="found", clip=true})
  blend(f.q, {})
  for k = 1, 4 do
    local p = (k == 1 and ws1) or (k == 2 and ws2) or (k == 3 and ws3) or ws4
    work((ellipse(f.cx + f.lx + k, f.cy + f.ly + k*1.4, f.lr[k], f.lr[k]*0.95) * f.q):soften(13 - k*1.5),
         {hand="broad", tool=(k < 3 and "filbert 7" or "filbert 6"), pile=p,
          length={14 - k, 32 - k*4}, coverage=3.2 - k*0.2, angle=0.7, curve={5,3},
          orient="across", pressure={0.55,0.42}, edge="found", clip=true})
    if k == 2 then blend(f.q, {}) end
  end
  blend(f.q, {})
end
print("ok")

--@ chunk 218
jm = rect(0, 400, 1000, 150) * tabfree2
work(jm, {hand="broad", tool=bigb, pile=dark_j, length={110,220}, coverage=1.6,
     angle=0.03, curve={0,0}, angle_jitter=0.4, pressure={0.7,0.56},
     load_at=function(x, y) return clamp(1.3 - math.abs(y - (477.4 - 0.02 * x)) / 20, 0.04, 1.0) end,
     dips={8,0.96,0.18}, edge="found", clip=true})
cs = ellipse(244, 550, 44, 8) + ellipse(290, 556, 46, 7)
   + ellipse(520, 612, 146, 9) + ellipse(614, 617, 114, 7)
   + ellipse(362, 690, 38, 8) + ellipse(406, 696, 40, 7)
   + ellipse(740, 630, 35, 8) + ellipse(780, 636, 36, 6)
work(cs:soften(6) * tabfree2, {hand="broad", tool=bigb, pile=bowl_deep,
     length={110,220}, coverage=1.8, angle=0.05, curve={0,0}, angle_jitter=0.4,
     pressure={0.72,0.58}, dips={8,0.96,0.18}, edge="found", clip=true})

--@ chunk 219
bounce = pile{{"raw umber", 2.6}, {"burnt sienna", 2.2}, {"yellow ochre", 1.4}, {"cadmium yellow", 0.4}, {"lead white", 0.4}, medium=0}
glow1 = ((ellipse(466, 474, 58, 52) + ellipse(582, 484, 50, 46)) * bowl_in) - q1:grow(2) - q2:grow(2)
work(glow1:soften(12), {hand="broad", tool=bigb, pile=bounce, length={90,180},
     coverage=1.3, angle=0.05, curve={0,0}, angle_jitter=0.5, pressure={0.6,0.46},
     load_at=function(x, y) return clamp(1.1 - math.abs(y - 500) / 46, 0.05, 1.0) end,
     edge="found", clip=true})

--@ chunk 220
cool_p = pile{{"ultramarine blue", 1.2}, {"green earth", 1.8}, {"raw umber", 3.6}, {"bone black", 1.8}, medium=0.15}
work(w2, {hand="broad", tool=bigb, pile=cool_p, length={110,220}, coverage=0.9,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.6,0.46},
     load_at=function(x, y) return clamp((x - 480) / 620 * 0.8, 0.03, 0.7) end,
     dips={8,0.95,0.2}, edge="found", clip=true})
work(tabfree2, {hand="broad", tool=bigb, pile=cool_p, length={110,220}, coverage=1.0,
     angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.6,0.46},
     load_at=function(x, y) return clamp((x - 560) / 520 * 0.85, 0.03, 0.75) end,
     dips={8,0.95,0.2}, edge="found", clip=true})

--@ chunk 221
warm_p = pile{{"lead white", 1.4}, {"cadmium yellow", 1.6}, {"cadmium red", 0.5}, {"yellow ochre", 0.8}, medium=0}
green_p = pile{{"green earth", 2.0}, {"raw umber", 2.0}, {"yellow ochre", 1.6}, {"lead white", 0.3}, medium=0}
fb = brush("rigger", 4.5, 0.85)
fb:load(warm_p, 0.55)
for _, s in ipairs({{446,452},{458,458},{436,462},{468,446},{452,470},
                    {570,466},{580,472},{564,474},{592,462},{574,462},
                    {334,626},{346,632},{328,638},{356,620},{340,650},
                    {714,578},{724,584},{708,588},{734,572},{718,596}}) do
  fb:stroke({{s[1], s[2]}, {s[1] + 3, s[2] + 2}}, {pressure={0.4, 0.05}, clip = F2[1].q + F2[2].q + F2[3].q + F2[4].q})
end
fb:reload(green_p, 0.5)
for _, s in ipairs({{480,486},{470,476},{492,496},{600,494},{590,486},{612,502},
                    {368,660},{360,650},{380,672},{742,608},{734,600},{756,618}}) do
  fb:stroke({{s[1], s[2]}, {s[1] + 2, s[2] + 2}}, {pressure={0.35, 0.05}, clip = F2[1].q + F2[2].q + F2[3].q + F2[4].q})
end

--@ chunk 222
work(tabfree2, {hand="broad", tool=bigb, pile=tbl_cool, length={110,220},
     coverage=1.5, angle=0.04, curve={0,0}, angle_jitter=0.4, pressure={0.66,0.52},
     load_at=function(x, y)
       return clamp(0.8 * clamp((y - 600) / 150, 0, 1) + 0.35 * clamp((x - 640) / 420, 0, 1), 0.03, 0.75)
     end, dips={8,0.96,0.18}, edge="found", clip=true})

--@ chunk 223
local pts = {{80,120},{500,150},{500,300},{100,300},{300,520},{620,480},{520,640},{880,650},{500,700}}
for i,p in ipairs(pts) do
  print(p[1], p[2], drying(p[1],p[2]))
end

--@ chunk 224
test_p = pile{{"lead white", 2},{"raw umber", 3},{"bone black", 1}}
tb = brush{kind="filbert", width=16}
tb:load(test_p, 0.9)
tb:stroke({{20,20},{70,30},{120,22}}, {pressure={0.9,0.9}, clip=rect(0,0,200,80)})
print("fullness", tb:fullness())
print(drying(60,26))

--@ chunk 225
print(wait(2*24*60))
print("corner", drying(60,26), "wall", drying(500,150), "table", drying(300,700), "fruit", drying(468,462))

--@ chunk 226
local names = {"bowlm","q1","q2","q3","q4","allq","wallm","tablem","wallw","w","w2","wl","jb","jm","jug_c","newtable","fg","pm","bl","inw","ringt","ringb","c1","c2","c3","c4","l1","l2","l3","l4","cs","arq","refl","sh_bottle","sh_bowl","shadows"}
for _,n in ipairs(names) do
  local v = _G[n]
  if type(v) == "table" and v.area then
    local ok, a = pcall(function() return v:area() end)
    print(n, ok and a or "err")
  else
    print(n, type(v))
  end
end

--@ chunk 227
local names = {"bowlm","q1","q2","q3","q4","allq","wallm","tablem","wallw","w","w2","wl","jb","jm","jug_c","newtable","fg","pm","bl","inw","ringt","ringb","c1","c2","c3","c4","l1","l2","l3","l4","cs","arq","refl","sh_bottle","sh_bowl","shadows"}
for _,n in ipairs(names) do
  local v = _G[n]
  if type(v) == "userdata" then
    local ok, a = pcall(function() return v:area() end)
    print(n, ok and a or ("err "..tostring(a)))
  end
end

--@ chunk 228
function bbox(m, step)
  step = step or 5
  local x0,y0,x1,y1 = 1e9,1e9,-1e9,-1e9
  for y = 0, H, step do
    for x = 0, W, step do
      if m:at(x,y) > 0.5 then
        if x<x0 then x0=x end if x>x1 then x1=x end
        if y<y0 then y0=y end if y>y1 then y1=y end
      end
    end
  end
  return x0,y0,x1,y1
end
for _,n in ipairs({"bowlm","q1","q2","q3","q4","jb","jug_c","inw","ringt","pm","cs","refl","shadows"}) do
  print(n, bbox(_G[n]))
end

--@ chunk 229
flat34 = brush{kind="filbert", width=34}
work(rect(0,0,200,80), {hand="broad", pile=test_p, tool=flat34, coverage=5, length={90,200}, pressure={0.7,0.95}, edge="found", clip=true, curve={0,0}})
print("corner", drying(60,26))

--@ chunk 230
-- silhouettes and passages for the rebuild
bottle_pts = {{211,234},{271,233},{269,275},{268,320},{276,352},{287,392},{293,430},{296,470},{295,510},{288,535},{274,548},{208,547},{192,538},{180,518},{173,492},{172,455},{174,418},{181,378},{192,346},{200,316},{205,282},{208,250}}
bottle_m = poly(bottle_pts, true):soften(1.5)
print("bottle", bottle_m:area(), bbox(bottle_m))

juncf = function(x) return 472 + 0.047*(x-300) end
wallA = below(juncf)
tableA = above(function(x) return juncf(x) - 30 end)
objects = bottle_m + bowlm + allq
wall_m = (wallA - objects)
table_m = (tableA - bowlm - allq - bottle_m)
print("wall", wall_m:area(), "table", table_m:area())
print("wall bbox", bbox(wall_m, 10))
print("table bbox", bbox(table_m, 10))

--@ chunk 231
wall_m = above(juncf) - objects
table_m = below(function(x) return juncf(x) - 30 end) - bowlm - allq - bottle_m
print("wall", wall_m:area(), bbox(wall_m,10))
print("table", table_m:area(), bbox(table_m,10))
print("corners", wall_m:at(20,20), table_m:at(20,700), wall_m:at(980,100))

--@ chunk 232
wall_dk = pile{{"raw umber",3.2},{"burnt sienna",1.8},{"bone black",2.6},{"ultramarine blue",0.4},{"lead white",0.15}}
work(wall_m, {hand="broad", pile=wall_dk, tool=flat34, coverage=5, length={90,200}, pressure={0.7,0.95}, edge="found", clip=true})

--@ chunk 233
lt = function(x,y) local d = math.sqrt(((x-280)/620)^2 + ((y-190)/520)^2); return clamp(1.05 - d*1.1, 0.12, 1) end
wall_lt = pile{{"raw umber",2.4},{"burnt sienna",2.2},{"yellow ochre",0.8},{"lead white",0.8},{"bone black",0.8}}
work(wall_m, {hand="broad", pile=wall_lt, tool=flat34, coverage=3, length={100,210}, pressure={0.6,0.95}, load_at=lt, edge="found", clip=true, angle=-0.22})

--@ chunk 234
work(wall_m, {hand="broad", pile=wall_dk, tool=flat34, coverage=4, length={110,240}, pressure={0.75,0.95}, edge="found", clip=true, broken=0.35, curve={8,4}})

--@ chunk 235
nw = noise{seed=17, octaves=3, period=230, persistence=0.55}
print(type(nw))
local a,b = pcall(function() return nw:at01(100,100) end)
print(a, b)
local c,d = pcall(function() return nw(100,100) end)
print(c, d)
print(ltf)

--@ chunk 236
ltf = function(x,y)
  local d = math.sqrt(((x-270)/640)^2 + ((y-180)/540)^2)
  local v = clamp(1.08 - d*1.15, 0.10, 1)
  return clamp(v * (0.62 + 0.62*nw:at01(x,y)), 0.06, 1)
end
work(rect(0,0,120,60), {hand="broad", pile=test_p, coverage=2, load_at=ltf, edge="found", clip=true})
print("ok1")

--@ chunk 237
work(rect(0,0,120,60), {hand="broad", pile=test_p, tool=flat26, coverage=2, pressure={0.5,0.9}, length={80,230}, edge="found", clip=true})
print("a ok")
work(rect(0,0,120,60), {hand="broad", pile=test_p, tool=flat26, coverage=2, pressure={0.5,0.9}, broken=0.4, clip=true})
print("b ok")

--@ chunk 238
wall_l2 = pile{{"raw umber",2.8},{"burnt sienna",2.2},{"yellow ochre",0.5},{"lead white",0.45},{"bone black",1.8},{"ultramarine blue",0.2}}
work(wall_m, {hand="broad", pile=wall_l2, tool=flat26, coverage=2.6, length={80,230}, pressure={0.5,0.9}, load_at=ltf, edge="found", clip=true, broken=0.4, curve={0.3,0.2}})
print("wall light laid")

--@ chunk 239
-- erase the test patch in the top-left corner
work(rect(0,0,230,100), {hand="broad", pile=wall_dk, tool=flat34, coverage=5, length={90,200}, pressure={0.75,0.95}, edge="found", clip=true})
work(rect(0,0,230,100), {hand="broad", pile=wall_l2, tool=flat26, coverage=2.2, pressure={0.5,0.9}, load_at=ltf, edge="found", clip=true})
tab_d = pile{{"raw umber",3.0},{"burnt sienna",2.2},{"bone black",1.6},{"yellow ochre",0.6}}
work(table_m, {hand="broad", pile=tab_d, tool=flat34, coverage=4.5, length={100,230}, pressure={0.7,0.95}, edge="found", clip=true, broken=0.3, curve={0.3,0.2}})
print("table base")

--@ chunk 240
lt_t = function(x,y) local d = math.sqrt(((x-300)/540)^2 + ((y-650)/300)^2); return clamp(1.0 - d*1.05, 0.07, 1) end
tab_l = pile{{"raw umber",2.2},{"burnt sienna",2.4},{"yellow ochre",1.0},{"lead white",0.6},{"bone black",1.1}}
work(table_m, {hand="broad", pile=tab_l, tool=flat26, coverage=2.8, length={90,240}, pressure={0.5,0.9}, load_at=lt_t, edge="found", clip=true, broken=0.4, curve={0.25,0.2}})
print("table light")

--@ chunk 241
red = pile{{"cadmium red",4},{"lead white",1}}
work(rect(0,0,230,100), {hand="broad", pile=red, tool=flat34, coverage=3, length={90,200}, pressure={0.75,0.95}, edge="found", clip=true})
print("red test", rect(0,0,230,100):area())

--@ chunk 242
print(wait(3*24*60))
for _,p in ipairs({{100,40},{600,200},{200,600},{850,600},{60,26}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 243
work(rect(0,0,235,105), {hand="broad", pile=wall_dk, tool=flat34, coverage=6, length={90,200}, pressure={0.75,1.0}, edge="found", clip=true})
work(rect(0,0,235,105), {hand="broad", pile=wall_l2, tool=flat26, coverage=2.4, pressure={0.5,0.9}, load_at=ltf, edge="found", clip=true})
print("corner fixed", drying(100,50))

--@ chunk 244
work(wall_m, {hand="broad", pile=wall_dk, tool=flat34, coverage=4.5, length={110,240}, pressure={0.8,1.0}, edge="found", clip=true, broken=0.35, curve={0.3,0.2}})
print("wall base 2")

--@ chunk 245
wall_l3 = pile{{"raw umber",2.4},{"burnt sienna",2.6},{"yellow ochre",1.1},{"lead white",1.1},{"bone black",1.2},{"ultramarine blue",0.15}}
work(wall_m, {hand="broad", pile=wall_l3, tool=flat26, coverage=2.2, length={80,230}, pressure={0.5,0.9}, load_at=ltf, edge="found", clip=true, broken=0.4, curve={0.3,0.2}})

--@ chunk 246
work(wall_m, {hand="broad", pile=wall_dk, tool=flat34, coverage=4, length={130,260}, pressure={0.8,1.0}, edge="found", clip=true, broken=0.35, curve={0.3,0.2}})
bigb2 = brush{kind="filbert", width=42}
wall_l4 = pile{{"raw umber",2.7},{"burnt sienna",2.4},{"yellow ochre",0.9},{"lead white",0.5},{"bone black",1.7}}
work(wall_m, {hand="broad", pile=wall_l4, tool=bigb2, coverage=2.0, length={150,330}, pressure={0.55,0.9}, load_at=ltf, edge="found", clip=true, curve={0.35,0.25}, broken=0.25})

--@ chunk 247
wall_dk2 = pile{{"raw umber",3.2},{"burnt sienna",1.6},{"bone black",2.8},{"ultramarine blue",0.5},{"lead white",0.1}}
rtf = function(x,y) return clamp(0.95*clamp((x-450)/450,0,1) + 0.30*clamp((260-y)/320,0,1), 0.04, 1) end
work(wall_m, {hand="broad", pile=wall_dk2, tool=bigb2, coverage=2.0, length={150,330}, pressure={0.6,0.95}, load_at=rtf, edge="found", clip=true, curve={0.35,0.25}, broken=0.25})

--@ chunk 248
work(wall_m, {hand="broad", pile=wall_dk, tool=flat34, coverage=4.5, length={130,260}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.3,0.2}})
wall_l5 = pile{{"raw umber",1.7},{"burnt sienna",2.4},{"yellow ochre",1.6},{"lead white",1.8},{"bone black",0.6}}
work(wall_m, {hand="broad", pile=wall_l5, tool=bigb2, coverage=2.4, length={150,330}, pressure={0.55,0.92}, load_at=ltf, edge="found", clip=true, curve={0.35,0.25}})

--@ chunk 249
work(wall_m, {hand="broad", pile=wall_l4, tool=bigb2, coverage=1.8, length={160,340}, pressure={0.5,0.85}, load_at=ltf, edge="found", clip=true, curve={0.12,0.1}, angle=-0.18})

--@ chunk 250
work(table_m, {hand="broad", pile=tab_d, tool=flat34, coverage=4.5, length={130,260}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.12,0.1}})
tab_l2 = pile{{"raw umber",1.8},{"burnt sienna",2.6},{"yellow ochre",1.5},{"lead white",1.5},{"bone black",0.8}}
work(table_m, {hand="broad", pile=tab_l2, tool=bigb2, coverage=2.2, length={150,330}, pressure={0.55,0.92}, load_at=lt_t, edge="found", clip=true, curve={0.15,0.12}})
tab_d2 = pile{{"raw umber",3.2},{"burnt sienna",2.0},{"bone black",2.4},{"yellow ochre",0.4},{"ultramarine blue",0.3}}
rt_t = function(x,y) return clamp(0.9*clamp((x-480)/430,0,1) + 0.25*clamp((300-y)/350,0,1), 0.04, 1) end
work(table_m, {hand="broad", pile=tab_d2, tool=bigb2, coverage=1.8, length={150,330}, pressure={0.6,0.95}, load_at=rt_t, edge="found", clip=true, curve={0.15,0.12}})

--@ chunk 251
jband = mask(function(x,y) local d = math.abs(y - juncf(x)); return clamp(1 - d/36, 0, 1) end) - bowlm - allq - bottle_m
print("jband", jband:area())
jn = pile{{"raw umber",3.2},{"burnt sienna",1.7},{"bone black",2.6},{"ultramarine blue",0.35},{"lead white",0.1}}
jlf = function(x,y) return clamp(1.05 - 0.55*clamp((x-350)/650,0,1), 0.25, 1) end
work(jband, {hand="broad", pile=jn, tool=bigb2, coverage=2.0, length={140,300}, pressure={0.5,0.9}, load_at=jlf, edge="found", clip=true, curve={0.12,0.1}})

--@ chunk 252
sh_b = (ellipse(330,562, 115, 32) + ellipse(262,548, 55, 20)):soften(20)
sh_bw = (ellipse(560,622, 175, 36) + ellipse(665,600, 75, 26)):soften(24)
sh_3 = ellipse(415,690, 75, 21):soften(15)
sh_4 = ellipse(790,630, 62, 18):soften(14)
allsh = (sh_b + sh_bw + sh_3 + sh_4):grow(8)
sh_pile = pile{{"raw umber",3.4},{"burnt sienna",1.7},{"bone black",2.6},{"ultramarine blue",0.25},{"lead white",0.1}}
shf = function(x,y) return clamp(1.1 - 0.5*clamp((x-300)/600,0,1), 0.3, 1) end
work(allsh, {hand="broad", pile=sh_pile, tool=flat26, coverage=1.3, length={120,260}, pressure={0.3,0.65}, load_at=shf, edge="soft", clip=true, curve={0.15,0.12}})
print("shadows laid")

--@ chunk 253
objs = bowlm + allq + bottle_m
cast = allsh - objs
c_bowl = ((ellipse(530,602,150,26) + ellipse(645,590,62,20)):soften(12)) - bowlm
c_bottle = (ellipse(240,548,58,17):soften(9)) - bottle_m
c_q3 = (ellipse(360,686,44,13):soften(7)) - q3
c_q4 = (ellipse(730,628,40,12):soften(7)) - q4
sh2 = cast + c_bowl + c_bottle + c_q3 + c_q4
print("cast", cast:area(), "contacts", (c_bowl+c_bottle+c_q3+c_q4):area())
work(sh2, {hand="broad", pile=sh_pile, tool=flat26, coverage=1.6, length={120,260}, pressure={0.35,0.7}, load_at=shf, edge="found", clip=true, curve={0.15,0.12}})

--@ chunk 254
bowl_paint = bowlm - allq
print("bowl_paint", bowl_paint:area(), bbox(bowl_paint,4))
rim_e = ellipse(527,481,152,44)
print("rim_e", rim_e:area(), bbox(rim_e,4))
bowl_front = bowl_paint - rim_e
print("bowl_front", bowl_front:area(), bbox(bowl_front,4))
bowl_in = (rim_e - allq)
print("bowl_in", bowl_in:area(), bbox(bowl_in,4))
print("corner checks", bowl_front:at(525,580), bowl_front:at(525,470), bowl_in:at(390,480), bowl_front:at(390,480))

--@ chunk 255
bowl_dk = pile{{"raw umber",3.0},{"burnt sienna",1.3},{"bone black",3.0},{"green earth",0.7},{"ultramarine blue",0.25}}
work(bowl_paint, {hand="broad", pile=bowl_dk, tool=flat26, coverage=4.5, length={70,180}, pressure={0.75,1.0}, edge="found", clip=true, curve={0.25,0.2}})
bowl_lt = pile{{"raw umber",2.2},{"burnt sienna",2.0},{"yellow ochre",0.9},{"lead white",0.9},{"bone black",1.6}}
blf = function(x,y) local d = math.sqrt(((x-420)/230)^2 + ((y-510)/130)^2); return clamp(1.05 - d*1.0, 0.05, 1) end
work(bowl_front, {hand="broad", pile=bowl_lt, tool=flat26, coverage=2.0, length={70,190}, pressure={0.5,0.9}, load_at=blf, edge="found", clip=true, curve={0.25,0.2}})
print("bowl done")

--@ chunk 256
rimpts = {}
for i = 0, 16 do
  local a = math.pi + math.pi * i / 16
  rimpts[#rimpts+1] = {527 + 152*math.cos(a), 481 + 44*math.sin(a)}
end
rimline = outline{pts=rimpts, char="searching", size=8, amount=0.9, lobe=14, edge=2, seed=4}
rim_m = rimline:mask() - allq
print("rim", rim_m:area())
rim_p = pile{{"yellow ochre",2.4},{"burnt sienna",1.6},{"lead white",2.0},{"raw umber",1.0}}
rimlf = function(x,y) return clamp(1.15 - 0.95*clamp((x-400)/280,0,1), 0.08, 1) end
fb8 = brush{kind="filbert", width=8}
work(rim_m, {hand="body", pile=rim_p, tool=fb8, coverage=2.4, pressure={0.5,0.9}, load_at=rimlf, edge="found", clip=true})

--@ chunk 257
work(bowl_paint, {hand="broad", pile=bowl_dk, tool=flat26, coverage=5, length={70,180}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.25,0.2}})
print("bowl re-flattened")

--@ chunk 258
print(wait(36*60))
print("bowl", drying(525,560), "wall", drying(200,200))

--@ chunk 259
work(bowl_front, {hand="broad", pile=bowl_lt, tool=flat26, coverage=2.0, length={70,190}, pressure={0.5,0.9}, load_at=blf, edge="found", clip=true, curve={0.25,0.2}})
-- bounce light from the table onto the bowl's lower front
bowl_bc = pile{{"raw umber",2.4},{"burnt sienna",2.6},{"yellow ochre",1.0},{"lead white",0.5},{"bone black",2.0}}
bcf = function(x,y) return clamp(1.0 - 0.9*clamp((590-y)/150,0,1) - 0.55*clamp((x-400)/330,0,1), 0.03, 1) end
work(bowl_front, {hand="broad", pile=bowl_bc, tool=flat26, coverage=1.5, length={70,190}, pressure={0.4,0.8}, load_at=bcf, edge="found", clip=true, curve={0.2,0.15}})
print("bowl light")

--@ chunk 260
work(bowl_front, {hand="broad", pile=bowl_dk, tool=flat26, coverage=4.5, length={70,180}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.25,0.2}})
bowl_lp = pile{{"raw umber",2.6},{"burnt sienna",1.9},{"yellow ochre",0.7},{"lead white",0.6},{"bone black",2.0}}
blf2 = function(x,y) local d = math.sqrt(((x-410)/250)^2 + ((y-505)/115)^2); return clamp(1.0 - d*0.95, 0.04, 1) end
work(bowl_front, {hand="broad", pile=bowl_lp, tool=bigb2, coverage=1.4, length={90,220}, pressure={0.45,0.85}, load_at=blf2, edge="found", clip=true, curve={0.2,0.15}})
print("bowl front relaid")

--@ chunk 261
print(wait(26*60))
rimpts = {{378,478},{405,496},{440,509},{480,518},{525,523},{568,520},{612,510},{650,496},{678,480}}
rimline = outline{pts=rimpts, char="firm", size=6, amount=0.45, seed=9}
rim_m = rimline:mask() - allq
print("rim", rim_m:area())
rim_p = pile{{"yellow ochre",2.6},{"burnt sienna",2.2},{"lead white",1.4},{"raw umber",1.4}}
rimlf = function(x,y) return clamp(1.2 - 1.0*clamp((x-390)/300,0,1), 0.06, 1) end
fb7 = brush{kind="filbert", width=7}
work(rim_m, {hand="body", pile=rim_p, tool=fb7, coverage=2.2, pressure={0.45,0.85}, load_at=rimlf, edge="found", clip=true})

--@ chunk 262
rb = ribbon(rimpts, 54) - allq
print("rb", rb:area())
work(rb, {hand="broad", pile=bowl_dk, tool=flat26, coverage=4.5, length={70,180}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.25,0.2}})
work(bowl_front, {hand="broad", pile=bowl_lp, tool=bigb2, coverage=1.4, length={90,220}, pressure={0.45,0.85}, load_at=blf2, edge="found", clip=true, curve={0.2,0.15}})
print("knocked back")

--@ chunk 263
rb2 = (ribbon(rimpts, 80) + ellipse(516,502,46,30)) - allq
print("rb2", rb2:area())
work(rb2, {hand="broad", pile=bowl_dk, tool=flat26, coverage=5, length={70,180}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.25,0.2}})
work(bowl_front, {hand="broad", pile=bowl_lp, tool=bigb2, coverage=1.4, length={90,220}, pressure={0.45,0.85}, load_at=blf2, edge="found", clip=true, curve={0.2,0.15}})
print("done")

--@ chunk 264
print(wait(24*60))
-- delicate rim light on the bowl
rim_band = ribbon(rimpts, 7):roughen(2.2, 16, 5) - allq
rim_p2 = pile{{"yellow ochre",2.2},{"burnt sienna",2.4},{"raw umber",2.2},{"lead white",0.9},{"bone black",0.5}}
fb6 = brush{kind="filbert", width=6}
work(rim_band, {hand="body", pile=rim_p2, tool=fb6, coverage=1.3, length={18,45}, pressure={0.4,0.75}, load_at=rimlf, edge="found", clip=true})
print("rim line")

--@ chunk 265
kb = (rim_band:grow(9)) - allq
print("kb", kb:area())
work(kb, {hand="broad", pile=bowl_dk, tool=flat26, coverage=3.5, length={70,180}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.25,0.2}})
work(bowl_front, {hand="broad", pile=bowl_lp, tool=bigb2, coverage=1.4, length={90,220}, pressure={0.45,0.85}, load_at=blf2, edge="found", clip=true, curve={0.2,0.15}})
print("knocked back 2")

--@ chunk 266
print(wait(24*60))
fq_dk = pile{{"bone black",2.6},{"raw umber",3.0},{"yellow ochre",1.2},{"green earth",0.6},{"lead white",0.15}}
fq_1 = pile{{"yellow ochre",3.2},{"raw umber",1.6},{"cadmium yellow",0.5},{"lead white",0.4}}
fq_2 = pile{{"yellow ochre",2.8},{"cadmium yellow",1.8},{"transparent oxide yellow",0.5},{"lead white",1.2}}
fq_3 = pile{{"lead white",2.4},{"cadmium yellow",2.4},{"transparent oxide yellow",0.3}}
fq_bc = pile{{"burnt sienna",2.4},{"yellow ochre",2.4},{"raw umber",1.2},{"lead white",0.3},{"bone black",0.8}}
fruit_b = brush{kind="filbert", width=12}
fruit_b2 = brush{kind="filbert", width=8}
function grad(rx, ry, px, py) return function(x,y)
  local d = math.sqrt(((x-px)/rx)^2 + ((y-py)/ry)^2)
  return clamp(1.0 - d*0.95, 0.02, 1)
end end
function build_fruit(m, cx, cy, r, lx, ly)
  work(m, {hand="broad", pile=fq_dk, tool=fruit_b, coverage=4, length={30,90}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_1, tool=fruit_b, coverage=1.8, length={25,70}, pressure={0.5,0.9}, load_at=grad(r*0.72,r*0.72,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_bc, tool=fruit_b, coverage=1.2, length={20,60}, pressure={0.4,0.8}, load_at=grad(r*0.5,r*0.5,cx+r*0.6,cy+r*0.55), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_2, tool=fruit_b, coverage=1.6, length={20,60}, pressure={0.45,0.85}, load_at=grad(r*0.48,r*0.48,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_3, tool=fruit_b2, coverage=1.4, length={15,45}, pressure={0.45,0.85}, load_at=grad(r*0.28,r*0.28,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
end
build_fruit(q3, 352, 642, 50, 330, 618)
print("q3 built")

--@ chunk 267
build_fruit(q4, 727, 592, 43, 708, 572)
print("q4 built")

--@ chunk 268
work(bowl_paint, {hand="broad", pile=bowl_dk, tool=flat26, coverage=5, length={70,180}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.25,0.2}})
bowl_lp2 = pile{{"raw umber",2.8},{"burnt sienna",1.7},{"yellow ochre",0.6},{"lead white",0.25},{"bone black",2.5}}
work(bowl_front, {hand="broad", pile=bowl_lp2, tool=bigb2, coverage=1.0, length={90,220}, pressure={0.45,0.85}, load_at=blf2, edge="found", clip=true, curve={0.2,0.15}})
bowl_in2 = pile{{"bone black",3.4},{"raw umber",2.6},{"green earth",0.8},{"burnt sienna",0.8}}
work(bowl_in, {hand="broad", pile=bowl_in2, tool=flat26, coverage=1.4, length={60,150}, pressure={0.5,0.9}, edge="found", clip=true, curve={0.25,0.2}})
print("bowl rebuilt dark")

--@ chunk 269
print(drying(525,560))
bowl_lp3 = pile{{"raw umber",2.7},{"burnt sienna",1.9},{"yellow ochre",0.8},{"lead white",0.35},{"bone black",2.2}}
blf3 = function(x,y) local d = math.sqrt(((x-440)/200)^2 + ((y-520)/105)^2); return clamp(1.0 - d*0.95, 0.03, 1) end
bowl_ang = function(x,y) return math.atan(y-500, x-527) + math.pi/2 end
work(bowl_front, {hand="broad", pile=bowl_dk, tool=flat26, coverage=4.5, length={70,180}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.25,0.2}})
work(bowl_front, {hand="broad", pile=bowl_lp3, tool=flat26, coverage=1.3, length={80,190}, pressure={0.5,0.9}, load_at=blf3, angle=bowl_ang, edge="found", clip=true, curve={0.25,0.2}})
print("bowl tangential")

--@ chunk 270
work(bowl_front, {hand="broad", pile=bowl_dk, tool=flat26, coverage=4.5, length={70,180}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.25,0.2}})
work(bowl_front, {hand="broad", pile=bowl_lp3, tool=bigb2, coverage=0.9, length={110,260}, pressure={0.45,0.8}, load_at=blf3, edge="found", clip=true, curve={0.2,0.15}})
print("bowl soft sheen")

--@ chunk 271
bt_dk = pile{{"bone black",3.0},{"green earth",2.2},{"raw umber",2.2},{"ultramarine blue",0.4},{"burnt sienna",0.5}}
bt_l = pile{{"raw umber",2.4},{"green earth",2.0},{"burnt sienna",1.4},{"bone black",1.4},{"lead white",0.35}}
bt_r = pile{{"burnt sienna",2.6},{"raw umber",2.4},{"yellow ochre",0.8},{"lead white",0.4},{"bone black",1.0}}
blf = function(x,y) local d = math.sqrt(((x-196)/62)^2 + ((y-410)/140)^2); return clamp(1.0 - d*0.95, 0.02, 1) end
brf = function(x,y) local d = math.sqrt(((x-282)/48)^2 + ((y-500)/95)^2); return clamp(1.0 - d*0.95, 0.02, 1) end
work(bottle_m, {hand="broad", pile=bt_dk, tool=flat26, coverage=4.5, length={60,160}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(bottle_m, {hand="broad", pile=bt_l, tool=flat26, coverage=1.3, length={60,160}, pressure={0.5,0.9}, load_at=blf, edge="found", clip=true, curve={0.3,0.2}})
work(bottle_m, {hand="broad", pile=bt_r, tool=flat26, coverage=1.1, length={60,160}, pressure={0.45,0.85}, load_at=brf, edge="found", clip=true, curve={0.3,0.2}})
print("bottle base")

--@ chunk 272
sp1 = ((ribbon({{198,358},{192,410},{190,458}}, 18):soften(9)) * bottle_m):grow(2)
sp2 = ((ribbon({{199,378},{194,412},{191,442}}, 7):soften(3)) * bottle_m)
print("sp1", sp1:area(), "sp2", sp2:area())
bt_s1 = pile{{"lead white",1.2},{"green earth",2.0},{"raw umber",1.6},{"bone black",0.8}}
bt_s2 = pile{{"lead white",3.0},{"green earth",1.4},{"yellow ochre",0.8}}
fb9 = brush{kind="filbert", width=9}
work(sp1, {hand="body", pile=bt_s1, tool=fb9, coverage=2.2, length={20,50}, pressure={0.5,0.9}, edge="found", clip=true})
work(sp2, {hand="body", pile=bt_s2, tool=fb7, coverage=1.8, length={15,40}, pressure={0.5,0.9}, edge="found", clip=true})
print("bottle specular")

--@ chunk 273
spb = sp1:grow(14)
print("spb", spb:area())
work(spb, {hand="broad", pile=bt_dk, tool=flat26, coverage=4.5, length={50,130}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.3,0.2}})
blf4 = function(x,y) local d = math.sqrt(((x-193)/42)^2 + ((y-412)/105)^2); return clamp(1.0 - d*0.95, 0.02, 1) end
work(spb, {hand="broad", pile=bt_s1, tool=bigb2, coverage=1.1, length={70,170}, pressure={0.4,0.8}, load_at=blf4, edge="found", clip=true, curve={0.2,0.15}})
print("bottle light redone")

--@ chunk 274
wallrep = spb - bottle_m
print("wallrep", wallrep:area())
work(wallrep, {hand="broad", pile=wall_dk, tool=flat26, coverage=4.5, length={70,180}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(wallrep, {hand="broad", pile=wall_l4, tool=flat26, coverage=1.8, length={70,190}, pressure={0.5,0.9}, load_at=ltf, edge="found", clip=true, curve={0.25,0.2}})
bt_knock = spb * bottle_m
bt_l2 = pile{{"raw umber",2.2},{"green earth",2.4},{"burnt sienna",1.6},{"bone black",2.0},{"lead white",0.3}}
work(bt_knock, {hand="broad", pile=bt_dk, tool=flat26, coverage=3.5, length={50,130}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(bt_knock, {hand="broad", pile=bt_l2, tool=flat26, coverage=1.0, length={60,150}, pressure={0.4,0.85}, load_at=blf4, edge="found", clip=true, curve={0.2,0.15}})
print("repaired")

--@ chunk 275
bt_l3 = pile{{"raw umber",2.6},{"green earth",2.2},{"burnt sienna",1.2},{"bone black",2.4},{"lead white",0.15}}
bt_r2 = pile{{"burnt sienna",3.0},{"raw umber",2.6},{"yellow ochre",0.7},{"bone black",1.6}}
blf5 = function(x,y) local d = math.sqrt(((x-190)/56)^2 + ((y-400)/160)^2); return clamp(1.0 - d*0.95, 0.02, 1) end
brf2 = function(x,y) local d = math.sqrt(((x-280)/45)^2 + ((y-505)/85)^2); return clamp(1.0 - d*0.95, 0.02, 1) end
work(bottle_m, {hand="broad", pile=bt_dk, tool=flat26, coverage=4.5, length={60,160}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(bottle_m, {hand="broad", pile=bt_l3, tool=flat26, coverage=1.1, length={60,160}, pressure={0.5,0.9}, load_at=blf5, edge="found", clip=true, curve={0.25,0.2}})
work(bottle_m, {hand="broad", pile=bt_r2, tool=flat26, coverage=0.9, length={60,160}, pressure={0.45,0.85}, load_at=brf2, edge="found", clip=true, curve={0.25,0.2}})
print("bottle clean")

--@ chunk 276
for _,p in ipairs({{180,420},{190,420},{200,420},{210,420},{220,420},{175,300},{200,300},{180,500},{200,500},{215,500}}) do
  print(p[1], p[2], bottle_m:at(p[1],p[2]))
end
print("col480")
for x = 170, 230, 5 do io_out = nil end
local s = ""
for x = 170, 235, 5 do s = s .. x .. ":" .. string.format("%.2f", bottle_m:at(x,480)) .. " " end
print(s)

--@ chunk 277
bt_dk2 = pile{{"bone black",3.0},{"raw umber",2.6},{"burnt sienna",1.2},{"green earth",0.8},{"ultramarine blue",0.15}}
bt_l4 = pile{{"raw umber",2.4},{"burnt sienna",2.0},{"bone black",1.8},{"green earth",0.6},{"lead white",0.25}}
work(bottle_m, {hand="broad", pile=bt_dk2, tool=flat26, coverage=4.5, length={60,160}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(bottle_m, {hand="broad", pile=bt_l4, tool=flat26, coverage=1.1, length={60,160}, pressure={0.5,0.9}, load_at=blf5, edge="found", clip=true, curve={0.25,0.2}})
work(bottle_m, {hand="broad", pile=bt_r2, tool=flat26, coverage=0.8, length={60,160}, pressure={0.45,0.85}, load_at=brf2, edge="found", clip=true, curve={0.25,0.2}})
print("bottle warm")

--@ chunk 278
bwall = (ellipse(235,420,125,190) + ellipse(240,280,95,85)) * wall_m
print("bwall", bwall:area())
work(bwall, {hand="broad", pile=wall_dk, tool=flat26, coverage=4.5, length={80,200}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.25,0.2}})
work(bwall, {hand="broad", pile=wall_l4, tool=flat26, coverage=2.0, length={80,200}, pressure={0.5,0.9}, load_at=ltf, edge="found", clip=true, curve={0.25,0.2}})
print("wall around bottle")

--@ chunk 279
work(wall_m, {hand="broad", pile=wall_dk, tool=bigb2, coverage=4.5, length={150,320}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.15,0.1}})
work(wall_m, {hand="broad", pile=wall_l4, tool=bigb2, coverage=1.7, length={150,320}, pressure={0.5,0.85}, load_at=ltf, edge="found", clip=true, curve={0.15,0.1}})
work(wall_m, {hand="broad", pile=wall_dk2, tool=bigb2, coverage=1.6, length={150,320}, pressure={0.6,0.9}, load_at=rtf, edge="found", clip=true, curve={0.15,0.1}})
print("wall rebuilt")

--@ chunk 280
ltf3 = function(x,y)
  local d = math.sqrt(((x-270)/640)^2 + ((y-180)/540)^2)
  local v = clamp(1.08 - d*1.15, 0.10, 1)
  return clamp(v * (0.85 + 0.30*nw:at01(x,y)), 0.08, 1)
end
work(wall_m, {hand="broad", pile=wall_dk, tool=bigb2, coverage=4.5, length={150,320}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.15,0.1}})
work(wall_m, {hand="broad", pile=wall_l4, tool=bigb2, coverage=1.8, length={150,320}, pressure={0.5,0.85}, load_at=ltf3, edge="found", clip=true, curve={0.15,0.1}})
work(wall_m, {hand="broad", pile=wall_dk2, tool=bigb2, coverage=1.6, length={150,320}, pressure={0.6,0.9}, load_at=rtf, edge="found", clip=true, curve={0.15,0.1}})
print("wall clean gradient")

--@ chunk 281
work(q3, {hand="broad", pile=fq_dk, tool=fruit_b, coverage=5, length={30,90}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(q4, {hand="broad", pile=fq_dk, tool=fruit_b, coverage=5, length={30,90}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print("fruit flattened")

--@ chunk 282
print(wait(24*60))
local function fruit_light(m, cx, cy, r, lx, ly)
  work(m, {hand="broad", pile=fq_1, tool=fruit_b, coverage=1.5, length={25,70}, pressure={0.5,0.9}, load_at=grad(r*0.78,r*0.78,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
  work(m, {hand="broad", pile=fq_bc, tool=fruit_b, coverage=1.1, length={20,60}, pressure={0.4,0.8}, load_at=grad(r*0.55,r*0.55,cx+r*0.62,cy+r*0.58), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
  work(m, {hand="broad", pile=fq_2, tool=fruit_b, coverage=1.3, length={20,60}, pressure={0.45,0.85}, load_at=grad(r*0.5,r*0.5,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
  work(m, {hand="broad", pile=fq_3, tool=fruit_b2, coverage=1.1, length={15,45}, pressure={0.45,0.85}, load_at=grad(r*0.3,r*0.3,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
end
fruit_light(q3, 352, 642, 50, 330, 618)
fruit_light(q4, 727, 592, 43, 708, 572)
print("fruit modelled")

--@ chunk 283
work(q3, {hand="broad", pile=fq_dk, tool=fruit_b, coverage=5, length={30,90}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(q4, {hand="broad", pile=fq_dk, tool=fruit_b, coverage=5, length={30,90}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(q3, {hand="broad", pile=fq_1, tool=fruit_b, coverage=1.2, length={25,70}, pressure={0.5,0.9}, load_at=grad(36,36,330,618), edge="found", clip=true, curve={0.3,0.2}})
work(q4, {hand="broad", pile=fq_1, tool=fruit_b, coverage=1.2, length={25,70}, pressure={0.5,0.9}, load_at=grad(31,31,708,572), edge="found", clip=true, curve={0.3,0.2}})
print("fruit base+first light")

--@ chunk 284
print(wait(22*60))
work(q3, {hand="broad", pile=fq_2, tool=fruit_b, coverage=1.1, length={20,60}, pressure={0.45,0.85}, load_at=grad(23,23,330,618), edge="found", clip=true, curve={0.3,0.2}})
work(q4, {hand="broad", pile=fq_2, tool=fruit_b, coverage=1.1, length={20,60}, pressure={0.45,0.85}, load_at=grad(20,20,708,572), edge="found", clip=true, curve={0.3,0.2}})
work(q3, {hand="broad", pile=fq_3, tool=fruit_b2, coverage=0.9, length={15,45}, pressure={0.45,0.85}, load_at=grad(12,12,330,618), edge="found", clip=true, curve={0.3,0.2}})
work(q4, {hand="broad", pile=fq_3, tool=fruit_b2, coverage=0.9, length={15,45}, pressure={0.45,0.85}, load_at=grad(10,10,708,572), edge="found", clip=true, curve={0.3,0.2}})
print("fruit lights")

--@ chunk 285
function gmask(r, lx, ly) return mask(function(x,y) return grad(r,r,lx,ly)(x,y) end) end
g1q3 = gmask(44,330,618) * q3
g2q3 = gmask(26,330,618) * q3
g3q3 = gmask(14,330,618) * q3
print("g", g1q3:area(), g2q3:area(), g3q3:area())
work(q3, {hand="broad", pile=fq_dk, tool=fruit_b, coverage=5, length={30,90}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(q3, {hand="broad", pile=fq_1, tool=fb9b, coverage=3.0, length={60,150}, pressure={0.5,0.9}, clip=g1q3, edge="found", curve={0.3,0.2}})
print("q3 gradient 1")

--@ chunk 286
work(q3, {hand="broad", pile=fq_dk, tool=fruit_b, coverage=5, length={30,90}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(q3, {hand="broad", pile=fq_1, tool=fb9b, coverage=2.6, length={70,160}, pressure={0.5,0.9}, clip=gmask(62,330,618)*q3, edge="found", curve={0.3,0.2}})
work(q3, {hand="broad", pile=fq_2, tool=fb9b, coverage=2.0, length={60,140}, pressure={0.45,0.85}, clip=gmask(34,330,618)*q3, edge="found", curve={0.3,0.2}})
work(q3, {hand="broad", pile=fq_3, tool=fruit_b2, coverage=1.6, length={40,90}, pressure={0.45,0.85}, clip=gmask(16,330,618)*q3, edge="found", curve={0.3,0.2}})
print("q3 three steps")

--@ chunk 287
work(q4, {hand="broad", pile=fq_dk, tool=fruit_b, coverage=5, length={30,90}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(q4, {hand="broad", pile=fq_1, tool=fb9b, coverage=2.4, length={70,160}, pressure={0.5,0.9}, clip=gmask(54,708,572)*q4, edge="found", curve={0.3,0.2}})
work(q4, {hand="broad", pile=fq_2, tool=fb9b, coverage=1.8, length={60,140}, pressure={0.45,0.85}, clip=gmask(30,708,572)*q4, edge="found", curve={0.3,0.2}})
work(q4, {hand="broad", pile=fq_3, tool=fruit_b2, coverage=1.4, length={40,90}, pressure={0.45,0.85}, clip=gmask(14,708,572)*q4, edge="found", curve={0.3,0.2}})
print("q4 three steps")

--@ chunk 288
work(bottle_m, {hand="broad", pile=bt_dk2, tool=flat26, coverage=7, length={50,140}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(bottle_m, {hand="broad", pile=bt_l4, tool=flat26, coverage=1.3, length={60,160}, pressure={0.5,0.9}, load_at=blf5, edge="found", clip=true, curve={0.25,0.2}})
work(bottle_m, {hand="broad", pile=bt_r2, tool=flat26, coverage=1.0, length={60,160}, pressure={0.45,0.85}, load_at=brf2, edge="found", clip=true, curve={0.25,0.2}})
print("bottle opaque")

--@ chunk 289
bt_sa = pile{{"raw umber",2.2},{"burnt sienna",1.8},{"lead white",0.8},{"green earth",1.2},{"bone black",1.2}}
bt_sb = pile{{"lead white",1.6},{"yellow ochre",1.2},{"raw umber",1.6},{"bone black",0.6},{"green earth",0.6}}
work(bottle_m, {hand="broad", pile=bt_sa, tool=flat26, coverage=2.4, length={80,200}, pressure={0.5,0.9}, clip=gmask(74,192,400)*bottle_m, edge="found", curve={0.2,0.15}})
work(bottle_m, {hand="broad", pile=bt_sb, tool=flat26, coverage=2.0, length={70,180}, pressure={0.5,0.9}, clip=gmask(30,190,380)*bottle_m, edge="found", curve={0.2,0.15}})
print("bottle light band")

--@ chunk 290
function gel(cx,cy,rx,ry,lx,ly) return mask(function(x,y)
  local d = math.sqrt(((x-lx)/rx)^2 + ((y-ly)/ry)^2)
  return clamp(1.0 - d*0.95, 0, 1)
end) end
bt_sa2 = pile{{"raw umber",2.6},{"burnt sienna",1.8},{"lead white",0.3},{"green earth",1.2},{"bone black",1.8}}
bt_sb2 = pile{{"lead white",1.0},{"yellow ochre",0.9},{"raw umber",2.2},{"bone black",1.0}}
work(bottle_m, {hand="broad", pile=bt_dk2, tool=flat26, coverage=5, length={50,140}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(bottle_m, {hand="broad", pile=bt_sa2, tool=flat26, coverage=2.2, length={80,200}, pressure={0.45,0.85}, clip=gel(240,430,62,190,200,430)*bottle_m, edge="found", curve={0.2,0.15}})
work(bottle_m, {hand="broad", pile=bt_sb2, tool=flat26, coverage=1.6, length={70,180}, pressure={0.45,0.85}, clip=gel(240,420,24,150,196,400)*bottle_m, edge="found", curve={0.2,0.15}})
print("bottle band 2")

--@ chunk 291
work(bottle_m, {hand="broad", pile=bt_dk2, tool=flat26, coverage=5, length={50,140}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(bottle_m, {hand="broad", pile=bt_sa2, tool=flat26, coverage=1.5, length={80,200}, pressure={0.4,0.8}, clip=gel(240,430,52,180,205,430)*bottle_m, edge="found", curve={0.2,0.15}})
sp_pts = {{198,322},{194,368},{191,414},{191,452}}
sp_mask = ribbon(sp_pts, {5,9,10,6}):soften(2) * bottle_m
bt_sp = pile{{"lead white",2.2},{"yellow ochre",1.4},{"raw umber",1.8},{"green earth",0.5},{"bone black",0.5}}
fb10 = brush{kind="filbert", width=10}
fb10:load(bt_sp, 0.95)
fb10:stroke({{200,318},{195,365},{192,410},{192,455}}, {pressure={0.55,0.8,0.75,0.4}, clip=sp_mask})
fb10:reload(bt_sp, 0.9)
fb10:stroke({{196,340},{193,390},{190,430}}, {pressure={0.5,0.8,0.45}, clip=sp_mask})
print("bottle specular 2")

--@ chunk 292
work((sp_mask:grow(9)), {hand="broad", pile=bt_sa2, tool=flat26, coverage=2.6, length={60,150}, pressure={0.5,0.9}, edge="found", clip=true, curve={0.2,0.15}})
bt_sp2 = pile{{"lead white",1.1},{"yellow ochre",1.3},{"raw umber",2.4},{"green earth",0.4},{"bone black",0.9}}
sp_pts2 = {{198,318},{194,360},{191,404},{192,440}}
sp_mask2 = ribbon(sp_pts2, {7,15,16,8}):soften(3) * bottle_m
fb12 = brush{kind="filbert", width=12}
fb12:load(bt_sp2, 0.95)
fb12:stroke({{200,315},{195,358},{192,402},{192,448}}, {pressure={0.45,0.7,0.7,0.35}, clip=sp_mask2})
fb12:reload(bt_sp2, 0.85)
fb12:stroke({{195,350},{192,395},{190,440}}, {pressure={0.4,0.65,0.3}, clip=sp_mask2})
print("bottle specular 3")

--@ chunk 293
bt_mid = pile{{"raw umber",2.4},{"burnt sienna",1.6},{"lead white",0.6},{"green earth",1.0},{"bone black",1.4}}
work((sp_mask2:grow(7)), {hand="broad", pile=bt_mid, tool=flat26, coverage=3.5, length={60,150}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.2,0.15}})
print("bottle spec knocked")

--@ chunk 294
work(bottle_m, {hand="broad", pile=bt_dk2, tool=flat26, coverage=5, length={50,140}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
work(bottle_m, {hand="broad", pile=bt_sa2, tool=flat26, coverage=2.0, length={80,200}, pressure={0.45,0.85}, clip=gel(250,430,66,190,206,430)*bottle_m, edge="found", curve={0.2,0.15}})
work(bottle_m, {hand="broad", pile=bt_mid, tool=flat26, coverage=2.2, length={70,170}, pressure={0.5,0.9}, clip=gel(250,400,26,95,196,398)*bottle_m, edge="found", curve={0.2,0.15}})
work(bottle_m, {hand="broad", pile=bt_sp2, tool=flat26, coverage=1.8, length={60,150}, pressure={0.5,0.9}, clip=gel(240,392,13,24,193,390)*bottle_m, edge="found", curve={0.2,0.15}})
print("bottle final model")

--@ chunk 295
rim_band = ribbon(rimpts, 34) * bowl_front
print("rim_band", rim_band:area())
rl = mask(function(x,y) return clamp(1.15 - 0.95*clamp((x-385)/310,0,1), 0.03, 1) end)
bowl_rim_p = pile{{"raw umber",2.4},{"burnt sienna",1.8},{"yellow ochre",0.7},{"lead white",0.7},{"bone black",1.6}}
work(rim_band, {hand="broad", pile=bowl_rim_p, tool=flat26, coverage=2.4, length={70,170}, pressure={0.5,0.9}, clip=rl*rim_band, edge="found", curve={0.2,0.15}})
print("bowl rim light")

--@ chunk 296
work(rim_band, {hand="broad", pile=bowl_lp3, tool=flat26, coverage=3.2, length={70,170}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.2,0.15}})
rim_band2 = ribbon(rimpts, 19) * bowl_front
print("rim_band2", rim_band2:area())
bowl_rim_p2 = pile{{"raw umber",2.6},{"burnt sienna",2.0},{"yellow ochre",0.8},{"lead white",0.45},{"bone black",1.9}}
work(rim_band2, {hand="broad", pile=bowl_rim_p2, tool=flat26, coverage=2.4, length={60,150}, pressure={0.45,0.85}, clip=rl*rim_band2, edge="found", curve={0.2,0.15}})
print("bowl rim 2")

--@ chunk 297
rim_y = function(x) local t = (x-525); return 523 - 0.00208*t*t end
rimv = function(x,y)
  local h = 1.15 - 0.95*clamp((x-385)/310,0,1)
  local v = 1.15 - 1.15*(y - rim_y(x))/26
  return clamp(h*v, 0, 1)
end
rimv_m = mask(rimv) * bowl_front
print("rimv_m", rimv_m:area())
work(rim_band, {hand="broad", pile=bowl_dk, tool=flat26, coverage=3.0, length={70,170}, pressure={0.7,1.0}, edge="found", clip=true, curve={0.2,0.15}})
work(bowl_front, {hand="broad", pile=bowl_lp3, tool=bigb2, coverage=1.4, length={110,260}, pressure={0.45,0.85}, load_at=blf3, edge="found", clip=true, curve={0.2,0.15}})
work(bowl_front, {hand="broad", pile=bowl_rim_p2, tool=flat26, coverage=3.0, length={60,150}, pressure={0.45,0.85}, clip=rimv_m, edge="found", curve={0.2,0.15}})
print("bowl rim 3")

--@ chunk 298
for _,p in ipairs({{525,560},{525,500},{400,500},{650,540},{525,540},{420,520}}) do
  print(p[1],p[2], rimv_m:at(p[1],p[2]), bowl_front:at(p[1],p[2]))
end

--@ chunk 299
print("lp3", bowl_lp3)
print("rp2", bowl_rim_p2)
print("dk", bowl_dk)
print("bt_dk2", bt_dk2)
print("wall_l4", wall_l4)

--@ chunk 300
bowl_lp4 = pile{{"raw umber",2.9},{"burnt sienna",1.5},{"yellow ochre",0.5},{"lead white",0.15},{"bone black",2.7}}
work(bowl_front, {hand="broad", pile=bowl_dk, tool=flat26, coverage=4.5, length={70,180}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.25,0.2}})
work(bowl_front, {hand="broad", pile=bowl_lp4, tool=flat26, coverage=1.2, length={70,180}, pressure={0.45,0.85}, load_at=blf3, edge="found", clip=true, curve={0.2,0.15}})
work(bowl_front, {hand="broad", pile=bowl_rim_p2, tool=flat26, coverage=1.6, length={60,150}, pressure={0.45,0.85}, clip=rimv_m, edge="found", curve={0.2,0.15}})
print("bowl again")

--@ chunk 301
fq_wd = pile{{"bone black",2.8},{"raw umber",3.4},{"burnt sienna",1.6},{"yellow ochre",0.7}}
function fruit_warm(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*0.85, cy + (cy-ly)*0.85
  work(m, {hand="broad", pile=fq_wd, tool=fruit_b, coverage=1.5, length={60,140}, pressure={0.5,0.9}, clip=gel(cx,cy,r*1.25,r*1.25,shx,shy)*m, edge="found", curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_1, tool=fb9b, coverage=1.2, length={70,160}, pressure={0.5,0.9}, clip=gmask(r*1.3,lx,ly)*m, edge="found", curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_3, tool=fruit_b2, coverage=1.8, length={40,90}, pressure={0.45,0.85}, clip=gmask(r*0.32,lx,ly)*m, edge="found", curve={0.3,0.2}})
end
fruit_warm(q1, 467, 467, 45, 447, 446)
fruit_warm(q2, 582, 477, 40, 563, 456)
fruit_warm(q3, 352, 642, 50, 330, 618)
fruit_warm(q4, 727, 592, 43, 708, 572)
print("fruit warmed")

--@ chunk 302
fq_dk2 = pile{{"bone black",2.6},{"raw umber",3.2},{"yellow ochre",1.4},{"burnt sienna",0.8},{"green earth",0.2}}
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_dk2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print("fruit flattened warm")

--@ chunk 303
print(wait(24*60))
local function fruit_light3(m, cx, cy, r, lx, ly)
  work(m, {hand="broad", pile=fq_1, tool=fb9b, coverage=2.6, length={70,160}, pressure={0.5,0.9}, clip=gmask(r*1.25,lx,ly)*m, edge="found", curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_2, tool=fb9b, coverage=2.0, length={60,140}, pressure={0.45,0.85}, clip=gmask(r*0.62,lx,ly)*m, edge="found", curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_3, tool=fruit_b2, coverage=1.6, length={40,90}, pressure={0.45,0.85}, clip=gmask(r*0.3,lx,ly)*m, edge="found", curve={0.3,0.2}})
end
fruit_light3(q1, 467, 467, 45, 447, 446)
fruit_light3(q2, 582, 477, 40, 563, 456)
fruit_light3(q3, 352, 642, 50, 330, 618)
fruit_light3(q4, 727, 592, 43, 708, 572)
print("fruit lights 3")

--@ chunk 304
fq_mid = pile{{"yellow ochre",2.6},{"raw umber",2.4},{"cadmium yellow",0.5},{"bone black",0.6},{"lead white",0.3}}
fq_l2 = pile{{"yellow ochre",2.4},{"cadmium yellow",1.4},{"transparent oxide yellow",0.5},{"lead white",1.4}}
fq_sd = pile{{"raw umber",3.2},{"burnt sienna",1.8},{"bone black",1.4},{"yellow ochre",0.4}}
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_mid, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print("fruit mid base")

--@ chunk 305
print(wait(22*60))
local function fruit_model(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*0.9, cy + (cy-ly)*0.9
  work(m, {hand="broad", pile=fq_l2, tool=fb9b, coverage=2.4, length={70,160}, pressure={0.5,0.9}, clip=gmask(r*1.1,lx,ly)*m, edge="found", curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_sd, tool=fb9b, coverage=2.2, length={70,160}, pressure={0.5,0.9}, clip=gmask(r*1.1,shx,shy)*m, edge="found", curve={0.3,0.2}})
end
fruit_model(q1, 467, 467, 45, 447, 446)
fruit_model(q2, 582, 477, 40, 563, 456)
fruit_model(q3, 352, 642, 50, 330, 618)
fruit_model(q4, 727, 592, 43, 708, 572)
print("fruit modelled 2")

--@ chunk 306
work(q3, {hand="broad", pile=fq_mid, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
covn = function(r, px, py, lo, hi) return function(x,y) return lo + (hi-lo)*grad(r,r,px,py)(x,y) end end
stipple(q3, {pile=fq_l2, width=4, coverage=covn(58,330,618,0.5,3.0), pressure={0.55,0.85}, feather=0.3, cluster=0.4, clip=true})
print("q3 stipple test")

--@ chunk 307
for _,m in ipairs({q1,q2,q4}) do
  work(m, {hand="broad", pile=fq_mid, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(20*60))
local function fruit_stip(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*0.95, cy + (cy-ly)*0.95
  stipple(m, {pile=fq_l2, width=4, coverage=covn(r*1.15,lx,ly,0.5,3.0), pressure={0.55,0.85}, feather=0.3, cluster=0.4, clip=true})
  stipple(m, {pile=fq_sd, width=4, coverage=covn(r*1.15,shx,shy,0.4,2.6), pressure={0.55,0.85}, feather=0.3, cluster=0.4, clip=true})
  stipple(m, {pile=fq_3, width=3, coverage=covn(r*0.36,lx,ly,0,2.0), pressure={0.6,0.9}, feather=0.4, cluster=0.3, clip=true})
end
fruit_stip(q1, 467, 467, 45, 447, 446)
fruit_stip(q2, 582, 477, 40, 563, 456)
fruit_stip(q3, 352, 642, 50, 330, 618)
fruit_stip(q4, 727, 592, 43, 708, 572)
print("fruit stippled")

--@ chunk 308
print(wait(20*60))
local function fruit_shadow(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*0.95, cy + (cy-ly)*0.95
  stipple(m, {pile=fq_sd, width=4, coverage=covn(r*1.2,shx,shy,0.1,2.4), pressure={0.55,0.85}, feather=0.35, cluster=0.4, clip=true})
end
fruit_shadow(q1, 467, 467, 45, 447, 446)
fruit_shadow(q2, 582, 477, 40, 563, 456)
fruit_shadow(q3, 352, 642, 50, 330, 618)
fruit_shadow(q4, 727, 592, 43, 708, 572)
print("fruit shadows")

--@ chunk 309
fq_sd2 = pile{{"raw umber",3.0},{"yellow ochre",1.2},{"bone black",2.2},{"green earth",0.3}}
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(22*60))
local function fruit_lit(m, cx, cy, r, lx, ly)
  stipple(m, {pile=fq_l2, width=4, coverage=covn(r*1.35,lx,ly,0.0,3.6), pressure={0.55,0.85}, feather=0.4, cluster=0.3, clip=true})
end
fruit_lit(q1, 467, 467, 45, 447, 446)
fruit_lit(q2, 582, 477, 40, 563, 456)
fruit_lit(q3, 352, 642, 50, 330, 618)
fruit_lit(q4, 727, 592, 43, 708, 572)
print("fruit lit")

--@ chunk 310
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_mid, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(22*60))
local function fruit_lit2(m, cx, cy, r, lx, ly)
  stipple(m, {pile=fq_l2, width=4, coverage=covn(r*1.15,lx,ly,0.5,3.0), pressure={0.55,0.85}, feather=0.3, cluster=0.4, clip=true})
end
fruit_lit2(q1, 467, 467, 45, 447, 446)
fruit_lit2(q2, 582, 477, 40, 563, 456)
fruit_lit2(q3, 352, 642, 50, 330, 618)
fruit_lit2(q4, 727, 592, 43, 708, 572)
print("fruit lit2")

--@ chunk 311
fq_mid2 = pile{{"yellow ochre",2.8},{"raw umber",1.8},{"cadmium yellow",0.6},{"bone black",0.3},{"lead white",0.4}}
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_mid2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(22*60))
local function fruit_lit3(m, cx, cy, r, lx, ly)
  stipple(m, {pile=fq_l2, width=3.5, coverage=covn(r*1.2,lx,ly,0.4,2.2), pressure={0.6,0.85}, feather=0.3, cluster=0, dips={10,0.9,0.35}, clip=true})
end
fruit_lit3(q1, 467, 467, 45, 447, 446)
fruit_lit3(q2, 582, 477, 40, 563, 456)
fruit_lit3(q3, 352, 642, 50, 330, 618)
fruit_lit3(q4, 727, 592, 43, 708, 572)
print("fruit lit3")

--@ chunk 312
print(wait(16*60))
local function fruit_lit4(m, cx, cy, r, lx, ly)
  stipple(m, {pile=fq_l2, width=4.5, coverage=covn(r*1.2,lx,ly,0.3,1.9), pressure={0.5,0.8}, feather=0.35, cluster=0, clip=true})
end
fruit_lit4(q1, 467, 467, 45, 447, 446)
fruit_lit4(q2, 582, 477, 40, 563, 456)
fruit_lit4(q3, 352, 642, 50, 330, 618)
fruit_lit4(q4, 727, 592, 43, 708, 572)
print("fruit lit4")

--@ chunk 313
print(wait(18*60))
work(q3, {hand="broad", pile=fq_mid2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(20*60))
stipple(q3, {pile=fq_l2, width=4, coverage=covn(62,330,618,0.15,6.5), pressure={0.5,0.8}, feather=0.35, cluster=0, clip=true})
print("q3 test high cov")

--@ chunk 314
print(wait(20*60))
work(q3, {hand="broad", pile=fq_mid2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(20*60))
stipple(q3, {pile=fq_l2, width=4, coverage=3.0, pressure={0.5,0.8}, cluster=0, clip=true})
print("q3 const cov")

--@ chunk 315
work(q3, {hand="broad", pile=fq_mid2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(20*60))
z_l = ellipse(338,620,40,36):soften(13) * q3
z_d = ellipse(382,670,36,30):soften(12) * q3
work(z_l, {hand="broad", pile=fq_l2, tool=fruit_b, coverage=2.4, length={25,60}, pressure={0.55,0.9}, edge="found", clip=true, curve={0.3,0.2}})
work(z_d, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=2.2, length={25,60}, pressure={0.55,0.9}, edge="found", clip=true, curve={0.3,0.2}})
print("q3 soft zones")

--@ chunk 316
fq_sdm = pile{{"raw umber",2.8},{"yellow ochre",1.4},{"bone black",1.2}}
z_d = (ellipse(378,666,34,30):soften(18)) * q3
z_l = (ellipse(334,616,34,30):soften(12)) * q3
work(z_d, {hand="broad", pile=fq_sdm, tool=fruit_b, coverage=2.4, length={25,60}, pressure={0.55,0.9}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(16*60))
work(z_l, {hand="broad", pile=fq_l2, tool=fruit_b, coverage=2.2, length={22,55}, pressure={0.55,0.9}, edge="found", clip=true, curve={0.3,0.2}})
print("q3 refined")

--@ chunk 317
fq_lt = pile{{"yellow ochre",2.6},{"cadmium yellow",1.0},{"transparent oxide yellow",0.4},{"lead white",1.0}}
fq_sk = pile{{"raw umber",3.0},{"yellow ochre",2.0},{"bone black",0.9}}
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_mid2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(20*60))
local function fruit_mod(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*1.0, cy + (cy-ly)*1.0
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_sk, tool=fruit_b, coverage=1.8, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,shx,shy), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
end
fruit_mod(q1, 467, 467, 45, 447, 446)
fruit_mod(q2, 582, 477, 40, 563, 456)
fruit_mod(q3, 352, 642, 50, 330, 618)
fruit_mod(q4, 727, 592, 43, 708, 572)
print("fruit modelled 3")

--@ chunk 318
fq_dim = pile{{"raw umber",2.6},{"yellow ochre",2.2},{"bone black",0.9},{"green earth",0.3}}
work(q4, {hand="broad", pile=fq_dim, tool=fruit_b, coverage=1.8, length={22,55}, pressure={0.5,0.9}, load_at=grad(60,60,700,566), edge="found", clip=true, curve={0.3,0.2}})
blend(q4, {})
work(q2, {hand="broad", pile=fq_dim, tool=fruit_b, coverage=0.9, length={22,55}, pressure={0.4,0.85}, load_at=grad(52,52,556,452), edge="found", clip=true, curve={0.3,0.2}})
blend(q2, {})
print("fruit values set")

--@ chunk 319
wall_mid = pile{{"raw umber",2.8},{"burnt sienna",2.2},{"bone black",1.4},{"yellow ochre",0.5}}
wall_lt2 = pile{{"raw umber",2.2},{"burnt sienna",2.6},{"yellow ochre",1.2},{"lead white",0.8},{"bone black",0.8}}
wall_dk3 = pile{{"raw umber",3.0},{"burnt sienna",1.6},{"bone black",2.6},{"ultramarine blue",0.4}}
work(wall_m, {hand="broad", pile=wall_mid, tool=bigb2, coverage=5, length={150,320}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
work(wall_m, {hand="broad", pile=wall_lt2, tool=bigb2, coverage=1.8, length={150,320}, pressure={0.5,0.85}, load_at=ltf3, edge="found", clip=true, curve={0.15,0.1}})
work(wall_m, {hand="broad", pile=wall_dk3, tool=bigb2, coverage=1.6, length={150,320}, pressure={0.6,0.9}, load_at=rtf, edge="found", clip=true, curve={0.15,0.1}})
blend(wall_m, {})
print("wall clean light")

--@ chunk 320
ltf4 = function(x,y)
  local d = math.sqrt(((x-270)/640)^2 + ((y-180)/540)^2)
  return clamp(1.08 - d*1.15, 0.10, 1)
end
work(wall_m, {hand="broad", pile=wall_mid, tool=bigb2, coverage=5, length={150,320}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
work(wall_m, {hand="broad", pile=wall_lt2, tool=bigb2, coverage=1.8, length={150,320}, pressure={0.5,0.85}, load_at=ltf4, edge="found", clip=true, curve={0.15,0.1}})
work(wall_m, {hand="broad", pile=wall_dk3, tool=bigb2, coverage=1.6, length={150,320}, pressure={0.6,0.9}, load_at=rtf, edge="found", clip=true, curve={0.15,0.1}})
blend(wall_m, {})
print("wall smooth")

--@ chunk 321
work(wall_m, {hand="broad", pile=wall_dk, tool=bigb2, coverage=5, length={150,320}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
cw = function(x,y) return 0.25 + 2.8*ltf4(x,y) end
stipple(wall_m, {pile=wall_lt2, width=7, coverage=cw, pressure={0.5,0.8}, cluster=0, clip=true})
print("wall stippled")

--@ chunk 322
work(wall_m, {hand="broad", pile=wall_mid, tool=bigb2, coverage=6, length={150,320}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
wall_lt3 = pile{{"raw umber",2.4},{"burnt sienna",2.5},{"yellow ochre",1.0},{"lead white",0.4},{"bone black",1.0}}
wall_dk4 = pile{{"raw umber",3.0},{"burnt sienna",1.9},{"bone black",2.2},{"ultramarine blue",0.3}}
work(wall_m, {hand="broad", pile=wall_lt3, tool=bigb2, coverage=1.8, length={150,320}, pressure={0.5,0.85}, load_at=ltf4, edge="found", clip=true, curve={0.15,0.1}})
work(wall_m, {hand="broad", pile=wall_dk4, tool=bigb2, coverage=1.6, length={150,320}, pressure={0.6,0.9}, load_at=rtf, edge="found", clip=true, curve={0.15,0.1}})
print("wall low contrast")

--@ chunk 323
tab_mid = pile{{"raw umber",2.8},{"burnt sienna",2.4},{"bone black",1.2},{"yellow ochre",0.7}}
work(table_m, {hand="broad", pile=tab_mid, tool=bigb2, coverage=5, length={130,300}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
tab_lt = pile{{"raw umber",2.2},{"burnt sienna",2.6},{"yellow ochre",1.3},{"lead white",0.6},{"bone black",0.6}}
tab_dk3 = pile{{"raw umber",3.0},{"burnt sienna",2.0},{"bone black",2.2},{"yellow ochre",0.3},{"ultramarine blue",0.3}}
work(table_m, {hand="broad", pile=tab_lt, tool=bigb2, coverage=1.8, length={130,300}, pressure={0.5,0.85}, load_at=lt_t, edge="found", clip=true, curve={0.15,0.1}})
work(table_m, {hand="broad", pile=tab_dk3, tool=bigb2, coverage=1.6, length={130,300}, pressure={0.6,0.9}, load_at=rt_t, edge="found", clip=true, curve={0.15,0.1}})
print("table clean")

--@ chunk 324
print("sh2", sh2:area(), "jband", jband:area())
work(jband, {hand="broad", pile=jn, tool=bigb2, coverage=1.8, length={140,300}, pressure={0.5,0.9}, load_at=jlf, edge="found", clip=true, curve={0.12,0.1}})
work(sh2, {hand="broad", pile=sh_pile, tool=flat26, coverage=1.7, length={120,260}, pressure={0.35,0.7}, load_at=shf, edge="found", clip=true, curve={0.15,0.12}})
print("shadows and junction")

--@ chunk 325
bt_green = pile{{"bone black",3.0},{"green earth",3.0},{"raw umber",1.6},{"ultramarine blue",0.5}}
bt_gl = pile{{"green earth",2.6},{"raw umber",1.8},{"lead white",0.5},{"bone black",1.2}}
bt_gsp = pile{{"lead white",1.8},{"green earth",1.8},{"yellow ochre",0.8},{"raw umber",1.0}}
work(bottle_m, {hand="broad", pile=bt_green, tool=flat26, coverage=6, length={50,140}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(20*60))
work(bottle_m, {hand="broad", pile=bt_gl, tool=flat26, coverage=1.8, length={70,180}, pressure={0.5,0.9}, load_at=blf5, edge="found", clip=true, curve={0.25,0.2}})
work(bottle_m, {hand="broad", pile=bt_r2, tool=flat26, coverage=1.0, length={60,160}, pressure={0.45,0.85}, load_at=brf2, edge="found", clip=true, curve={0.25,0.2}})
work(bottle_m, {hand="broad", pile=bt_gsp, tool=flat26, coverage=1.6, length={60,150}, pressure={0.5,0.9}, clip=gel(250,392,26,80,196,392)*bottle_m, edge="found", curve={0.2,0.15}})
print("bottle green")

--@ chunk 326
bt_grn2 = pile{{"bone black",3.6},{"green earth",2.2},{"raw umber",1.8},{"ultramarine blue",0.2}}
bt_gl2 = pile{{"green earth",2.6},{"raw umber",1.6},{"bone black",2.0},{"lead white",0.3}}
bt_gsp2 = pile{{"green earth",2.0},{"lead white",1.2},{"raw umber",1.4},{"yellow ochre",0.6}}
work(bottle_m, {hand="broad", pile=bt_grn2, tool=flat26, coverage=6, length={50,140}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(20*60))
work(bottle_m, {hand="broad", pile=bt_gl2, tool=flat26, coverage=1.6, length={70,180}, pressure={0.5,0.9}, load_at=blf5, edge="found", clip=true, curve={0.25,0.2}})
work(bottle_m, {hand="broad", pile=bt_r2, tool=flat26, coverage=0.8, length={60,160}, pressure={0.45,0.85}, load_at=brf2, edge="found", clip=true, curve={0.25,0.2}})
bt_sp_m = ellipse(196,392,11,38):soften(5) * bottle_m
work(bottle_m, {hand="broad", pile=bt_gsp2, tool=flat26, coverage=2.2, length={50,130}, pressure={0.5,0.9}, clip=bt_sp_m, edge="found", curve={0.2,0.15}})
print("bottle green 2")

--@ chunk 327
bt_gsp3 = pile{{"green earth",2.2},{"lead white",0.7},{"raw umber",1.8},{"yellow ochre",0.4}}
work(bottle_m, {hand="broad", pile=bt_grn2, tool=flat26, coverage=6, length={50,140}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(20*60))
work(bottle_m, {hand="broad", pile=bt_gl2, tool=flat26, coverage=2.2, length={70,180}, pressure={0.5,0.9}, load_at=blf5, edge="found", clip=true, curve={0.25,0.2}})
work(bottle_m, {hand="broad", pile=bt_r2, tool=flat26, coverage=1.0, length={60,160}, pressure={0.45,0.85}, load_at=brf2, edge="found", clip=true, curve={0.25,0.2}})
bt_sp_m = ellipse(196,390,8,24):soften(5) * bottle_m
work(bottle_m, {hand="broad", pile=bt_gsp3, tool=flat26, coverage=2.4, length={40,110}, pressure={0.5,0.9}, clip=bt_sp_m, edge="found", curve={0.2,0.15}})
print("bottle green 3")

--@ chunk 328
blf6 = function(x,y) local d = math.sqrt(((x-205)/85)^2 + ((y-420)/210)^2); return clamp(1.0 - d*0.9, 0.12, 1) end
bt_gsp4 = pile{{"green earth",2.4},{"lead white",0.5},{"raw umber",2.0},{"yellow ochre",0.4}}
work(bottle_m, {hand="broad", pile=bt_grn2, tool=flat26, coverage=6, length={50,140}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(20*60))
work(bottle_m, {hand="broad", pile=bt_gl2, tool=flat26, coverage=2.2, length={80,200}, pressure={0.45,0.85}, load_at=blf6, edge="found", clip=true, curve={0.2,0.15}})
work(bottle_m, {hand="broad", pile=bt_r2, tool=flat26, coverage=1.0, length={60,160}, pressure={0.45,0.85}, load_at=brf2, edge="found", clip=true, curve={0.2,0.15}})
bt_sp_m = (ellipse(196,388,9,30):soften(9)) * bottle_m
work(bottle_m, {hand="broad", pile=bt_gsp4, tool=flat26, coverage=1.6, length={40,110}, pressure={0.4,0.8}, clip=bt_sp_m, edge="found", curve={0.2,0.15}})
print("bottle green 4")

--@ chunk 329
ltt = function(x,y) local d = math.sqrt(((x-270)/330)^2 + ((y-660)/210)^2); return clamp(1.0 - d*0.85, 0.12, 1) end
tab_lt2 = pile{{"raw umber",1.8},{"burnt sienna",2.8},{"yellow ochre",1.6},{"lead white",1.2}}
work(table_m, {hand="broad", pile=tab_lt2, tool=bigb2, coverage=1.8, length={130,300}, pressure={0.5,0.85}, load_at=ltt, edge="found", clip=true, curve={0.15,0.1}})
bowl_in_l = pile{{"raw umber",2.8},{"burnt sienna",1.8},{"bone black",2.2},{"yellow ochre",0.5}}
in_lf = function(x,y) local d = math.sqrt(((x-430)/150)^2 + ((y-470)/70)^2); return clamp(1.0 - d*0.9, 0.08, 1) end
work(bowl_in, {hand="broad", pile=bowl_in_l, tool=flat26, coverage=1.4, length={50,130}, pressure={0.4,0.8}, load_at=in_lf, edge="found", clip=true, curve={0.2,0.15}})
print("table light and bowl interior")

--@ chunk 330
work(table_m, {hand="broad", pile=tab_mid, tool=bigb2, coverage=6, length={130,300}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
tab_lt3 = pile{{"raw umber",2.2},{"burnt sienna",2.5},{"yellow ochre",1.2},{"lead white",0.5},{"bone black",0.8}}
work(table_m, {hand="broad", pile=tab_lt3, tool=bigb2, coverage=1.6, length={130,300}, pressure={0.5,0.85}, load_at=ltt, edge="found", clip=true, curve={0.15,0.1}})
work(table_m, {hand="broad", pile=tab_dk3, tool=bigb2, coverage=1.4, length={130,300}, pressure={0.6,0.9}, load_at=rt_t, edge="found", clip=true, curve={0.15,0.1}})
print("table retoned")

--@ chunk 331
work(sh2, {hand="broad", pile=sh_pile, tool=flat26, coverage=1.7, length={120,260}, pressure={0.35,0.7}, load_at=shf, edge="found", clip=true, curve={0.15,0.12}})
-- a warmer reflected glow on the table around the bowl and fruit
glow = pile{{"burnt sienna",3.0},{"yellow ochre",1.4},{"raw umber",1.6},{"lead white",0.4}}
gf = function(x,y) local d = math.sqrt(((x-470)/230)^2 + ((y-640)/70)^2); return clamp(1.0 - d*0.9, 0.08, 1) end
glow_m = (ellipse(470,640,230,70):soften(30)) - bowlm - allq - bottle_m
work(glow_m, {hand="broad", pile=glow, tool=bigb2, coverage=1.2, length={90,220}, pressure={0.4,0.8}, load_at=gf, edge="found", clip=true, curve={0.2,0.15}})
print("shadows and glow")

--@ chunk 332
sh_pile2 = pile{{"raw umber",3.0},{"burnt sienna",1.6},{"bone black",3.0},{"yellow ochre",0.3}}
S1 = (ellipse(330,562,112,30) + ellipse(262,548,52,18)):soften(12)
S2 = (ellipse(560,620,170,32) + ellipse(662,600,70,22)):soften(14)
S3 = ellipse(412,688,72,19):soften(9)
S4 = ellipse(786,628,58,16):soften(8)
C1 = (ellipse(530,600,150,24) + ellipse(645,588,60,18)):soften(10) - bowlm
C2 = ellipse(240,546,56,15):soften(8) - bottle_m
C3 = ellipse(360,684,42,12):soften(6) - q3
C4 = ellipse(730,626,38,11):soften(6) - q4
sh3 = (S1+S2+S3+S4) - bowlm - allq - bottle_m + C1 + C2 + C3 + C4
print("sh3", sh3:area())
work(sh3, {hand="broad", pile=sh_pile2, tool=flat26, coverage=2.2, length={110,240}, pressure={0.4,0.8}, load_at=shf, edge="found", clip=true, curve={0.15,0.12}})
print("strong shadows")

--@ chunk 333
fq_hi = pile{{"lead white",2.2},{"cadmium yellow",2.2},{"transparent oxide yellow",0.3}}
b5 = brush{kind="filbert", width=6}
b5:load(fq_hi, 0.9)
b5:stroke({{439,440},{447,444},{453,449}}, {pressure={0.55,0.85,0.45}, clip=q1})
b5:reload(fq_hi, 0.85)
b5:stroke({{556,450},{562,454},{568,459}}, {pressure={0.55,0.85,0.45}, clip=q2})
b5:reload(fq_hi, 0.85)
b5:stroke({{322,612},{330,616},{336,621}}, {pressure={0.55,0.85,0.45}, clip=q3})
b5:reload(fq_hi, 0.85)
b5:stroke({{700,566},{707,570},{713,575}}, {pressure={0.55,0.85,0.45}, clip=q4})
print("fruit highlights")

--@ chunk 334
function soften_hi(m, lx, ly, rx, ry)
  local z = ellipse(lx,ly,rx,ry):soften(4) * m
  work(m, {hand="broad", pile=fq_l2, tool=fruit_b, coverage=2.2, length={25,60}, pressure={0.5,0.9}, clip=z, edge="found", curve={0.3,0.2}})
end
soften_hi(q1, 446, 445, 22, 16)
soften_hi(q2, 562, 455, 20, 14)
soften_hi(q3, 329, 616, 22, 16)
soften_hi(q4, 707, 571, 20, 14)
print("dashes softened")

--@ chunk 335
b4 = brush{kind="filbert", width=5}
fq_hi2 = pile{{"lead white",2.6},{"cadmium yellow",2.4},{"transparent oxide yellow",0.2}}
b4:load(fq_hi2, 0.95)
b4:touch(445, 444, {pressure=0.75, drag={1.6,0.8}, clip=q1})
b4:reload(fq_hi2, 0.9)
b4:touch(561, 454, {pressure=0.7, drag={1.4,0.7}, clip=q2})
b4:reload(fq_hi2, 0.9)
b4:touch(328, 615, {pressure=0.8, drag={1.8,0.9}, clip=q3})
b4:reload(fq_hi2, 0.9)
b4:touch(706, 570, {pressure=0.7, drag={1.5,0.75}, clip=q4})
print("round highlights")

--@ chunk 336
function soften_hi2(m, lx, ly, rx, ry)
  local z = ellipse(lx,ly,rx,ry):soften(6) * m
  work(m, {hand="broad", pile=fq_l2, tool=fruit_b, coverage=2.6, length={25,60}, pressure={0.55,0.9}, clip=z, edge="found", curve={0.3,0.2}})
end
soften_hi2(q1, 444, 444, 26, 20)
soften_hi2(q2, 561, 454, 24, 18)
soften_hi2(q3, 327, 615, 26, 20)
soften_hi2(q4, 706, 570, 24, 18)
print("blobs knocked back")

--@ chunk 337
function soften_hi3(m, lx, ly, rx, ry)
  local z = ellipse(lx,ly,rx,ry):soften(8) * m
  work(m, {hand="broad", pile=fq_l2, tool=fruit_b, coverage=4.0, length={25,60}, pressure={0.6,0.95}, clip=z, edge="found", curve={0.3,0.2}})
end
soften_hi3(q1, 444, 444, 30, 24)
soften_hi3(q2, 561, 454, 28, 22)
soften_hi3(q3, 327, 615, 30, 24)
soften_hi3(q4, 706, 570, 28, 22)
print("knocked harder")

--@ chunk 338
print(wait(20*60))
function soften_hi4(m, lx, ly, rx, ry)
  local z = ellipse(lx,ly,rx,ry):soften(10) * m
  print("z", z:area(), z:at(lx,ly))
  work(m, {hand="broad", pile=fq_mid2, tool=fruit_b, coverage=5, length={25,60}, pressure={0.7,1.0}, clip=z, edge="found", curve={0.3,0.2}})
end
soften_hi4(q1, 444, 444, 32, 26)
soften_hi4(q2, 561, 454, 30, 24)
soften_hi4(q3, 327, 615, 32, 26)
soften_hi4(q4, 706, 570, 30, 24)
print("knocked with base")

--@ chunk 339
local function fruit_sh2(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*1.0, cy + (cy-ly)*1.0
  work(m, {hand="broad", pile=fq_sk, tool=fruit_b, coverage=1.5, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,shx,shy), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
end
fruit_sh2(q1, 467, 467, 45, 447, 446)
fruit_sh2(q2, 582, 477, 40, 563, 456)
fruit_sh2(q3, 352, 642, 50, 330, 618)
fruit_sh2(q4, 727, 592, 43, 708, 572)
print("fruit shadows 2")

--@ chunk 340
fq_warm = pile{{"burnt sienna",2.6},{"transparent oxide yellow",1.4},{"raw umber",1.4},{"lead white",0.3},medium=0.12}
local function fruit_warm2(m, cx, cy, r, lx, ly)
  work(m, {hand="broad", pile=fq_warm, tool=fruit_b, coverage=1.2, length={25,60}, pressure={0.45,0.85}, load_at=grad(r*1.25,r*1.25,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
end
fruit_warm2(q1, 467, 467, 45, 447, 446)
fruit_warm2(q2, 582, 477, 40, 563, 456)
fruit_warm2(q3, 352, 642, 50, 330, 618)
fruit_warm2(q4, 727, 592, 43, 708, 572)
print("fruit warmed 2")

--@ chunk 341
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_mid2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(20*60))
local function fruit_final(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*1.0, cy + (cy-ly)*1.0
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_sk, tool=fruit_b, coverage=1.8, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,shx,shy), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
end
fruit_final(q1, 467, 467, 45, 447, 446)
fruit_final(q2, 582, 477, 40, 563, 456)
fruit_final(q3, 352, 642, 50, 330, 618)
fruit_final(q4, 727, 592, 43, 708, 572)
print("fruit final")

--@ chunk 342
local function fruit_val(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*1.0, cy + (cy-ly)*1.0
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=1.6, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.3,r*1.3,shx,shy), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=1.2, length={20,50}, pressure={0.45,0.85}, load_at=grad(r*1.0,r*1.0,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
end
fruit_val(q1, 467, 467, 45, 447, 446)
fruit_val(q2, 582, 477, 40, 563, 456)
fruit_val(q3, 352, 642, 50, 330, 618)
fruit_val(q4, 727, 592, 43, 708, 572)
print("fruit values")

--@ chunk 343
local function fruit_val2(m, cx, cy, r, lx, ly)
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=2.4, length={30,80}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=1.5, length={20,50}, pressure={0.45,0.85}, load_at=grad(r*1.0,r*1.0,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
end
fruit_val2(q1, 467, 467, 45, 447, 446)
fruit_val2(q2, 582, 477, 40, 563, 456)
fruit_val2(q3, 352, 642, 50, 330, 618)
fruit_val2(q4, 727, 592, 43, 708, 572)
print("fruit values 2")

--@ chunk 344
fq_mid3 = pile{{"yellow ochre",2.8},{"raw umber",2.2},{"cadmium yellow",0.6},{"bone black",0.5},{"lead white",0.3}}
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_mid3, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(20*60))
local function fruit_pass(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*1.0, cy + (cy-ly)*1.0
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_sk, tool=fruit_b, coverage=1.8, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,shx,shy), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
end
fruit_pass(q1, 467, 467, 45, 447, 446)
fruit_pass(q2, 582, 477, 40, 563, 456)
fruit_pass(q3, 352, 642, 50, 330, 618)
fruit_pass(q4, 727, 592, 43, 708, 572)
print("fruit pass done")

--@ chunk 345
print(wait(20*60))
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=2.2, length={30,80}, pressure={0.7,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print("fruit darkened evenly")

--@ chunk 346
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_mid3, tool=fruit_b, coverage=3.5, length={30,80}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print("fruit mid step")

--@ chunk 347
print(wait(20*60))
local function fruit_lit5(m, cx, cy, r, lx, ly)
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=1.6, length={20,50}, pressure={0.45,0.85}, load_at=grad(r*1.1,r*1.1,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
end
fruit_lit5(q1, 467, 467, 45, 447, 446)
fruit_lit5(q2, 582, 477, 40, 563, 456)
fruit_lit5(q3, 352, 642, 50, 330, 618)
fruit_lit5(q4, 727, 592, 43, 708, 572)
print("fruit lit5")

--@ chunk 348
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_mid3, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(20*60))
print("fruit even base")

--@ chunk 349
local function fruit_soft(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*0.95, cy + (cy-ly)*0.95
  local sm = (ellipse(shx, shy, r*1.15, r*1.1):soften(r*0.55)) * m
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, clip=true, edge="loose", hug=false, curve={0.3,0.2}})
  local lm = (ellipse(lx, ly, r*1.0, r*0.95):soften(r*0.5)) * m
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=1.8, length={20,50}, pressure={0.45,0.85}, clip=true, edge="loose", hug=false, curve={0.3,0.2}})
end
fruit_soft(q3, 352, 642, 50, 330, 618)
print("q3 soft masks")

--@ chunk 350
local function fruit_soft2(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*0.95, cy + (cy-ly)*0.95
  local sm = (ellipse(shx, shy, r*1.15, r*1.1):soften(r*0.55)) * m
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, clip=sm, edge="loose", hug=false, curve={0.3,0.2}})
  local lm = (ellipse(lx, ly, r*1.0, r*0.95):soften(r*0.5)) * m
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=1.8, length={20,50}, pressure={0.45,0.85}, clip=lm, edge="loose", hug=false, curve={0.3,0.2}})
end
fruit_soft2(q3, 352, 642, 50, 330, 618)
print("q3 soft masks 2")

--@ chunk 351
spill = (ellipse(467,467,70,70) + ellipse(582,477,64,62) + ellipse(352,642,74,74) + ellipse(727,592,66,64)):soften(4)
spill = spill - q1 - q2 - q3 - q4
print("spill", spill:area())
work(spill, {hand="broad", pile=tab_mid, tool=fruit_b, coverage=5, length={40,110}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.2,0.15}})
work(spill, {hand="broad", pile=tab_lt3, tool=flat26, coverage=1.5, length={60,150}, pressure={0.5,0.85}, load_at=ltt, edge="found", clip=true, curve={0.2,0.15}})
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print("spill repaired, fruit dark base")

--@ chunk 352
work(table_m, {hand="broad", pile=tab_mid, tool=bigb2, coverage=5, length={130,300}, pressure={0.85,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
work(table_m, {hand="broad", pile=tab_lt3, tool=bigb2, coverage=1.6, length={130,300}, pressure={0.5,0.85}, load_at=ltt, edge="found", clip=true, curve={0.15,0.1}})
work(table_m, {hand="broad", pile=tab_dk3, tool=bigb2, coverage=1.4, length={130,300}, pressure={0.6,0.9}, load_at=rt_t, edge="found", clip=true, curve={0.15,0.1}})
print("table flat")

--@ chunk 353
local function fruit_final2(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*1.0, cy + (cy-ly)*1.0
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_sk, tool=fruit_b, coverage=1.8, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,shx,shy), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
end
fruit_final2(q1, 467, 467, 45, 447, 446)
fruit_final2(q2, 582, 477, 40, 563, 456)
fruit_final2(q3, 352, 642, 50, 330, 618)
fruit_final2(q4, 727, 592, 43, 708, 572)
print("fruit modelled 4")

--@ chunk 354
bowl_in_l = pile{{"raw umber",2.8},{"burnt sienna",1.8},{"bone black",2.2},{"yellow ochre",0.5}}
ring_fix = ((ellipse(467,467,72,72) + ellipse(582,477,66,64)):soften(5)) - q1 - q2
print("ring", ring_fix:area())
work(ring_fix, {hand="broad", pile=bowl_in2, tool=flat26, coverage=4.5, length={50,130}, pressure={0.8,1.0}, edge="found", clip=true, curve={0.25,0.2}})
print(wait(18*60))
work(jband, {hand="broad", pile=jn, tool=bigb2, coverage=1.8, length={140,300}, pressure={0.5,0.9}, load_at=jlf, edge="found", clip=true, curve={0.12,0.1}})
work(sh3, {hand="broad", pile=sh_pile2, tool=flat26, coverage=2.0, length={110,240}, pressure={0.4,0.8}, load_at=shf, edge="found", clip=true, curve={0.15,0.12}})
work(glow_m, {hand="broad", pile=glow, tool=bigb2, coverage=1.2, length={90,220}, pressure={0.4,0.8}, load_at=gf, edge="found", clip=true, curve={0.2,0.15}})
print("rings, junction, shadows, glow")

--@ chunk 355
work(jband, {hand="broad", pile=tab_dk3, tool=bigb2, coverage=2.6, length={140,300}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.12,0.1}})
bowl_in3 = pile{{"raw umber",3.0},{"burnt sienna",1.6},{"bone black",2.6},{"green earth",0.3}}
work(bowl_in, {hand="broad", pile=bowl_in3, tool=flat26, coverage=3, length={60,150}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.25,0.2}})
print("junction and bowl interior eased")

--@ chunk 356
function cres(m, cx, cy, r, shx, shy, k)
  return (ellipse(shx, shy, r*k, r*k*0.95):soften(r*0.6)) * m
end
work(q3, {hand="broad", pile=fq_mid3, tool=fruit_b, coverage=5, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(16*60))
work(q3, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=2.2, length={20,50}, pressure={0.45,0.85}, clip=cres(q3,352,642,50,378,668,0.85), edge="found", hug=false, curve={0.3,0.2}})
work(q3, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=2.0, length={18,45}, pressure={0.45,0.85}, clip=cres(q3,352,642,50,330,618,0.8), edge="found", hug=false, curve={0.3,0.2}})
print("q3 hug test")

--@ chunk 357
work(q3, {hand="broad", pile=fq_mid3, tool=fruit_b, coverage=5, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(16*60))
work(q3, {hand="body", pile=fq_sd2, tool=fb9b, coverage=2.6, length={10,26}, pressure={0.4,0.8}, clip=cres(q3,352,642,50,380,670,0.9), edge="loose", hug=false, curve={0.3,0.2}})
work(q3, {hand="body", pile=fq_lt, tool=fb9b, coverage=2.4, length={10,26}, pressure={0.4,0.8}, clip=cres(q3,352,642,50,328,616,0.85), edge="loose", hug=false, curve={0.3,0.2}})
print("q3 loose edge test")

--@ chunk 358
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_dk2, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(20*60))
local function fruit_final3(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*1.0, cy + (cy-ly)*1.0
  work(m, {hand="broad", pile=fq_lt, tool=fruit_b, coverage=2.2, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,lx,ly), edge="found", clip=true, curve={0.3,0.2}})
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, load_at=grad(r*1.15,r*1.15,shx,shy), edge="found", clip=true, curve={0.3,0.2}})
  blend(m, {})
end
fruit_final3(q1, 467, 467, 45, 447, 446)
fruit_final3(q2, 582, 477, 40, 563, 456)
fruit_final3(q3, 352, 642, 50, 330, 618)
fruit_final3(q4, 727, 592, 43, 708, 572)
print("fruit final3")

--@ chunk 359
work(bowl_in, {hand="broad", pile=bowl_in3, tool=flat26, coverage=4, length={60,150}, pressure={0.7,1.0}, edge="found", clip=true, curve={0.25,0.2}})
work(jband, {hand="broad", pile=tab_mid, tool=bigb2, coverage=2.6, length={140,300}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.12,0.1}})
print("bowl interior and junction cleaned")

--@ chunk 360
jband2 = (mask(function(x,y) local d = math.abs(y - juncf(x)); return clamp(1 - d/19, 0, 1) end):soften(7)) - bowlm - allq - bottle_m
print("jband2", jband2:area())
jn2 = pile{{"raw umber",3.0},{"burnt sienna",2.0},{"bone black",2.0},{"yellow ochre",0.4}}
work(jband2, {hand="broad", pile=jn2, tool=bigb2, coverage=3, length={140,300}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.12,0.1}})
print("junction line")

--@ chunk 361
work(wall_m, {hand="broad", pile=wall_mid, tool=bigb2, coverage=6, length={150,320}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
work(wall_m, {hand="broad", pile=wall_lt3, tool=bigb2, coverage=1.8, length={150,320}, pressure={0.5,0.85}, load_at=ltf4, edge="found", clip=true, curve={0.15,0.1}})
work(wall_m, {hand="broad", pile=wall_dk4, tool=bigb2, coverage=1.6, length={150,320}, pressure={0.6,0.9}, load_at=rtf, edge="found", clip=true, curve={0.15,0.1}})
print("wall final")

--@ chunk 362
work(wall_m, {hand="broad", pile=wall_dk, tool=bigb2, coverage=6, length={150,320}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.15,0.1}})
print(wait(20*60))
work(wall_m, {hand="broad", pile=wall_lt2, tool=bigb2, coverage=1.8, length={150,320}, pressure={0.5,0.85}, load_at=ltf4, edge="found", clip=true, curve={0.15,0.1}})
print("wall darkened")

--@ chunk 363
work(wall_m, {hand="broad", pile=wall_mid, tool=bigb2, coverage=7, length={150,320}, pressure={0.95,1.0}, edge="found", clip=true, curve={0.1,0.08}})
print("wall even")

--@ chunk 364
wall_dk5 = pile{{"raw umber",3.0},{"burnt sienna",2.0},{"bone black",1.8},{"yellow ochre",0.5}}
work(wall_m, {hand="broad", pile=wall_dk5, tool=bigb2, coverage=7, length={150,320}, pressure={0.95,1.0}, edge="found", clip=true, curve={0.1,0.08}})
print(wait(20*60))
work(jband2, {hand="broad", pile=jn2, tool=bigb2, coverage=2.6, length={140,300}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.12,0.1}})
wall_glow_m = (ellipse(300,210,240,160):soften(70)) * wall_m
print("glow area", wall_glow_m:area())
work(wall_m, {hand="broad", pile=wall_lt2, tool=bigb2, coverage=1.4, length={150,320}, pressure={0.45,0.8}, clip=wall_glow_m, edge="soft", curve={0.15,0.1}})
print("wall darker with glow")

--@ chunk 365
work(wall_glow_m, {hand="broad", pile=wall_dk5, tool=bigb2, coverage=6, length={150,320}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.12,0.1}})
bt_sp_fix = (ellipse(197,392,16,44):soften(10)) * bottle_m
work(bottle_m, {hand="broad", pile=bt_gl2, tool=flat26, coverage=3, length={40,110}, pressure={0.6,0.95}, clip=bt_sp_fix, edge="found", curve={0.2,0.15}})
print("wall and bottle cleaned")

--@ chunk 366
work(jband2, {hand="broad", pile=jn2, tool=bigb2, coverage=2.4, length={140,300}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.12,0.1}})
work(bowl_front, {hand="broad", pile=bowl_lp3, tool=flat26, coverage=2.0, length={60,150}, pressure={0.45,0.85}, clip=rimv_m, edge="found", curve={0.2,0.15}})
bt_edge = ribbon({{206,252},{198,320},{188,400},{184,470},{192,532}}, 7):soften(2) * bottle_m
work(bottle_m, {hand="body", pile=bt_gl2, tool=fb9b, coverage=2.0, length={15,40}, pressure={0.4,0.8}, clip=bt_edge, edge="found"})
print("junction, bowl rim, bottle edge")

--@ chunk 367
fq_l3 = pile{{"yellow ochre",2.6},{"cadmium yellow",1.2},{"transparent oxide yellow",0.4},{"lead white",0.8}}
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_l3, tool=fruit_b, coverage=1.3, length={30,80}, pressure={0.5,0.9}, edge="found", clip=true, curve={0.3,0.2}})
end
print("fruit lifted")

--@ chunk 368
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_l3, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
print(wait(22*60))
local function fruit_cres(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*0.9, cy + (cy-ly)*0.9
  work(m, {hand="broad", pile=fq_sd2, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, clip=cres(m,cx,cy,r,shx,shy,0.8), edge="found", hug=false, curve={0.3,0.2}})
  blend(m, {})
end
fruit_cres(q1, 467, 467, 45, 447, 446)
fruit_cres(q2, 582, 477, 40, 563, 456)
fruit_cres(q3, 352, 642, 50, 330, 618)
fruit_cres(q4, 727, 592, 43, 708, 572)
print("fruit crescents")

--@ chunk 369
fq_gold = pile{{"yellow ochre",2.8},{"cadmium yellow",0.5},{"transparent oxide yellow",0.8},{"burnt sienna",0.6},{"lead white",0.5}}
for _,m in ipairs({q1,q2,q3,q4}) do
  work(m, {hand="broad", pile=fq_gold, tool=fruit_b, coverage=6, length={30,90}, pressure={0.9,1.0}, edge="found", clip=true, curve={0.3,0.2}})
end
bowl_in4 = pile{{"raw umber",3.0},{"burnt sienna",1.8},{"bone black",2.4},{"yellow ochre",0.4}}
work(bowl_in, {hand="broad", pile=bowl_in4, tool=flat26, coverage=4, length={60,150}, pressure={0.7,1.0}, edge="found", clip=true, curve={0.25,0.2}})
print("fruit golden base")

--@ chunk 370
print(wait(22*60))
fq_shdw = pile{{"raw umber",3.0},{"burnt sienna",1.8},{"yellow ochre",1.0},{"bone black",1.6}}
local function fruit_cres2(m, cx, cy, r, lx, ly)
  local shx, shy = cx + (cx-lx)*0.9, cy + (cy-ly)*0.9
  work(m, {hand="broad", pile=fq_shdw, tool=fruit_b, coverage=2.0, length={22,55}, pressure={0.5,0.9}, clip=cres(m,cx,cy,r,shx,shy,0.8), edge="found", hug=false, curve={0.3,0.2}})
  blend(m, {})
end
fruit_cres2(q1, 467, 467, 45, 447, 446)
fruit_cres2(q2, 582, 477, 40, 563, 456)
fruit_cres2(q3, 352, 642, 50, 330, 618)
fruit_cres2(q4, 727, 592, 43, 708, 572)
print("fruit crescents 2")

--@ chunk 371
work(wall_m, {hand="broad", pile=wall_dk5, tool=bigb2, coverage=7, length={150,320}, pressure={0.95,1.0}, edge="found", clip=true, curve={0.1,0.08}})
print(wait(20*60))
work(bowl_front, {hand="broad", pile=bowl_rim_p2, tool=flat26, coverage=2.0, length={60,150}, pressure={0.45,0.85}, clip=rimv_m, edge="found", curve={0.2,0.15}})
work(bowl_front, {hand="broad", pile=bowl_lp4, tool=bigb2, coverage=1.2, length={90,220}, pressure={0.45,0.8}, load_at=blf3, edge="found", clip=true, curve={0.2,0.15}})
print("wall clean, bowl relit")

--@ chunk 372
right_rim = rimv_m * mask(function(x,y) return clamp((x-500)/70, 0, 1) end)
left_rim = rimv_m * mask(function(x,y) return clamp((530-x)/70, 0, 1) end)
print("right", right_rim:area(), "left", left_rim:area())
work(bowl_front, {hand="broad", pile=bowl_lp4, tool=flat26, coverage=3, length={60,150}, pressure={0.6,0.95}, clip=right_rim, edge="found", curve={0.2,0.15}})
work(bowl_front, {hand="broad", pile=bowl_rim_p2, tool=flat26, coverage=2.0, length={60,150}, pressure={0.45,0.85}, clip=left_rim, edge="found", curve={0.2,0.15}})
print("rim corrected")

--@ chunk 373
print(wait(14*60))
fq_dimple = pile{{"raw umber",2.2},{"bone black",1.4},{"green earth",0.8}}
bd = brush{kind="rigger", width=3.5}
bd:load(fq_dimple, 0.9)
bd:touch(471, 425, {pressure=0.75, drag={0.8,0.4}, clip=q1})
bd:reload(fq_dimple, 0.9)
bd:touch(586, 442, {pressure=0.7, drag={0.8,0.4}, clip=q2})
bd:reload(fq_dimple, 0.9)
bd:touch(357, 595, {pressure=0.8, drag={0.9,0.45}, clip=q3})
bd:reload(fq_dimple, 0.9)
bd:touch(731, 551, {pressure=0.7, drag={0.8,0.4}, clip=q4})
print("dimples")

--@ chunk 374
wall_cool = pile{{"raw umber",3.0},{"burnt sienna",1.2},{"bone black",2.0},{"ultramarine blue",0.9}}
work(wall_m, {hand="broad", pile=wall_cool, tool=bigb2, coverage=1.3, length={150,320}, pressure={0.5,0.85}, edge="found", clip=true, curve={0.12,0.1}})
print("wall cooled")

--@ chunk 375
work(wall_m, {hand="broad", pile=wall_dk5, tool=bigb2, coverage=7, length={150,320}, pressure={0.95,1.0}, edge="found", clip=true, curve={0.1,0.08}})
print("wall restored")

--@ chunk 376
print(wait(20*60))
work(jband2, {hand="broad", pile=jn2, tool=bigb2, coverage=2.2, length={140,300}, pressure={0.6,0.95}, edge="found", clip=true, curve={0.12,0.1}})
print("junction line final")

--@ chunk 377
bt_flat = pile{{"bone black",3.4},{"green earth",2.6},{"raw umber",1.8},{"ultramarine blue",0.3}}
bt_even = pile{{"green earth",2.4},{"raw umber",1.8},{"bone black",2.2},{"lead white",0.2}}
work(bottle_m, {hand="broad", pile=bt_flat, tool=flat26, coverage=7, length={60,160}, pressure={0.95,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print(wait(18*60))
work(bottle_m, {hand="broad", pile=bt_even, tool=flat26, coverage=1.6, length={70,180}, pressure={0.5,0.9}, edge="found", clip=true, curve={0.2,0.15}})
print("bottle even")

--@ chunk 378
bt_darker = pile{{"bone black",4.2},{"green earth",2.4},{"raw umber",1.2},{"ultramarine blue",0.3}}
work(bottle_m, {hand="broad", pile=bt_darker, tool=flat26, coverage=7, length={60,160}, pressure={0.95,1.0}, edge="found", clip=true, curve={0.3,0.2}})
print("bottle darkened")

--@ chunk 379
print(wait(18*60))
bt_line = ribbon({{199,300},{193,360},{190,420},{192,470}}, 9):soften(3) * bottle_m
bt_ln = pile{{"green earth",2.2},{"raw umber",1.6},{"lead white",0.5},{"bone black",1.4}}
work(bottle_m, {hand="body", pile=bt_ln, tool=fb9b, coverage=1.8, length={16,42}, pressure={0.4,0.75}, clip=bt_line, edge="found"})
print("bottle sheen")

--@ chunk 380
function bbox(m, step)
  step = step or 4
  local x0, y0, x1, y1, n = 1e9, 1e9, -1e9, -1e9, 0
  for y = 0, H, step do
    for x = 0, W, step do
      if m:at(x, y) > 0.5 then
        if x < x0 then x0 = x end
        if x > x1 then x1 = x end
        if y < y0 then y0 = y end
        if y > y1 then y1 = y end
        n = n + 1
      end
    end
  end
  return string.format("x %d..%d  y %d..%d  cells %d", x0, x1, y0, y1, n)
end
names = {"C1","C2","C3","C4","S1","S2","S3","S4","sh3","spill","ring_fix","jband2","glow_m","wall_glow_m","bwall","z_d","z_l"}
for _, n in ipairs(names) do
  local m = _G[n]
  if m then print(n, bbox(m)) end
end
print("drying table", drying(300, 600), drying(700, 620), drying(880, 560))
print("drying wall", drying(300, 150), drying(850, 120))
print("drying bowl", drying(520, 540), "bottle", drying(230, 350))
print("drying q1", drying(465, 470), "q3", draining and 0 or drying(355, 640))

--@ chunk 381
-- Read the bowl's geometry off the canvas and lay a clean dark mass, so the
-- chalky haze and soft edges in it go. Fruit stand in front of it: subtract them.
bowl_g  = ellipse(521, 480, 176, 64) + ellipse(521, 516, 152, 86)
qA = ellipse(466, 461, 62, 62)
qB = ellipse(578, 467, 44, 59)
qC = ellipse(355, 645, 59, 76)
qD = ellipse(730, 600, 51, 61)
fg = qA:grow(5) + qB:grow(5) + qC:grow(5) + qD:grow(5)
bowl_m = bowl_g - fg
print("bowl", bowl_m:area(), "whole", bowl_g:area())
print(bbox(bowl_m))
bowl_dark = pile{{"bone black",2.8},{"raw umber",2.6},{"green earth",1.4},{"burnt sienna",1.0},{"ultramarine blue",0.3}}
bigb = brush{kind="filbert", width=40}
work(bowl_m, {hand="broad", tool=bigb, pile=bowl_dark, length={90,190}, coverage=3, angle=0.15,
             curve={0,0}, edge="found", clip=true, ruler=true})
wait(60)
work(bowl_m, {hand="broad", tool=bigb, pile=bowl_dark, length={90,190}, coverage=3, angle=1.5,
             curve={0,0}, edge="found", clip=true, ruler=true})
print(drying(500, 540), drying(400, 470))

--@ chunk 382
-- The fruit: kill the bald yellow and the pale halo. New silhouettes, grown
-- over the haloes and roughened so the edge is a drawn one.
qA2 = ellipse(466, 461, 68, 66):roughen(3, 55, 3)
qB2 = ellipse(578, 468, 47, 61):roughen(3, 50, 7)
qC2 = ellipse(355, 646, 64, 81):roughen(3, 60, 11)
qD2 = ellipse(730, 600, 54, 64):roughen(3, 50, 13)
fq_base = pile{{"bone black",2.4},{"raw umber",2.8},{"yellow ochre",2.0},{"burnt sienna",0.8},{"green earth",0.3}}
fb8 = brush("filbert", 8)
for _, m in ipairs({qA2, qB2, qC2, qD2}) do
  fb8:load(fq_base, 0.9)
  work(m, {hand="body", tool=fb8, pile=fq_base, coverage=4.5, length={18,45},
           angle=0.4, edge="found", clip=true})
end
print("done", drying(466, 461), drying(355, 646))

--@ chunk 383
print(wait(20 * 60))
print(drying(466, 461), drying(355, 646), drying(730, 600))

--@ chunk 384
function litload(cx, cy, r, lo, hi)
  local ax, ay = -0.72, -0.69
  return function(x, y)
    local d = ((x - cx) * ax + (y - cy) * ay) / r
    local t = clamp(d * 0.8 + 0.42, 0, 1)
    return lo + (hi - lo) * t
  end
end
fq_L1 = pile{{"yellow ochre",2.6},{"cadmium yellow",0.7},{"transparent oxide yellow",0.5},{"burnt sienna",1.0},{"lead white",0.3}}
fq_L2 = pile{{"yellow ochre",2.4},{"cadmium yellow",1.2},{"transparent oxide yellow",0.6},{"lead white",0.8},{"burnt sienna",0.4}}
fb10b = brush("filbert", 10)
for _, t in ipairs({{qA2, 466, 461, 68, fq_L1, 0.28, 0.9},
                    {qB2, 578, 468, 60, fq_L1, 0.24, 0.85},
                    {qC2, 355, 646, 80, fq_L2, 0.3, 1.0},
                    {qD2, 730, 600, 63, fq_L2, 0.26, 0.95}}) do
  local m = t[1]
  fb10b:load(t[5], 0.9)
  work(m, {hand="broad", tool=fb10b, pile=t[5], coverage=2.6, length={20,55},
           load_at=litload(t[2], t[3], t[4], t[6], t[7]), edge="found", clip=true, angle=0.3})
  blend(m, {angle=0.6})
end
print("laid", drying(466, 461))

--@ chunk 385
print(wait(20 * 60))
fq_w1 = pile{{"yellow ochre",2.8},{"transparent oxide yellow",0.9},{"cadmium yellow",0.5},{"burnt sienna",0.6},{"lead white",0.4}}
fb12c = brush("filbert", 12)
for _, t in ipairs({{qA2, 466, 461, 68, 0.3, 0.95},
                    {qB2, 578, 468, 60, 0.26, 0.9},
                    {qC2, 355, 646, 80, 0.32, 1.0},
                    {qD2, 730, 600, 63, 0.28, 0.95}}) do
  local m = t[1]
  fb12c:load(fq_w1, 0.9)
  work(m, {hand="broad", tool=fb12c, pile=fq_w1, coverage=2.2, length={22,60},
           load_at=litload(t[2], t[3], t[4], t[6] - 0.1, t[5]), edge="found", clip=true, angle=-0.5})
  blend(m, {angle=1.1})
end
print("warm", drying(466, 461))

--@ chunk 386
-- That warm pass went over the top and flattened the fruit into orange discs.
-- Back to the dark base, then the light pile that worked, warmed a little and
-- kept lean so the base still shows through.
print(wait(20 * 60))
fq_base2 = pile{{"bone black",2.4},{"raw umber",3.0},{"burnt sienna",1.6},{"yellow ochre",1.2}}
fb9c = brush("filbert", 9)
for _, m in ipairs({qA2, qB2, qC2, qD2}) do
  fb9c:load(fq_base2, 0.9)
  work(m, {hand="body", tool=fb9c, pile=fq_base2, coverage=4.5, length={18,45},
           angle=0.4, edge="found", clip=true})
end
print(wait(20 * 60))
fq_L3 = pile{{"yellow ochre",2.7},{"transparent oxide yellow",0.45},{"cadmium yellow",0.4},{"burnt sienna",1.2},{"lead white",0.35}}
for _, t in ipairs({{qA2, 466, 461, 68, 0.3, 0.95},
                    {qB2, 578, 468, 60, 0.26, 0.9},
                    {qC2, 355, 646, 80, 0.32, 1.0},
                    {qD2, 730, 600, 63, 0.28, 0.95}}) do
  local m = t[1]
  fb12c:load(fq_L3, 0.9)
  work(m, {hand="broad", tool=fb12c, pile=fq_L3, coverage=2.0, length={22,60},
           load_at=litload(t[2], t[3], t[4], t[6], t[5]), edge="found", clip=true, angle=-0.5})
  blend(m, {angle=1.1})
end
print("relaid", drying(466, 461))

--@ chunk 387
-- Paint the fruit as zones of value with broken edges instead of blending a
-- gradient: shadow crescent, mid, lit cap, each its own pile.
fq_sh = pile{{"raw umber",3.0},{"burnt sienna",1.8},{"bone black",1.8},{"yellow ochre",0.5}}
fq_md = pile{{"raw umber",2.2},{"burnt sienna",2.2},{"yellow ochre",1.6},{"lead white",0.4}}
fq_lt = pile{{"yellow ochre",2.6},{"cadmium yellow",1.0},{"transparent oxide yellow",0.5},{"lead white",0.8}}
piles = {fq_sh, fq_md, fq_lt}
fz = {}
for _, t in ipairs({{"qA2", 466, 461, 68}, {"qB2", 578, 468, 60}, {"qC2", 355, 646, 80}, {"qD2", 730, 600, 63}}) do
  local m, cx, cy, r = _G[t[1]], t[2], t[3], t[4]
  local mid = (ellipse(cx - 0.12 * r, cy - 0.10 * r, 0.80 * r, 0.78 * r) * m):roughen(2.5, 34, t[4])
  local lit = (ellipse(cx - 0.30 * r, cy - 0.34 * r, 0.52 * r, 0.50 * r) * m):roughen(2, 28, t[3])
  fz[#fz + 1] = {m - mid, mid, lit}
end
fb11 = brush("filbert", 11)
for i = 1, 3 do
  fb11:load(piles[i], 0.9)
  for j = 1, 4 do
    work(fz[j][i], {hand="body", tool=fb11, pile=piles[i], coverage=2.4 - 0.25 * (i - 1),
                     length={16,48}, angle=0.9 - 0.35 * i, edge="found", clip=true})
  end
end
print("zones", drying(355, 646))

--@ chunk 388
-- The zones read as a coin on a disc. One terminator curve instead: paint the
-- whole fruit mid, then the lit side of a single curve, then a shadow crescent.
print(wait(20 * 60))
fq_md2 = pile{{"raw umber",2.2},{"burnt sienna",2.2},{"yellow ochre",1.6},{"lead white",0.4}}
fb13 = brush("filbert", 13)
for _, m in ipairs({qA2, qB2, qC2, qD2}) do
  fb13:load(fq_md2, 0.9)
  work(m, {hand="body", tool=fb13, pile=fq_md2, coverage=2.8, length={20,55},
           angle=0.5, edge="found", clip=true})
end
print("mid", drying(355, 646))

--@ chunk 389
print(wait(20 * 60))
fq_lt2 = pile{{"yellow ochre",2.8},{"cadmium yellow",0.6},{"transparent oxide yellow",0.6},{"burnt sienna",1.0},{"lead white",0.4}}
term = {}
for _, t in ipairs({{"qA2", 466, 461, 68}, {"qB2", 578, 468, 60}, {"qC2", 355, 646, 80}, {"qD2", 730, 600, 63}}) do
  local m, cx, cy, r = _G[t[1]], t[2], t[3], t[4]
  local z = (ellipse(cx - 0.20 * r, cy - 0.18 * r, 0.94 * r, 0.90 * r) * m):roughen(3, 40, t[2])
  term[t[1]] = z
end
for _, n in ipairs({"qA2", "qB2", "qC2", "qD2"}) do
  fb13:load(fq_lt2, 0.9)
  work(term[n], {hand="broad", tool=fb13, pile=fq_lt2, coverage=1.5, length={26,62},
                 angle=0.8, edge="found", clip=true})
end
print("lit", drying(355, 646))

--@ chunk 390
print(wait(12 * 60))
fq_sh2 = pile{{"raw umber",3.2},{"burnt sienna",2.0},{"bone black",2.0},{"yellow ochre",0.4}}
fb10d = brush("filbert", 10)
for _, n in ipairs({"qA2", "qB2", "qC2", "qD2"}) do
  fb10d:load(fq_sh2, 0.9)
  work(_G[n] - term[n], {hand="body", tool=fb10d, pile=fq_sh2, coverage=1.9, length={18,48},
                          angle=1.5, edge="found", clip=true})
end
print("shadow", drying(355, 646))

--@ chunk 391
print(wait(8 * 60))
for _, m in ipairs({qA2, qB2, qC2, qD2}) do
  blend(m, {angle=0.4})
end
print("blended", drying(355, 646))

--@ chunk 392
-- The old bowl's pale edge is still showing round the new black one. Grow the
-- bowl over it and repaint the ring.
fg2 = qA2:grow(3) + qB2:grow(3) + qC2:grow(3) + qD2:grow(3)
bowl_g2 = bowl_g:grow(11)
ring = bowl_g2 - bowl_g
print("ring", ring:area(), bbox(ring))
work(ring - fg2, {hand="broad", tool=bigb, pile=bowl_dark, length={60,140}, coverage=3.5,
                  angle=1.2, curve={0,0}, edge="found", clip=true, ruler=true})
lipy = function(x) return 494 + 26 * math.sqrt(math.max(0, 1 - ((x - 521) / 176) ^ 2)) end
lipband = (below(function(x) return lipy(x) + 10 end) - below(lipy)) * bowl_g2
print("lip", lipband:area())
print("inside", ((bowl_g2 - below(lipy)) - fg2):area())

--@ chunk 393
lipband = (below(lipy) - below(function(x) return lipy(x) + 10 end)) * bowl_g2
print("lip", lipband:area(), bbox(lipband))
inside_m = (bowl_g2 - below(lipy)) - fg2
wall_m = (below(lipy) * bowl_g2) - fg2
print("wall", wall_m:area(), "inside", inside_m:area())

--@ chunk 394
print(wait(20 * 60))
in_p = pile{{"raw umber",2.2},{"burnt sienna",2.2},{"bone black",1.2},{"yellow ochre",0.6}}
belly_p = pile{{"raw umber",2.2},{"burnt sienna",2.4},{"yellow ochre",1.0},{"lead white",0.4},{"bone black",0.8}}
fb16 = brush("filbert", 16)
work(inside_m, {hand="broad", tool=fb16, pile=in_p, coverage=2.4, length={40,110},
                angle=0.15, curve={0,0}, edge="found", clip=true})
belly_z = (ellipse(452, 545, 98, 58) * wall_m):roughen(4, 45, 9) + (ellipse(430, 585, 60, 30) * wall_m):roughen(3, 30, 4)
fb16:load(belly_p, 0.9)
work(belly_z, {hand="broad", tool=fb16, pile=belly_p, coverage=2.0, length={35,90},
               angle=0.9, curve={0,0}, edge="found", clip=true})
print("bowl form", drying(500, 560))

--@ chunk 395
-- That was two big flakey passes: the bowl now reads as a crust. Flatten it back
-- to an even deep dark, crossing angles, the way the field recipe does it.
bm = bowl_g2 - fg2
work(bm, {hand="broad", tool=bigb, pile=bowl_dark, length={90,190}, coverage=3, angle=0.15,
          curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work(bm, {hand="broad", tool=bigb, pile=bowl_dark, length={90,190}, coverage=3, angle=1.5,
          curve={0,0}, edge="found", clip=true, ruler=true})
print("flattened", drying(500, 560))

--@ chunk 396
-- The bowl is an even dark now. What it needs is the one note that makes it a
-- bowl: the near lip, drawn as strokes, warm light on the left dying to the right.
print(wait(20 * 60))
lip_lt = pile{{"raw umber",2.0},{"burnt sienna",1.6},{"yellow ochre",1.2},{"lead white",1.0},{"bone black",0.5}}
lip_dk = pile{{"raw umber",2.6},{"burnt sienna",1.8},{"yellow ochre",0.5},{"lead white",0.25},{"bone black",1.2}}
bl5 = brush("filbert", 5)
bl5:load(lip_lt, 0.85)
bl5:stroke({{352, 502}, {372, 506}, {398, 511}, {430, 515}}, {pressure={0.75, 0.5}, ramps={0.1, 0.4}, shake=0.6})
bl5:reload(lip_lt, 0.85)
bl5:stroke({{436, 516}, {470, 519}, {506, 521}, {534, 521}}, {pressure={0.7, 0.55}, ramps={0.1, 0.35}, shake=0.5})
bl5:reload(lip_dk, 0.8)
bl5:stroke({{542, 520}, {578, 519}, {614, 515}, {650, 510}}, {pressure={0.6, 0.45}, ramps={0.1, 0.4}, shake=0.6})
bl5:reload(lip_dk, 0.75)
bl5:stroke({{658, 508}, {678, 504}, {694, 499}}, {pressure={0.5, 0.3}, ramps={0.1, 0.5}, shake=0.6})
print("lip", drying(450, 518))

--@ chunk 397
-- The lip went on as a flat grey tape. Lay it again, warmer and brighter, in
-- broken strokes that die away to the right; the grey underneath shows as the
-- duller note.
print(wait(18 * 60))
lip_w = pile{{"yellow ochre",2.2},{"transparent oxide yellow",0.8},{"lead white",1.4},{"burnt sienna",0.5}}
bl4 = brush("filbert", 4)
bl4:load(lip_w, 0.8)
bl4:stroke({{349, 501}, {361, 504}}, {pressure={0.5, 0.25}, ramps={0.1, 0.5}})
bl4:reload(lip_w, 0.9)
bl4:stroke({{366, 505}, {382, 508}, {399, 511}}, {pressure={0.7, 0.55}, ramps={0.08, 0.35}, shake=0.7})
bl4:reload(lip_w, 0.9)
bl4:stroke({{404, 512}, {424, 515}, {444, 517}}, {pressure={0.8, 0.6}, ramps={0.08, 0.3}, shake=0.5})
bl4:reload(lip_w, 0.85)
bl4:stroke({{452, 518}, {478, 520}, {500, 521}}, {pressure={0.7, 0.45}, ramps={0.1, 0.4}, shake=0.6})
bl4:reload(lip_w, 0.7)
bl4:stroke({{508, 521}, {534, 521}}, {pressure={0.5, 0.3}, ramps={0.1, 0.45}, shake=0.6})
bl4:reload(lip_w, 0.6)
bl4:stroke({{544, 520}, {568, 519}, {590, 517}}, {pressure={0.45, 0.25}, ramps={0.12, 0.5}, shake=0.7})
bl4:reload(lip_w, 0.5)
bl4:stroke({{602, 515}, {628, 512}, {652, 508}}, {pressure={0.35, 0.15}, ramps={0.15, 0.5}, shake=0.8})
print("lip2", drying(450, 518))

--@ chunk 398
-- The bright lip came out as a row of equal dashes. Run one continuous band
-- along the curve underneath them so the rim reads as a single note.
lip_mid = pile{{"raw umber",2.4},{"burnt sienna",1.8},{"yellow ochre",0.8},{"lead white",0.4},{"bone black",0.9}}
bl6 = brush("filbert", 6)
bl6:load(lip_mid, 0.8)
bl6:stroke({{344, 500}, {372, 506}, {410, 512}}, {pressure={0.5, 0.32}, ramps={0.12, 0.45}, shake=0.7})
bl6:reload(lip_mid, 0.85)
bl6:stroke({{400, 511}, {432, 515}, {470, 519}}, {pressure={0.55, 0.4}, ramps={0.1, 0.4}, shake=0.6})
bl6:reload(lip_mid, 0.85)
bl6:stroke({{460, 519}, {498, 521}, {540, 521}}, {pressure={0.5, 0.45}, ramps={0.1, 0.35}, shake=0.5})
bl6:reload(lip_mid, 0.8)
bl6:stroke({{530, 521}, {570, 519}, {610, 516}}, {pressure={0.45, 0.3}, ramps={0.12, 0.45}, shake=0.7})
bl6:reload(lip_mid, 0.7)
bl6:stroke({{600, 516}, {640, 511}, {680, 505}}, {pressure={0.35, 0.18}, ramps={0.15, 0.5}, shake=0.8})
bl6:reload(lip_mid, 0.6)
bl6:stroke({{672, 506}, {688, 501}, {700, 497}}, {pressure={0.25, 0.08}, ramps={0.15, 0.5}, shake=0.8})
print("lip3", drying(450, 518))

--@ chunk 399
-- The table is full of pink bruises and dry-brush scribbles. Flatten it to one
-- warm brown field, objects excluded so no pale ring survives.
bottle_g = ellipse(232, 400, 80, 155) + ellipse(233, 305, 52, 72) + ellipse(233, 258, 40, 46)
obj = bottle_g + bowl_g2 + qA2 + qB2 + qC2 + qD2
yj = function(x) return 478 + 0.046 * x end
table_m = below(yj)
table_free = table_m - obj
print("table", table_m:area(), "free", table_free:area())
tab_p = pile{{"raw umber",2.6},{"burnt sienna",2.6},{"yellow ochre",0.8},{"bone black",0.7}}
bigf = brush{kind="filbert", width=44}
work(table_free, {hand="broad", tool=bigf, pile=tab_p, length={100,220}, coverage=3, angle=0.08,
                  curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work(table_free, {hand="broad", tool=bigf, pile=tab_p, length={100,220}, coverage=3, angle=1.48,
                  curve={0,0}, edge="found", clip=true, ruler=true})
print("table flattened", drying(200, 700))

--@ chunk 400
-- Give the table its light: a pool at the front left, the right falling away.
print(wait(20 * 60))
tab_l = pile{{"raw umber",2.2},{"burnt sienna",2.6},{"yellow ochre",1.2},{"lead white",0.5}}
tab_d = pile{{"raw umber",3.0},{"burnt sienna",1.8},{"bone black",1.6},{"yellow ochre",0.3}}
halo = obj:grow(12)
pool_l = (ellipse(150, 700, 380, 200) * (table_m - halo))
work(pool_l, {hand="broad", tool=bigf, pile=tab_l, length={90,200}, coverage=2.4, angle=0.1,
              curve={0,0}, edge="soft", clip=true})
wait(120)
pool_d = (rect(610, 460, 390, 290):roughen(22, 130, 7)) * (table_m - halo)
work(pool_d, {hand="broad", tool=bigf, pile=tab_d, length={90,200}, coverage=2.4, angle=0.1,
              curve={0,0}, edge="soft", clip=true})
print("light", drying(150, 700), drying(850, 650))

--@ chunk 401
-- edge="soft" with 200-unit strokes ran the light right across the table and
-- left black flakes. Flatten again, then build the falloff in narrow bands with
-- found edges, each a little lighter than the last.
work(table_free, {hand="broad", tool=bigf, pile=tab_p, length={100,220}, coverage=3, angle=0.08,
                  curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work(table_free, {hand="broad", tool=bigf, pile=tab_p, length={100,220}, coverage=3, angle=1.48,
                  curve={0,0}, edge="found", clip=true, ruler=true})
pool_m = table_m - halo
P = {
  pile{{"raw umber",2.5},{"burnt sienna",2.6},{"yellow ochre",0.9},{"lead white",0.2},{"bone black",0.5}},
  pile{{"raw umber",2.3},{"burnt sienna",2.6},{"yellow ochre",1.1},{"lead white",0.35},{"bone black",0.4}},
  pile{{"raw umber",2.1},{"burnt sienna",2.5},{"yellow ochre",1.3},{"lead white",0.5},{"bone black",0.3}},
  pile{{"raw umber",1.9},{"burnt sienna",2.4},{"yellow ochre",1.5},{"lead white",0.7},{"bone black",0.2}},
}
B = {}
for i, t in ipairs({{300, 670, 430, 240, 9}, {230, 680, 340, 190, 4}, {150, 690, 250, 140, 8}, {80, 700, 160, 95, 2}}) do
  B[i] = ((ellipse(t[1], t[2], t[3], t[4]):roughen(9, 95, t[5])) * pool_m)
end
for i = 1, 4 do
  wait(130)
  work(B[i], {hand="broad", tool=bigf, pile=P[i], length={90,190}, coverage=2.4,
              angle=0.1, curve={0,0}, edge="found", clip=true})
end
print("bands", drying(150, 700))

--@ chunk 402
function oval(cx, cy, rx, ry, ang)
  local c, s = math.cos(ang), math.sin(ang)
  return mask(function(x, y)
    local u = (x - cx) / rx
    local v = (y - cy) / ry
    local a = u * c + v * s
    local b = -u * s + v * c
    if a * a + b * b <= 1 then return 1 else return 0 end
  end)
end
sh_p = pile{{"raw umber",3.0},{"burnt sienna",1.6},{"bone black",2.2},{"yellow ochre",0.2}}
sh_area = table_m - obj:grow(10)
sh_b = oval(420, 578, 165, 42, 0.20) * sh_area
sh_bowl = oval(790, 596, 175, 48, 0.10) * sh_area
sh_c = oval(408, 700, 105, 32, 0.22) * sh_area
sh_d = oval(800, 646, 120, 32, 0.20) * sh_area
shadows = sh_b + sh_bowl + sh_c + sh_d
rings = ((bowl_g2:grow(11) - bowl_g2) + (bottle_g:grow(10) - bottle_g)
         + (qC2:grow(10) - qC2) + (qD2:grow(10) - qD2)) * table_m
print("shadows", shadows:area(), "rings", rings:area())
blsh = brush("filbert", 24)
blsh:load(sh_p, 0.9)
work(shadows, {hand="broad", tool=blsh, pile=sh_p, length={40,120}, coverage=2.4,
               angle=0.25, curve={0,0}, edge="found", clip=true})
blsh:load(sh_p, 0.9)
work(rings, {hand="broad", tool=blsh, pile=sh_p, length={20,50}, coverage=2.2,
             angle=1.2, curve={0,0}, edge="found", clip=true})
print("shadows laid", drying(420, 578))

--@ chunk 403
-- The shadows came out as light smudges: that pile renders far lighter than its
-- arithmetic. Flatten the shadow shapes again, much darker and solid.
sh_p2 = pile{{"bone black",4.0},{"raw umber",1.8},{"green earth",1.2},{"burnt sienna",0.6},{"ultramarine blue",0.25}}
work(shadows, {hand="broad", tool=blsh, pile=sh_p2, length={40,120}, coverage=3.6,
               angle=0.25, curve={0,0}, edge="found", clip=true})
wait(130)
work(shadows, {hand="broad", tool=blsh, pile=sh_p2, length={40,120}, coverage=2.6,
               angle=1.25, curve={0,0}, edge="found", clip=true})
blsh2 = brush("filbert", 12)
work(rings, {hand="broad", tool=blsh2, pile=sh_p2, length={14,40}, coverage=3.0,
             angle=1.2, curve={0,0}, edge="found", clip=true})
print("dark shadows", drying(420, 578), drying(790, 596))

--@ chunk 404
keep = rect(560, 430, 300, 320)          -- near the bowl and the bottle
n1 = lose(sh_b + sh_bowl, {pile=sh_p2, where=(sh_b + sh_bowl) - keep,
                           reach={22, 16}, load=0.3, pressure={0.42, 0.03}, every=1.3})
n2 = lose(sh_c + sh_d, {pile=sh_p2, where=(sh_c + sh_d) - keep,
                        reach={20, 14}, load=0.3, pressure={0.4, 0.03}, every=1.3})
n3 = lose(rings, {pile=sh_p2, where=rings:map(function(v) return v * 0.55 end),
                  reach={12, 7}, load=0.28, pressure={0.35, 0.03}, every=1.6})
print("lost", n1, n2, n3, drying(790, 596))

--@ chunk 405
-- lose() put black spikes round the shadows. Repaint the shadow shapes as
-- wedges rather than ellipses, in a dark warm brown, not black.
sh_p3 = pile{{"raw umber",3.2},{"burnt sienna",2.0},{"bone black",1.6},{"yellow ochre",0.4}}
wedge = poly({{300,538},{392,548},{524,568},{648,594},{700,616},{672,634},{540,606},{400,572},{300,562}}, true)
      + poly({{686,556},{790,568},{896,592},{976,616},{958,644},{840,622},{716,598},{676,582}}, true)
      + poly({{396,684},{468,698},{536,716},{566,736},{516,734},{440,714},{392,700}}, true)
      + poly({{762,614},{846,626},{928,648},{946,668},{874,662},{792,644},{758,630}}, true)
sh2 = wedge:grow(20) * (table_m - obj:grow(8))
print("sh2", sh2:area())
work(sh2, {hand="broad", tool=blsh, pile=sh_p3, length={40,120}, coverage=3.4,
           angle=0.25, curve={0,0}, edge="found", clip=true})
wait(130)
work(sh2, {hand="broad", tool=blsh, pile=sh_p3, length={40,120}, coverage=2.4,
           angle=1.25, curve={0,0}, edge="found", clip=true})
sh_far = sh2 * (rect(560, 430, 440, 320) + rect(460, 650, 200, 100))
bl30 = brush("filbert", 30)
bl30:load(tab_p, 0.8)
work(sh_far, {hand="broad", tool=bl30, pile=tab_p, length={40,110}, coverage=1.15,
              angle=0.3, curve={0,0}, edge="found", clip=true})
print("wedges", drying(700, 600))

--@ chunk 406
-- Everything I have put on the table since the flatten is a mess: black grass
-- from lose(), a pale halo of old fruit paint, pale shadow smears. Start the
-- table again from the field recipe and build it once, carefully.
work(table_free, {hand="broad", tool=bigf, pile=tab_p, length={100,220}, coverage=3, angle=0.08,
                  curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work(table_free, {hand="broad", tool=bigf, pile=tab_p, length={100,220}, coverage=3, angle=1.48,
                  curve={0,0}, edge="found", clip=true, ruler=true})
wait(130)
pool_m = table_m - obj:grow(11)
pool_l2 = (ellipse(230, 690, 420, 235):roughen(26, 150, 5)) * pool_m
work(pool_l2, {hand="broad", tool=bl30, pile=tab_l, length={60,150}, coverage=2.4,
               angle=0.1, curve={0,0}, edge="found", clip=true})
wait(130)
pool_d2 = (rect(640, 460, 360, 290):roughen(28, 160, 9)) * pool_m
work(pool_d2, {hand="broad", tool=bl30, pile=tab_d, length={60,150}, coverage=2.4,
               angle=0.1, curve={0,0}, edge="found", clip=true})
print("table rebuilt", drying(200, 700), drying(900, 650))

--@ chunk 407
print(wait(18 * 60))
sh_p4 = pile{{"bone black",2.4},{"raw umber",2.6},{"burnt sienna",1.6},{"green earth",0.6},{"yellow ochre",0.2}}
sh3 = wedge * (table_m - obj:grow(6))
work(sh3, {hand="broad", tool=blsh, pile=sh_p4, length={40,120}, coverage=3.4,
           angle=0.25, curve={0,0}, edge="found", clip=true})
wait(130)
work(sh3, {hand="broad", tool=blsh, pile=sh_p4, length={40,120}, coverage=2.2,
           angle=1.25, curve={0,0}, edge="found", clip=true})
rings3 = ((bowl_g2:grow(8) - bowl_g2) + (bottle_g:grow(9) - bottle_g)
          + (qC2:grow(8) - qC2) + (qD2:grow(8) - qD2)) * table_m
work(rings3, {hand="broad", tool=blsh2, pile=sh_p4, length={12,34}, coverage=2.6,
              angle=1.2, curve={0,0}, edge="found", clip=true})
wait(130)
bl30:load(tab_d, 0.75)
work(sh3 * rect(560, 430, 440, 320), {hand="broad", tool=bl30, pile=tab_d, length={40,110},
           coverage=1.2, angle=0.3, curve={0,0}, edge="found", clip=true})
bl30:load(tab_l, 0.7)
work(sh3 * rect(430, 630, 230, 120), {hand="broad", tool=bl30, pile=tab_l, length={30,80},
           coverage=1.1, angle=0.3, curve={0,0}, edge="found", clip=true})
print("shadows", drying(700, 600), drying(430, 700))

--@ chunk 408
sh_p5 = pile{{"bone black",3.0},{"raw umber",2.4},{"burnt sienna",1.6},{"green earth",0.7},{"yellow ochre",0.3}}
sh_p6 = pile{{"bone black",1.8},{"raw umber",2.8},{"burnt sienna",2.0},{"yellow ochre",0.4}}
sps = {sh_p4, sh_p5, sh_p6}
sw = {rect(240, 700, 120, 46), rect(400, 700, 120, 46), rect(560, 700, 120, 46)}
for i = 1, 3 do
  work(sw[i], {hand="broad", tool=blsh, pile=sps[i], length={40,110},
               coverage=3.4, angle=0.25, curve={0,0}, edge="found", clip=true})
end
print("swatches")

--@ chunk 409
-- The swatches say the table is simply too light: it is the second-brightest
-- thing in a low-key picture. Take the whole table down a step and rebuild the
-- light on it.
tab_d3 = pile{{"raw umber",3.0},{"burnt sienna",1.8},{"bone black",2.2},{"yellow ochre",0.2},{"green earth",0.3}}
tab_l3 = pile{{"raw umber",2.6},{"burnt sienna",2.2},{"yellow ochre",0.9},{"lead white",0.2},{"bone black",1.1}}
tab_f3 = pile{{"raw umber",3.2},{"burnt sienna",1.6},{"bone black",2.6},{"green earth",0.5}}
work(table_free, {hand="broad", tool=bigf, pile=tab_d3, length={100,220}, coverage=3, angle=0.08,
                  curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work(table_free, {hand="broad", tool=bigf, pile=tab_d3, length={100,220}, coverage=3, angle=1.48,
                  curve={0,0}, edge="found", clip=true, ruler=true})
wait(130)
work(pool_l2, {hand="broad", tool=bl30, pile=tab_l3, length={60,150}, coverage=2.4,
               angle=0.1, curve={0,0}, edge="found", clip=true})
wait(130)
work(pool_d2, {hand="broad", tool=bl30, pile=tab_f3, length={60,150}, coverage=2.4,
               angle=0.1, curve={0,0}, edge="found", clip=true})
print("table down", drying(200, 700), drying(900, 650))

--@ chunk 410
print(wait(18 * 60))
work(sh3, {hand="broad", tool=blsh, pile=sh_p5, length={40,120}, coverage=3.4,
           angle=0.25, curve={0,0}, edge="found", clip=true})
wait(130)
work(sh3, {hand="broad", tool=blsh, pile=sh_p5, length={40,120}, coverage=2.2,
           angle=1.25, curve={0,0}, edge="found", clip=true})
work(rings3, {hand="broad", tool=blsh2, pile=sh_p5, length={12,34}, coverage=2.6,
              angle=1.2, curve={0,0}, edge="found", clip=true})
wait(130)
bl30:load(tab_f3, 0.75)
work(sh3 * rect(560, 430, 440, 320), {hand="broad", tool=bl30, pile=tab_f3, length={40,110},
           coverage=1.2, angle=0.3, curve={0,0}, edge="found", clip=true})
bl30:load(tab_l3, 0.7)
work(sh3 * rect(430, 630, 230, 120), {hand="broad", tool=bl30, pile=tab_l3, length={30,80},
           coverage=1.1, angle=0.3, curve={0,0}, edge="found", clip=true})
print("shadows down", drying(700, 600))

--@ chunk 411
-- The bottle is a flat black shape with one straight stripe. Turn its left
-- side, bounce warm light off the table into its right, and break the glint.
bt_turn = pile{{"green earth",2.6},{"raw umber",1.8},{"bone black",1.0},{"lead white",0.3},{"ultramarine blue",0.2}}
bt_bnc = pile{{"raw umber",2.4},{"burnt sienna",2.0},{"yellow ochre",0.5},{"bone black",1.0}}
cres_L = bottle_g - ellipse(202, 400, 78, 152)
cres_R = (bottle_g - ellipse(262, 400, 78, 152)) * rect(0, 380, 1000, 220)
print("cresL", cres_L:area(), "cresR", cres_R:area())
fb14b = brush("filbert", 14)
fb14b:load(bt_turn, 0.85)
work(cres_L, {hand="body", tool=fb14b, pile=bt_turn, coverage=1.7, length={22,55},
              angle=1.52, edge="found", clip=true})
fb14b:load(bt_bnc, 0.8)
work(cres_R, {hand="body", tool=fb14b, pile=bt_bnc, coverage=1.5, length={20,50},
              angle=1.52, edge="found", clip=true})
print(wait(150))
bt_glint = pile{{"lead white",1.2},{"green earth",1.2},{"raw umber",0.8},{"ultramarine blue",0.25}}
fb5b = brush("filbert", 5)
fb5b:load(bt_glint, 0.7)
fb5b:stroke({{183, 342}, {180, 362}}, {pressure={0.5, 0.3}, ramps={0.1, 0.5}, shake=0.6})
fb5b:reload(bt_glint, 0.8)
fb5b:stroke({{179, 372}, {176, 420}, {177, 438}}, {pressure={0.75, 0.5}, ramps={0.1, 0.35}, shake=0.5})
fb5b:reload(bt_glint, 0.75)
fb5b:stroke({{173, 452}, {172, 486}}, {pressure={0.6, 0.25}, ramps={0.12, 0.5}, shake=0.6})
fb5b:reload(bt_turn, 0.7)
fb5b:stroke({{205, 246}, {208, 268}, {209, 292}}, {pressure={0.55, 0.3}, ramps={0.12, 0.45}, shake=0.6})
fb5b:reload(bt_glint, 0.6)
fb5b:stroke({{200, 220}, {234, 214}, {266, 219}}, {pressure={0.45, 0.35}, ramps={0.12, 0.4}, shake=0.5})
print("bottle", drying(180, 400))

--@ chunk 412
-- I mismeasured the bottle: it is x 182-282, not 152-312, so the two
-- crescents I laid came out on the wrong sides and outside the silhouette.
-- Redraw its geometry, flatten the wall, the table patch and the bottle itself,
-- and start its modelling again.
bottle_g2 = ellipse(232, 410, 50, 120) + ellipse(233, 310, 40, 55) + ellipse(233, 262, 30, 50)
print(bbox(bottle_g2))
obj2 = bottle_g2 + bowl_g2 + qA2 + qB2 + qC2 + qD2
wall_w = pile{{"raw umber",3.0},{"burnt sienna",2.4},{"yellow ochre",0.5},{"bone black",1.4}}
wall_m2 = above(yj) - obj2
tbl_fix = (table_m * rect(110, 400, 250, 210)) - obj2
bt_flat2 = pile{{"bone black",3.2},{"green earth",2.2},{"raw umber",1.4},{"ultramarine blue",0.25}}
print("wall", wall_m2:area(), "patch", tbl_fix:area(), bbox(tbl_fix))
work(wall_m2, {hand="broad", tool=bigf, pile=wall_w, length={110,230}, coverage=3, angle=0.06,
               curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work(wall_m2, {hand="broad", tool=bigf, pile=wall_w, length={110,230}, coverage=3, angle=1.5,
               curve={0,0}, edge="found", clip=true, ruler=true})
wait(130)
work(tbl_fix, {hand="broad", tool=bigf, pile=tab_d3, length={90,190}, coverage=3, angle=0.08,
               curve={0,0}, edge="found", clip=true, ruler=true})
wait(130)
work(tbl_fix, {hand="broad", tool=bigf, pile=tab_d3, length={90,190}, coverage=3, angle=1.48,
               curve={0,0}, edge="found", clip=true, ruler=true})
wait(130)
work(bottle_g2, {hand="broad", tool=brush("filbert", 20), pile=bt_flat2, length={50,120},
                 coverage=3.6, angle=0.1, curve={0,0}, edge="found", clip=true})
print("cleaned", drying(230, 400))

--@ chunk 413
-- The new wall came out light and mauve, the table patch shows as a rectangle,
-- and the bottle is a blob. Warmer wall, pool back over the patch, bottle redrawn
-- as a real profile.
wall_w2 = pile{{"burnt sienna",3.0},{"raw umber",2.2},{"yellow ochre",0.8},{"bone black",1.6},{"lead white",0.1}}
bottle_g3 = poly({{206,216},{258,216},{258,250},{262,288},{272,320},{281,352},{283,400},
                  {281,455},{272,505},{263,527},{200,527},{191,505},{183,455},{182,400},
                  {185,352},{193,320},{203,288},{207,250}}, true)
obj3 = bottle_g3 + bowl_g2 + qA2 + qB2 + qC2 + qD2
wall_m3 = above(yj) - obj3
print("bottle", bbox(bottle_g3), bottle_g3:area())
work(wall_m3, {hand="broad", tool=bigf, pile=wall_w2, length={110,230}, coverage=3, angle=0.06,
               curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work(wall_m3, {hand="broad", tool=bigf, pile=wall_w2, length={110,230}, coverage=3, angle=1.5,
               curve={0,0}, edge="found", clip=true, ruler=true})
wait(130)
work(pool_l2, {hand="broad", tool=bl30, pile=tab_l3, length={60,150}, coverage=2.2,
               angle=0.1, curve={0,0}, edge="found", clip=true})
wait(130)
work(bottle_g3, {hand="broad", tool=brush("filbert", 22), pile=bt_flat2, length={50,120},
                 coverage=3.6, angle=1.5, curve={0,0}, edge="found", clip=true})
print("fixed", drying(230, 400))

--@ chunk 414
print(wait(16 * 60))
cres_L = bottle_g3 - ellipse(252, 370, 54, 165)
cres_R = (bottle_g3 - ellipse(212, 380, 54, 168)) * rect(0, 395, 1000, 220)
print("L", cres_L:area(), "R", cres_R:area())
fb12b = brush("filbert", 12)
fb12b:load(bt_turn, 0.8)
work(cres_L, {hand="body", tool=fb12b, pile=bt_turn, coverage=1.5, length={20,50},
              angle=1.52, edge="found", clip=true})
fb12b:load(bt_bnc, 0.75)
work(cres_R, {hand="body", tool=fb12b, pile=bt_bnc, coverage=1.3, length={18,46},
              angle=1.52, edge="found", clip=true})
print(wait(150))
bt_glint = pile{{"lead white",1.3},{"green earth",1.2},{"raw umber",0.8},{"ultramarine blue",0.25}}
fb4c = brush("filbert", 4)
fb4c:load(bt_glint, 0.75)
fb4c:stroke({{201, 342}, {198, 366}}, {pressure={0.5, 0.3}, ramps={0.1, 0.5}, shake=0.6})
fb4c:reload(bt_glint, 0.85)
fb4c:stroke({{198, 376}, {196, 424}, {197, 446}}, {pressure={0.8, 0.55}, ramps={0.1, 0.35}, shake=0.5})
fb4c:reload(bt_glint, 0.8)
fb4c:stroke({{195, 458}, {195, 494}}, {pressure={0.6, 0.25}, ramps={0.12, 0.5}, shake=0.6})
fb4c:reload(bt_turn, 0.7)
fb4c:stroke({{214, 232}, {215, 262}, {216, 288}}, {pressure={0.6, 0.3}, ramps={0.12, 0.45}, shake=0.6})
fb4c:reload(bt_glint, 0.55)
fb4c:stroke({{210, 220}, {232, 215}, {254, 219}}, {pressure={0.45, 0.35}, ramps={0.12, 0.4}, shake=0.5})
print("bottle mod", drying(200, 400))

--@ chunk 415
print(wait(12 * 60))
-- The bowl's belly: a warm reflected light low on the front left.
belly2 = (ellipse(448, 578, 105, 46) * bowl_g2)
work(belly2, {hand="broad", tool=brush("filbert", 20), pile=bt_bnc, coverage=1.4,
              length={30,80}, angle=0.12, curve={0,0}, edge="found", clip=true})
print(wait(150))
-- Fruit: the calyx at the top and the light on the lit side.
fq_dk3 = pile{{"raw umber",2.2},{"bone black",1.4},{"green earth",0.8},{"burnt sienna",0.6}}
fq_hi2 = pile{{"lead white",1.6},{"cadmium yellow",2.0},{"transparent oxide yellow",0.4}}
bd = brush("filbert", 4)
for _, t in ipairs({{462, 399}, {576, 412}, {352, 574}, {728, 544}}) do
  bd:load(fq_dk3, 0.7)
  bd:stroke({{t[1] - 9, t[2] + 1}, {t[1] - 2, t[2] - 2}, {t[1] + 6, t[2] + 2}}, {pressure={0.6, 0.45}, ramps={0.15, 0.4}, shake=0.7})
  bd:reload(fq_dk3, 0.55)
  bd:stroke({{t[1] - 4, t[2] + 6}, {t[1] + 3, t[2] + 7}}, {pressure={0.5, 0.25}, ramps={0.2, 0.5}, shake=0.8})
end
bh = brush("filbert", 6)
for _, t in ipairs({{438, 424}, {446, 438}, {555, 434}, {562, 446}, {326, 600}, {340, 588}, {706, 567}, {714, 578}}) do
  bh:load(fq_hi2, 0.65)
  bh:touch(t[1], t[2], {pressure=0.62, drag={5, -4}})
end
print("accents", drying(438, 424))

--@ chunk 416
-- The belly light came out as a flat brown splodge on the bowl. Take it back to
-- the bowl's own dark: the black vessel with one bright lip is the stronger
-- statement anyway.
work(belly2, {hand="broad", tool=brush("filbert", 20), pile=bowl_dark, length={30,80},
              coverage=3.2, angle=0.12, curve={0,0}, edge="found", clip=true})
wait(150)
work(belly2, {hand="broad", tool=brush("filbert", 20), pile=bowl_dark, length={30,80},
              coverage=3.0, angle=1.5, curve={0,0}, edge="found", clip=true})
print("belly back", drying(448, 578))

--@ chunk 417
-- The wall is a dead flat field over half the picture. Give it strokes: a veil of
-- a warmer, slightly lighter brown, and the far corner going down.
print(wait(14 * 60))
wall_lt2 = pile{{"burnt sienna",2.6},{"raw umber",2.0},{"yellow ochre",0.6},{"bone black",1.0}}
wall_dk2 = pile{{"raw umber",3.0},{"burnt sienna",1.6},{"bone black",2.2},{"yellow ochre",0.2}}
fb34 = brush("filbert", 34)
work(wall_m3, {hand="broad", tool=fb34, pile=wall_lt2, length={70,190}, coverage=1.0,
               angle=0.32, edge="found", clip=true})
wait(130)
wall_corner = (rect(540, 0, 460, 470):roughen(34, 170, 3)) * wall_m3
work(wall_corner, {hand="broad", tool=fb34, pile=wall_dk2, length={70,190}, coverage=1.2,
                   angle=0.32, edge="found", clip=true})
print("wall strokes", drying(200, 200))

--@ chunk 418
-- The corner pass laid a field of dark worms. Put the wall back there with the
-- field recipe and leave it alone.
corner_fix = (rect(510, 0, 490, 500) * wall_m3):grow(10)
print("corner_fix", corner_fix:area())
work(corner_fix, {hand="broad", tool=bigf, pile=wall_w2, length={110,230}, coverage=3, angle=0.06,
                  curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work(corner_fix, {hand="broad", tool=bigf, pile=wall_w2, length={110,230}, coverage=3, angle=1.5,
                  curve={0,0}, edge="found", clip=true, ruler=true})
print("wall back", drying(800, 200))

--@ chunk 419
-- A soft glow on the wall behind the bowl, and a faint warm bounce on the
-- table in front of it. Both as thin broken veils, not repaints.
print(wait(14 * 60))
wall_gl = pile{{"burnt sienna",2.4},{"raw umber",2.2},{"yellow ochre",0.7},{"bone black",0.8}}
wall_gl2 = pile{{"burnt sienna",2.4},{"raw umber",1.8},{"yellow ochre",1.0},{"burnt sienna",0.6},{"lead white",0.2}}
G1 = (ellipse(545, 400, 370, 235):roughen(30, 170, 5)) * wall_m3
work(G1, {hand="broad", tool=bigf, pile=wall_gl, length={90,200}, coverage=1.1,
          angle=0.3, edge="found", clip=true})
wait(130)
G2 = (ellipse(520, 395, 235, 155):roughen(24, 140, 7)) * wall_m3
work(G2, {hand="broad", tool=bigf, pile=wall_gl2, length={90,200}, coverage=1.0,
          angle=0.3, edge="found", clip=true})
wait(130)
tb = pile{{"burnt sienna",2.6},{"yellow ochre",1.0},{"raw umber",2.0},{"bone black",0.6}}
work((ellipse(470, 632, 160, 48):roughen(16, 90, 12)) * (table_m - obj3),
     {hand="broad", tool=brush("filbert", 26), pile=tb, length={30,90}, coverage=1.0,
      angle=0.15, edge="found", clip=true})
print("glow", drying(545, 300))

--@ chunk 420
-- The glow came out as pale chalk marks. Knock it back to the wall, then try one
-- band whose value is barely off the wall's so it reads as warmth, not paint.
work((G1 + G2):grow(12), {hand="broad", tool=bigf, pile=wall_w2, length={90,200},
         coverage=3, angle=0.06, curve={0,0}, edge="found", clip=true, ruler=true})
wait(150)
work((G1 + G2):grow(12), {hand="broad", tool=bigf, pile=wall_w2, length={90,200},
         coverage=3, angle=1.5, curve={0,0}, edge="found", clip=true, ruler=true})
print(wait(140))
wall_warm = pile{{"burnt sienna",3.0},{"raw umber",2.2},{"yellow ochre",0.9},{"bone black",1.2},{"lead white",0.1}}
G3 = (ellipse(530, 400, 330, 215):roughen(30, 170, 5)) * wall_m3
work(G3, {hand="broad", tool=bigf, pile=wall_warm, length={90,200}, coverage=1.2,
          angle=0.3, edge="found", clip=true})
print("glow2", drying(545, 300))

--@ chunk 421
-- The glow knock-back was masked on the WALL, so its 12-unit rim came back over
-- the objects: the bottle's turn and the tops of the two quinces in the bowl are
-- gone. Flatten the bottle and lay its modelling again.
print(wait(14 * 60))
work(bottle_g3, {hand="broad", tool=brush("filbert", 22), pile=bt_flat2, length={50,120},
                 coverage=3.4, angle=1.5, curve={0,0}, edge="found", clip=true})
wait(140)
work(bottle_g3, {hand="broad", tool=brush("filbert", 22), pile=bt_flat2, length={50,120},
                 coverage=3.0, angle=0.1, curve={0,0}, edge="found", clip=true})
wait(140)
fb12b:load(bt_turn, 0.8)
work(cres_L, {hand="body", tool=fb12b, pile=bt_turn, coverage=1.5, length={20,50},
              angle=1.52, edge="found", clip=true})
fb12b:load(bt_bnc, 0.75)
work(cres_R, {hand="body", tool=fb12b, pile=bt_bnc, coverage=1.3, length={18,46},
              angle=1.52, edge="found", clip=true})
print(wait(140))
fb4c:load(bt_glint, 0.75)
fb4c:stroke({{201, 342}, {198, 366}}, {pressure={0.5, 0.3}, ramps={0.1, 0.5}, shake=0.6})
fb4c:reload(bt_glint, 0.85)
fb4c:stroke({{198, 376}, {196, 424}, {197, 446}}, {pressure={0.8, 0.55}, ramps={0.1, 0.35}, shake=0.5})
fb4c:reload(bt_glint, 0.8)
fb4c:stroke({{195, 458}, {195, 494}}, {pressure={0.6, 0.25}, ramps={0.12, 0.5}, shake=0.6})
fb4c:reload(bt_turn, 0.7)
fb4c:stroke({{214, 232}, {215, 262}, {216, 288}}, {pressure={0.6, 0.3}, ramps={0.12, 0.45}, shake=0.6})
fb4c:reload(bt_glint, 0.55)
fb4c:stroke({{210, 220}, {232, 215}, {254, 219}}, {pressure={0.45, 0.35}, ramps={0.12, 0.4}, shake=0.5})
print("bottle again", drying(200, 400))

--@ chunk 422
-- The tops of the two quinces in the bowl were cut flat by that wall mask. Lay
-- their crowns back.
print(wait(14 * 60))
topA = qA2 * rect(0, 378, 1000, 74)
topB = qB2 * rect(0, 378, 1000, 86)
fb10e = brush("filbert", 10)
fb10e:load(fq_L3, 0.9)
work(topA, {hand="body", tool=fb10e, pile=fq_L3, coverage=2.4, length={16,44},
            angle=-0.5, edge="found", clip=true})
work(topB, {hand="body", tool=fb10e, pile=fq_L3, coverage=2.4, length={16,44},
            angle=-0.5, edge="found", clip=true})
print(wait(140))
caps = (ellipse(442, 424, 32, 24) * qA2) + (ellipse(558, 434, 25, 20) * qB2)
fb8e = brush("filbert", 8)
fb8e:load(fq_hi2, 0.8)
work(caps, {hand="body", tool=fb8e, pile=fq_hi2, coverage=1.7, length={14,36},
            angle=-0.6, edge="found", clip=true})
print(wait(140))
bd:load(fq_dk3, 0.7)
bd:stroke({{453, 400}, {460, 397}, {467, 401}}, {pressure={0.6, 0.45}, ramps={0.15, 0.4}, shake=0.7})
bd:reload(fq_dk3, 0.55)
bd:stroke({{458, 405}, {465, 406}}, {pressure={0.5, 0.25}, ramps={0.2, 0.5}, shake=0.8})
bd:reload(fq_dk3, 0.7)
bd:stroke({{567, 413}, {574, 410}, {581, 414}}, {pressure={0.6, 0.45}, ramps={0.15, 0.4}, shake=0.7})
bd:reload(fq_dk3, 0.55)
bd:stroke({{572, 418}, {579, 419}}, {pressure={0.5, 0.25}, ramps={0.2, 0.5}, shake=0.8})
print("crowns", drying(462, 400))

--@ chunk 423
-- Those crowns came out as flat caps with hard edges. Rebuild the two quinces
-- in the bowl with the sequence that worked on day 65: mid over the whole
-- fruit, light on the lit side of one terminator, shadow crescent, blend.
print(wait(14 * 60))
fb13:load(fq_md2, 0.9)
work(qA2, {hand="body", tool=fb13, pile=fq_md2, coverage=2.8, length={20,55},
           angle=0.5, edge="found", clip=true})
fb13:load(fq_md2, 0.9)
work(qB2, {hand="body", tool=fb13, pile=fq_md2, coverage=2.8, length={20,55},
           angle=0.5, edge="found", clip=true})
print(wait(140))
termA = (ellipse(452, 449, 64, 61) * qA2):roughen(3, 40, 466)
termB = (ellipse(566, 457, 56, 54) * qB2):roughen(3, 40, 578)
fb12c:load(fq_L3, 0.9)
work(termA, {hand="broad", tool=fb12c, pile=fq_L3, coverage=2.0, length={22,60},
             angle=0.8, edge="found", clip=true})
fb12c:load(fq_L3, 0.9)
work(termB, {hand="broad", tool=fb12c, pile=fq_L3, coverage=2.0, length={22,60},
             angle=0.8, edge="found", clip=true})
print(wait(140))
fb10d:load(fq_sh2, 0.9)
work(qA2 - termA, {hand="body", tool=fb10d, pile=fq_sh2, coverage=1.9, length={18,48},
                   angle=1.5, edge="found", clip=true})
fb10d:load(fq_sh2, 0.9)
work(qB2 - termB, {hand="body", tool=fb10d, pile=fq_sh2, coverage=1.9, length={18,48},
                   angle=1.5, edge="found", clip=true})
print(wait(140))
blend(qA2, {angle=0.4})
blend(qB2, {angle=0.4})
print("rebuilt", drying(466, 461))

--@ chunk 424
-- The blend left them as pale smooth domes. Give them back their turn: a
-- shadow crescent on the lower right, a small lit cap on the upper left.
print(wait(14 * 60))
shA = qA2 - ellipse(458, 450, 58, 55)
shB = qB2 - ellipse(570, 456, 50, 50)
fb10d:load(fq_sh2, 0.9)
work(shA, {hand="body", tool=fb10d, pile=fq_sh2, coverage=1.9, length={16,44},
           angle=1.45, edge="found", clip=true})
fb10d:load(fq_sh2, 0.9)
work(shB, {hand="body", tool=fb10d, pile=fq_sh2, coverage=1.9, length={16,44},
           angle=1.45, edge="found", clip=true})
print(wait(140))
capA = ellipse(444, 430, 32, 27) * qA2
capB = ellipse(560, 442, 26, 23) * qB2
fb8e:load(fq_hi2, 0.8)
work(capA, {hand="body", tool=fb8e, pile=fq_hi2, coverage=1.5, length={14,36},
            angle=-0.6, edge="found", clip=true})
fb8e:load(fq_hi2, 0.8)
work(capB, {hand="body", tool=fb8e, pile=fq_hi2, coverage=1.5, length={14,36},
            angle=-0.6, edge="found", clip=true})
print("turned", drying(466, 461))

--@ chunk 425
-- Still patches. The fruit on the table are mottled dark over gold; that speckle
-- is what is missing. Break the two in the bowl up the same way.
print(wait(14 * 60))
fb7 = brush("filbert", 7)
fb7:load(fq_sh2, 0.75)
work(qA2, {hand="body", tool=fb7, pile=fq_sh2, coverage=0.95, length={10,26},
           angle=1.2, edge="found", clip=true})
fb7:load(fq_sh2, 0.75)
work(qB2, {hand="body", tool=fb7, pile=fq_sh2, coverage=0.95, length={10,26},
           angle=1.2, edge="found", clip=true})
print(wait(140))
fb9e = brush("filbert", 9)
fb9e:load(fq_md2, 0.7)
work(qA2, {hand="body", tool=fb9e, pile=fq_md2, coverage=0.85, length={12,30},
           angle=0.7, edge="found", clip=true})
fb9e:load(fq_md2, 0.7)
work(qB2, {hand="body", tool=fb9e, pile=fq_md2, coverage=0.85, length={12,30},
           angle=0.7, edge="found", clip=true})
print("mottled", drying(466, 461))

--@ chunk 426
-- Too dark now. Gold them back over the mottle at a coverage that lets some of
-- the dashes stay, then the accents.
print(wait(14 * 60))
fb11:load(fq_L3, 0.9)
work(qA2, {hand="body", tool=fb11, pile=fq_L3, coverage=1.5, length={16,46},
           angle=-0.55, edge="found", clip=true})
fb11:load(fq_L3, 0.9)
work(qB2, {hand="body", tool=fb11, pile=fq_L3, coverage=1.5, length={16,46},
           angle=-0.55, edge="found", clip=true})
print(wait(140))
bd:load(fq_dk3, 0.7)
bd:stroke({{453, 400}, {460, 397}, {467, 401}}, {pressure={0.55, 0.4}, ramps={0.15, 0.4}, shake=0.7})
bd:reload(fq_dk3, 0.5)
bd:stroke({{458, 405}, {465, 406}}, {pressure={0.45, 0.2}, ramps={0.2, 0.5}, shake=0.8})
bd:reload(fq_dk3, 0.7)
bd:stroke({{567, 413}, {574, 410}, {581, 414}}, {pressure={0.55, 0.4}, ramps={0.15, 0.4}, shake=0.7})
bd:reload(fq_dk3, 0.5)
bd:stroke({{572, 418}, {579, 419}}, {pressure={0.45, 0.2}, ramps={0.2, 0.5}, shake=0.8})
bh:load(fq_hi2, 0.7)
bh:touch(442, 428, {pressure=0.6, drag={6, -5}})
bh:touch(452, 440, {pressure=0.45, drag={5, -4}})
bh:touch(558, 440, {pressure=0.55, drag={5, -4}})
print("gilded", drying(466, 461))

--@ chunk 427
-- A thin warm edge along the bowl's front-left silhouette where the light
-- catches it, and a faint veil of strokes to keep the wall from being dead.
print(wait(14 * 60))
bowl_edge_p = pile{{"raw umber",2.2},{"burnt sienna",1.8},{"yellow ochre",0.6},{"lead white",0.3}}
fb5d = brush("filbert", 5)
fb5d:load(bowl_edge_p, 0.7)
fb5d:stroke({{350, 512}, {362, 534}, {374, 552}}, {pressure={0.4, 0.25}, ramps={0.15, 0.5}, shake=0.8})
fb5d:reload(bowl_edge_p, 0.8)
fb5d:stroke({{384, 560}, {404, 572}, {424, 580}}, {pressure={0.55, 0.4}, ramps={0.12, 0.4}, shake=0.7})
fb5d:reload(bowl_edge_p, 0.75)
fb5d:stroke({{434, 583}, {462, 591}, {490, 596}}, {pressure={0.5, 0.3}, ramps={0.12, 0.45}, shake=0.7})
fb5d:reload(bowl_edge_p, 0.6)
fb5d:stroke({{500, 598}, {520, 600}}, {pressure={0.35, 0.15}, ramps={0.15, 0.5}, shake=0.8})
print(wait(140))
work(wall_m3, {hand="broad", tool=bigf, pile=wall_warm, length={100,220}, coverage=0.6,
               angle=0.3, edge="found", clip=true})
print("edge + veil", drying(400, 570))

--@ chunk 428
-- The wall is half the picture and dead flat. Three veils of almost the same
-- brown at different angles: too little contrast to mark, enough to give the
-- field a painterly grain.
print(wait(14 * 60))
wv1 = pile{{"burnt sienna",3.0},{"raw umber",2.2},{"yellow ochre",0.9},{"bone black",1.5}}
wv2 = pile{{"burnt sienna",3.0},{"raw umber",2.4},{"yellow ochre",0.7},{"bone black",1.7}}
wv3 = pile{{"burnt sienna",3.1},{"raw umber",2.0},{"yellow ochre",1.0},{"bone black",1.5}}
work(wall_m3, {hand="broad", tool=bigf, pile=wv1, length={100,220}, coverage=0.6,
               angle=0.2, edge="found", clip=true})
wait(130)
work(wall_m3, {hand="broad", tool=bigf, pile=wv2, length={100,220}, coverage=0.6,
               angle=1.2, edge="found", clip=true})
wait(130)
work(wall_m3, {hand="broad", tool=bigf, pile=wv3, length={100,220}, coverage=0.6,
               angle=-0.4, edge="found", clip=true})
print("wall grain", drying(200, 200))

--@ chunk 429
capD = ellipse(710, 568, 28, 24) * qD2
fb11:load(fq_L3, 0.85)
work(capD, {hand="body", tool=fb11, pile=fq_L3, coverage=1.4, length={14,38},
            angle=-0.6, edge="found", clip=true})
print(wait(140))
bq = pile{{"burnt sienna",2.4},{"raw umber",2.0},{"yellow ochre",0.8},{"lead white",0.1}}
bounceD = ellipse(752, 628, 24, 17) * qD2
fb10f = brush("filbert", 10)
fb10f:load(bq, 0.7)
work(bounceD, {hand="body", tool=fb10f, pile=bq, coverage=1.1, length={12,30},
               angle=1.2, edge="found", clip=true})
print("D", drying(730, 600))

--@ chunk 430
-- Those two blobs went on like stickers. Break them with the fruit's own
-- mottling again.
print(wait(12 * 60))
fb7:load(fq_sh2, 0.7)
work(qD2, {hand="body", tool=fb7, pile=fq_sh2, coverage=0.8, length={10,26},
           angle=1.25, edge="found", clip=true})
print(wait(140))
fb9e:load(fq_md2, 0.65)
work(qD2, {hand="body", tool=fb9e, pile=fq_md2, coverage=0.7, length={12,30},
           angle=-0.5, edge="found", clip=true})
print("D mottle", drying(730, 600))

--@ chunk 431
-- That came out as pale worms. Rebuild D exactly the way A and B ended up good.
print(wait(14 * 60))
fb13:load(fq_md2, 0.9)
work(qD2, {hand="body", tool=fb13, pile=fq_md2, coverage=2.8, length={20,55},
           angle=0.5, edge="found", clip=true})
print(wait(140))
termD = (ellipse(717, 589, 59, 57) * qD2):roughen(3, 40, 730)
fb12c:load(fq_L3, 0.9)
work(termD, {hand="broad", tool=fb12c, pile=fq_L3, coverage=2.0, length={22,60},
             angle=0.8, edge="found", clip=true})
print(wait(140))
fb10d:load(fq_sh2, 0.9)
work(qD2 - termD, {hand="body", tool=fb10d, pile=fq_sh2, coverage=1.9, length={18,48},
                   angle=1.5, edge="found", clip=true})
print(wait(140))
blend(qD2, {angle=0.4})
print("D rebuilt", drying(730, 600))

--@ chunk 432
print(wait(14 * 60))
fb7:load(fq_sh2, 0.75)
work(qD2, {hand="body", tool=fb7, pile=fq_sh2, coverage=0.9, length={10,26},
           angle=1.2, edge="found", clip=true})
print(wait(140))
fb11:load(fq_L3, 0.9)
work(qD2, {hand="body", tool=fb11, pile=fq_L3, coverage=1.5, length={16,46},
           angle=-0.55, edge="found", clip=true})
print(wait(140))
bd:load(fq_dk3, 0.7)
bd:stroke({{719, 544}, {726, 541}, {733, 545}}, {pressure={0.55, 0.4}, ramps={0.15, 0.4}, shake=0.7})
bd:reload(fq_dk3, 0.5)
bd:stroke({{724, 549}, {731, 550}}, {pressure={0.45, 0.2}, ramps={0.2, 0.5}, shake=0.8})
bh:load(fq_hi2, 0.7)
bh:touch(706, 566, {pressure=0.55, drag={5, -4}})
print("D done", drying(730, 600))

--@ chunk 433
shD = qD2 - ellipse(722, 592, 52, 50)
fb10d:load(fq_sh2, 0.9)
work(shD, {hand="body", tool=fb10d, pile=fq_sh2, coverage=1.5, length={16,44},
           angle=1.45, edge="found", clip=true})
print(wait(140))
fq_hot = pile{{"lead white",2.4},{"cadmium yellow",2.2},{"transparent oxide yellow",0.5}}
bh2 = brush("filbert", 9)
bh2:load(fq_hot, 1.0)
bh2:touch(322, 592, {pressure=0.85, drag={9, -7}})
bh2:reload(fq_hot, 1.0)
bh2:touch(348, 580, {pressure=0.7, drag={8, -6}})
bh2:reload(fq_hot, 0.9)
bh2:touch(436, 420, {pressure=0.6, drag={7, -5}})
print("last accents", drying(730, 600))

--@ chunk 434
-- The two quinces in the bowl are as bright as the ones on the table, so the
-- bowl reads no further back than the foreground. Knock them down one step.
print(wait(16 * 60))
fruit_back = pile{{"raw umber",3.0},{"burnt sienna",2.4},{"bone black",1.2},{"yellow ochre",0.3}}
fb13:load(fruit_back, 0.6)
work(qA2, {hand="body", tool=fb13, pile=fruit_back, coverage=0.6, length={20,50},
           angle=1.1, edge="found", clip=true})
fb13:load(fruit_back, 0.6)
work(qB2, {hand="body", tool=fb13, pile=fruit_back, coverage=0.6, length={20,50},
           angle=1.1, edge="found", clip=true})
print("set back", drying(466, 461))

--@ chunk 435
-- The veil came on as dark worms. Cover them with a deeper gold instead, one
-- step darker than the fruit on the table, which is what I wanted in the first
-- place.
print(wait(14 * 60))
fq_Lb = pile{{"yellow ochre",2.8},{"cadmium yellow",0.3},{"transparent oxide yellow",0.3},
             {"burnt sienna",1.8},{"lead white",0.2}}
fb11:load(fq_Lb, 0.9)
work(qA2, {hand="body", tool=fb11, pile=fq_Lb, coverage=1.6, length={16,46},
           angle=-0.55, edge="found", clip=true})
fb11:load(fq_Lb, 0.9)
work(qB2, {hand="body", tool=fb11, pile=fq_Lb, coverage=1.6, length={16,46},
           angle=-0.55, edge="found", clip=true})
print(wait(140))
fb7:load(fq_sh2, 0.55)
work(qA2, {hand="body", tool=fb7, pile=fq_sh2, coverage=0.5, length={10,24},
           angle=1.3, edge="found", clip=true})
fb7:load(fq_sh2, 0.55)
work(qB2, {hand="body", tool=fb7, pile=fq_sh2, coverage=0.5, length={10,24},
           angle=1.3, edge="found", clip=true})
print("regold", drying(466, 461))

--@ chunk 436
-- Back to the mottle that worked on these two: dark dashes, then the deeper gold
-- over them, then the crown. No more passes after this.
print(wait(14 * 60))
fb7:load(fq_sh2, 0.75)
work(qA2, {hand="body", tool=fb7, pile=fq_sh2, coverage=0.9, length={10,26},
           angle=1.2, edge="found", clip=true})
fb7:load(fq_sh2, 0.75)
work(qB2, {hand="body", tool=fb7, pile=fq_sh2, coverage=0.9, length={10,26},
           angle=1.2, edge="found", clip=true})
print(wait(140))
fb11:load(fq_Lb, 0.9)
work(qA2, {hand="body", tool=fb11, pile=fq_Lb, coverage=1.5, length={16,46},
           angle=-0.55, edge="found", clip=true})
fb11:load(fq_Lb, 0.9)
work(qB2, {hand="body", tool=fb11, pile=fq_Lb, coverage=1.5, length={16,46},
           angle=-0.55, edge="found", clip=true})
print(wait(140))
bd:load(fq_dk3, 0.7)
bd:stroke({{453, 400}, {460, 397}, {467, 401}}, {pressure={0.55, 0.4}, ramps={0.15, 0.4}, shake=0.7})
bd:reload(fq_dk3, 0.5)
bd:stroke({{458, 405}, {465, 406}}, {pressure={0.45, 0.2}, ramps={0.2, 0.5}, shake=0.8})
bd:reload(fq_dk3, 0.7)
bd:stroke({{567, 413}, {574, 410}, {581, 414}}, {pressure={0.55, 0.4}, ramps={0.15, 0.4}, shake=0.7})
bd:reload(fq_dk3, 0.5)
bd:stroke({{572, 418}, {579, 419}}, {pressure={0.45, 0.2}, ramps={0.2, 0.5}, shake=0.8})
bh:load(fq_hi2, 0.6)
bh:touch(442, 428, {pressure=0.45, drag={5, -4}})
bh:touch(558, 440, {pressure=0.4, drag={4, -3}})
print("final fruit", drying(466, 461))
