--@ engine 5
--@ chunk 1
canvas{size=600, aspect=4, linen={16,17}, seed=77, ground={{pile={{"lead white",12},{"yellow ochre",0.25}}, um=130, apply="brush"}}}
--@ chunk 2
for i,n in ipairs({"cobalt blue", "yellow ochre", "vermilion", "raw umber"}) do
  work(rect((i-1)*250, 0, 240, 110), {hand="body", pile=pile{{n,1},{"lead white",2}}, coverage=0.4, angle=0.03, tool="filbert 16", streak=0.4})
end
--@ chunk 3
blend(rect(0, 0, 1000, 110), {angle=0.02})
wait(1440)
