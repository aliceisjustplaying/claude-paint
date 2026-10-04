-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=950, aspect=1.36, linen={17,14}, seed=28, ground={{pile={{"yellow ochre",2},{"red earth",1},{"lead white",1}},um=95,apply="knife",texture=0.22},{pile={{"lead white",3},{"yellow ochre",1}},um=30,apply="brush"}}}

--@ chunk 2
pSky = pile{{"smalt",3},{"cobalt blue",2},{"lead white",3},{"red earth",0.5},{"yellow ochre",0.35}}; sky = above(function(x) return 418 + 7*math.sin(x/150) + 0.014*(x-500) end); work(sky,{hand="broad",pile=pSky,angle=0.18,coverage=1.4,fill=true})

--@ chunk 3
wait(1440); pTop = pile{{"smalt",3},{"cobalt blue",2},{"red earth",0.8},{"bone black",0.2},{"lead white",0.7},medium=0.15}; topmask=above(function(x) return 260 end):soften(105); work(topmask,{hand="glaze",pile=pTop}); pMid=pile{{"smalt",2},{"cobalt blue",2},{"red earth",0.45},{"lead white",2},{"yellow ochre",0.4}}; midsky=rect(0,180,1000,240):soften(95)*sky; work(midsky,{hand="broad",pile=pMid,angle=0.06,coverage=1.4,fill=true}); pGlow=pile{{"lead white",4},{"yellow ochre",1.2},{"vermilion",0.4},{"red earth",0.22},{"pale smalt",0.4}}; glow=rect(0,335,1000,95):soften(55)*sky; work(glow,{hand="glaze",pile=pGlow})
