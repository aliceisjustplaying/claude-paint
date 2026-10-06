-- easel session "scratch": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box every
--@ engine 5

--@ chunk 1
canvas{aspect=1.3, ground={{apply="knife", pile={{"lead white", 6}, {"yellow ochre", 1}, {"red earth", 0.4}}, texture=0.4, um=120}}, linen={18, 16}, seed=417, size=650}

--@ chunk 2
sh_core = pile{{"ultramarine blue",1},{"cobalt violet",0.8},{"permanent alizarin",0.35},{"burnt sienna",0.25},{"lead white",0.35}, name="sh_core"}
sh_mid2 = pile{{"cobalt violet",1},{"ultramarine blue",0.5},{"burnt sienna",0.3},{"lead white",0.9},{"rose madder",0.2}, name="sh_mid2"}
st_hot = pile{{"orange chrome",1},{"vermilion",0.3},{"yellow ochre",0.6},{"lead white",0.3}, name="st_hot"}
f_deep = pile{{"Mars orange",1},{"burnt sienna",0.5},{"yellow ochre",1},{"cobalt violet",0.4},{"lead white",0.3}, name="f_deep"}
work(rect(50,50,400,300), {hand="hatch", piles={{sh_core,function() return 1 end},{sh_mid2,function() return 0.8 end}}, coverage=2.2, fill=true, angle=-0.1})
work(rect(450,50,300,300), {hand="hatch", piles={{st_hot,function() return 1 end},{f_deep,function() return 0.6 end}}, coverage=2.2, fill=true, angle=-0.1})
print(wait(20*24*60))

--@ chunk 3
sh_rose = pile{{"cobalt violet",1},{"rose madder",0.5},{"vermilion",0.12},{"lead white",0.6},{"burnt sienna",0.2}, name="sh_rose"}
sh_warm2 = pile{{"cobalt violet",1},{"Mars orange",0.4},{"rose madder",0.3},{"lead white",0.5}, name="sh_warm2"}
work(rect(60,60,180,130), {hand="hatch", pile=sh_rose, coverage=0.5, angle=-0.1, length={8,20}, pressure={0.4,0.8}, seed=5})
work(rect(60,200,180,130), {hand="hatch", pile=sh_warm2, coverage=0.5, angle=-0.1, length={8,20}, pressure={0.4,0.8}, seed=6})
local n = noise{seed=3, period=50, octaves=3}
local edge = mask(function(x,y) local xe = 450 + 18*n(x*0.3,y); return clamp(1 - math.abs(x-xe)/14, 0, 1) end) * rect(0,50,1000,300)
work(edge, {hand="hatch", piles={{st_hot,function() return 1 end},{f_deep,function() return 0.6 end}}, coverage=1.2, angle=-0.1, length={10,24}, pressure={0.45,0.8}, threshold=0.35, mix_jitter=0.4, seed=7})

--@ chunk 4
sh_plum = pile{{"cobalt violet",1},{"permanent alizarin",0.3},{"burnt sienna",0.3},{"ultramarine blue",0.15},{"lead white",0.35}, name="sh_plum"}
work(rect(260,60,170,270), {hand="hatch", piles={{sh_plum,function() return 1 end},{sh_rose,function() return 0.3 end}}, coverage=0.35, angle=-0.1, length={8,20}, pressure={0.4,0.8}, seed=8})
