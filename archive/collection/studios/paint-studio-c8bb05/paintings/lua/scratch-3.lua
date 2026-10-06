-- easel session "scratch": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box every
--@ engine 5

--@ chunk 1
canvas{aspect=1.3, ground={{apply="knife", pile={{"lead white", 6}, {"yellow ochre", 1}, {"red earth", 0.4}}, texture=0.4, um=120}}, linen={18, 16}, seed=417, size=650}

--@ chunk 2

st_lit = pile{{"cadmium yellow",1},{"orange chrome",0.6},{"lead white",1},{"Naples yellow",0.6}, name="st_lit"}
st_hot = pile{{"orange chrome",1},{"vermilion",0.3},{"yellow ochre",0.6},{"lead white",0.3}, name="st_hot"}
f_gold = pile{{"lead white",0.34},{"cadmium yellow",0.34},{"yellow ochre",0.21},{"orange chrome",0.10}, name="f_gold"}
f_deep = pile{{"Mars orange",0.31},{"burnt sienna",0.16},{"yellow ochre",0.31},{"cobalt violet",0.12},{"lead white",0.09}, name="f_deep"}
st_shade = pile{{"cobalt violet",1},{"burnt sienna",0.6},{"rose madder",0.3},{"lead white",0.6},{"ultramarine blue",0.2}, name="st_shade"}
sh_mid2 = pile{{"cobalt violet",1},{"ultramarine blue",0.5},{"burnt sienna",0.3},{"lead white",0.9},{"rose madder",0.2}, name="sh_mid2"}
f_lav = pile{{"cobalt violet",1},{"lead white",1.2},{"ultramarine blue",0.25},{"rose madder",0.2}, name="f_lav"}
local m = rect(50,50,900,500)
work(m, {hand="broad", piles={{f_gold,function() return 1 end},{st_hot,function() return 0.5 end}}, coverage=2, fill=true})
work(m, {hand="hatch", piles={{st_lit,function() return 0.8 end},{st_hot,function(x,y) return 0.6 end},{f_deep,function(x,y) return 0.4 end},{f_gold,function() return 0.4 end}}, coverage=1.3, scale_at=function(x,y) return 0.8+1.5*y/550 end, length={8,22}, angle=0.0, mix_jitter=0.4})
print(wait(20*24*60))

--@ chunk 3

print(drying(300,300), drying(800,500))
local depth = function(y) return clamp(y/550,0,1) end
local sc = function(x,y) return 0.8+1.5*depth(y) end
work(rect(50,50,300,500), {hand="hatch", pile=sh_mid2, coverage=0.15, scale_at=sc, length={8,22}, angle=0.0, pressure={0.4,0.7}, load=0.5, seed=1})
work(rect(350,50,300,500), {hand="hatch", pile=st_shade, coverage=0.15, scale_at=sc, length={8,22}, angle=0.0, pressure={0.4,0.7}, load=0.5, seed=2})
work(rect(650,50,300,500), {hand="hatch", pile=f_lav, coverage=0.15, scale_at=sc, length={8,22}, angle=0.0, pressure={0.4,0.7}, load=0.5, seed=3})

--@ chunk 4

blend(rect(60,60,280,480), {angle=0.0})
local r = rag{width=30}
r:dip(0.4)
r:wipe(rect(360,60,280,480), {pressure=0.45, angle=0, passes=1, refold=0.3})
