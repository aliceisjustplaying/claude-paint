-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=440, aspect=2, linen=15, seed=24, ground={{pile={{"lead white",5},{"yellow ochre",1}},um=60,apply="brush"}}}; p=pile{{"raw umber",1},thinner=0.5}; b=brush("filbert",70); b:load(p,0.7); b:stroke({{100,150},{800,150}}); b:load(p,0.7); b:stroke({{100,210},{800,210}})

--@ chunk 2
r=rag{width=45,seed=4}; r:dip(0.5); r:wipe({{180,150},{420,150}}, {pressure=0.8}); r:wipe({{500,330},{780,330}}, {pressure=0.8}); print(r)

--@ chunk 3
r:refold(); r:wipe({{180,210},{420,210}}, {pressure=0.6}); q=pile{{"raw sienna",1},thinner=0.5}; b:load(q,0.5); b:stroke({{250,120},{270,250}}); b:load(pile{{"lead white",3},{"yellow ochre",1}},0.5); b:stroke({{400,230},{660,230}}); wait(0.5)

--@ chunk 4
r:wipe({{110,350},{350,350}}, {pressure=0.5}); print(r)
