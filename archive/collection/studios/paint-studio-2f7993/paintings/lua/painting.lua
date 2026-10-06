-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=1140, aspect=1.5, linen={16,14}, seed=1891,
  ground={{pile={{"lead white",10},{"yellow ochre",0.6}}, um=120, apply="knife", texture=0.25},
          {pile={{"raw umber",2},{"red earth",1},{"lead white",3}}, um=8, apply="brush"}}}
print(W,H)
c = chalk()
-- gentle horizontal curves: horizon, meadow swells, pool; tree masses
c:sketch({{0,418},{200,410},{420,414},{640,408},{1000,402}}, {pressure=0.25})
c:sketch({{0,500},{250,480},{520,492},{800,470},{1000,476}}, {pressure=0.2})
c:sketch({{560,410},{540,300},{580,170},{680,95},{790,110},{880,190},{930,300},{915,405}}, {pressure=0.25})
c:sketch({{150,412},{140,300},{175,215},{225,200},{260,280},{262,410}}, {pressure=0.2})
c:sketch({{160,560},{260,535},{400,540},{470,565},{380,596},{230,592},{160,560}}, {pressure=0.2})
fix()
