-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=1140, aspect=1.5, linen={18,16}, seed=1891,
 ground={{pile={{"lead white",10},{"yellow ochre",0.3}}, um=120, apply="knife", texture=0.3},
         {pile={{"raw umber",1},{"lead white",2}}, um=10, apply="brush"}}}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2
c = chalk()
-- horizon / distant tree line
c:sketch({{0,405},{150,398},{300,402},{450,395},{600,400},{800,396},{1000,402}}, {pressure=0.25})
-- left tree group silhouette
c:sketch({{60,420},{55,330},{70,240},{110,170},{170,120},{240,95},{300,110},{350,150},{385,210},{400,290},{395,360},{380,420}}, {pressure=0.3})
-- slender tree right
c:sketch({{705,430},{702,350},{708,260},{700,190}}, {pressure=0.3})
c:sketch({{660,330},{690,230},{720,180},{750,240},{760,330}}, {pressure=0.2})
-- pool
c:sketch({{520,470},{600,458},{720,455},{830,465},{790,505},{650,515},{540,500},{520,470}}, {pressure=0.25})
-- path
c:sketch({{120,667},{220,600},{330,540},{430,490},{520,465}}, {pressure=0.25})
-- figure
c:sketch({{612,440},{612,468}}, {pressure=0.35})

--@ chunk 3
umb = pile{{"raw umber",3},{"bone black",0.4},{"raw sienna",1}, thinner=0.6}
dk  = pile{{"raw umber",2},{"bone black",1},{"Antwerp blue",0.2}, thinner=0.5}
land = below(function(x) return 400 + 4*math.sin(x/90) end)
trees = poly({{55,420},{50,330},{62,250},{95,185},{140,140},{185,115},{230,92},{280,100},{320,125},{355,160},{378,215},{395,280},{398,350},{385,420}}, true):roughen(18, 60, 3)
-- whole land and trees thin umber wash
work(land + trees, {hand="broad", pile=umb, coverage=1.2, angle=function(x,y) return y>400 and 0.05 or 1.4 end})
print(drying(300,550))
