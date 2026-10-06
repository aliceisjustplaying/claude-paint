-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 2

--@ chunk 1
canvas{size=480, aspect=1.3, linen={15, 14}, seed=1811,
  ground={
    {pile={{"red earth", 2}, {"yellow ochre", 2}, {"lead white", 6}}, um=80, apply="knife", texture=0.35},
    {pile={{"lead white", 8}, {"yellow ochre", 0.6}, {"raw umber", 0.2}}, um=50, apply="roller", texture=0.5}
  }}
print(W, H)
print(table.concat(tubes(), ", "))

--@ chunk 2
-- Composition geometry kept as globals for later chunks
hill_pts = {{-10,430},{60,406},{140,382},{210,368},{262,364},{322,374},{380,394},{430,420},{482,458},{530,500},{585,545},{650,588},{730,624},{820,654},{920,680},{1010,700}}
horizon_y = 490

oak = {
  trunk = {pts={{252,370},{248,350},{251,330},{256,310},{258,292}}, w={30,25,23,21,19}},
  L1 = {pts={{258,292},{238,268},{214,252},{190,236},{172,208},{150,184},{138,152},{130,120},{136,88}}, w={17,15,13,12,10,8,6,4,2}},
  L2 = {pts={{258,290},{268,262},{262,236},{272,208},{266,176},{276,146},{270,112},{280,80},{276,52}}, w={15,13,11,10,8,6,5,3,1.5}},
  L3 = {pts={{260,296},{292,284},{326,282},{358,270},{392,262},{420,246},{448,240},{476,228}}, w={14,12,10,9,7,5,3.5,2}},
  S1 = {pts={{252,324},{226,314},{200,320},{172,316},{146,326},{120,324}}, w={12,10,8,6,4,2}},
}

h = pencil("2H")
-- horizon and far shore
h:line({{440,491},{600,490},{760,491},{1000,490}}, {pressure=0.3})
-- hill and shore contour
h:line(hill_pts, {pressure=0.35})
-- oak skeleton
for _, k in ipairs({"trunk","L1","L2","L3","S1"}) do
  h:line(oak[k].pts, {pressure=0.3})
end
-- figure on the slope (feet at 382)
h:line({{346,383},{347,360},{345,350},{348,342},{353,340},{357,343},{358,352},{360,362},{361,383}}, {pressure=0.3})
-- faint moon ring
h:line({{636,205},{640,214},{648,222},{658,226}}, {pressure=0.2})
print(wait(0))

--@ chunk 3
sw = {
  {"zenith",   pile{{"smalt",3},{"lead white",1},{"Prussian blue",0.15}}},
  {"upper",    pile{{"smalt",2},{"lead white",3}}},
  {"mid",      pile{{"pale smalt",2},{"lead white",4},{"green earth",1}}},
  {"low",      pile{{"lead white",5},{"yellow ochre",0.7},{"chrome yellow",0.3}}},
  {"glow",     pile{{"lead white",4},{"chrome yellow",1},{"vermilion",0.3}}},
  {"deepglow", pile{{"lead white",3},{"vermilion",1},{"chrome yellow",1}}},
  {"cloudD",   pile{{"smalt",2},{"raw umber",1},{"lead white",2},{"red earth",0.5}}},
  {"cloudV",   pile{{"smalt",2},{"red earth",1},{"lead white",3}}},
  {"hillD",    pile{{"raw umber",3},{"Prussian blue",1},{"bone black",1}}},
  {"black",    pile{{"bone black",1}}},
  {"olive",    pile{{"green earth",2},{"yellow ochre",1},{"raw umber",2}}},
  {"white",    pile{{"lead white",1}}},
}
for i, s in ipairs(sw) do
  local x0 = 20 + (i-1)*80
  work(rect(x0, 715, 72, 48), {hand="body", pile=s[2], coverage=1.3, angle=0.1, clip=true, seed=i})
end
print(wait(0))

--@ chunk 4
sk = {}
sk[1] = pile{{"smalt",3},{"Prussian blue",0.25},{"raw umber",0.25},{"lead white",1.2}, medium=0.2}
sk[2] = pile{{"smalt",3},{"Prussian blue",0.1},{"lead white",2}, medium=0.2}
sk[3] = pile{{"smalt",2},{"lead white",3}, medium=0.2}
sk[4] = pile{{"pale smalt",2},{"lead white",4},{"green earth",0.6}, medium=0.2}
sk[5] = pile{{"lead white",5},{"pale smalt",0.6},{"green earth",0.4},{"yellow ochre",0.5}, medium=0.2}
sk[6] = pile{{"lead white",5},{"yellow ochre",0.7},{"chrome yellow",0.4}, medium=0.2}
sk[7] = pile{{"lead white",4},{"chrome yellow",1},{"vermilion",0.4}, medium=0.2}

work(rect(0,0,W,95), {hand="broad", pile=sk[1], angle=0, angle_jitter=0.04, coverage=2.5, fill=true, edge="soft", seed=11})
print(wait(0))
