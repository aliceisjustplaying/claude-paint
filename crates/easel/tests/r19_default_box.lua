-- easel session "r19": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=300, aspect=4, linen=15, seed=11, ground={{pile={{"lead white", 3}, {"red earth", 1}}, um=110, apply="knife"}, {pile={{"lead white", 5}, {"yellow ochre", 1}}, um=60, apply="brush"}}}

--@ chunk 2
local names = tubes()
assert(#names == 14, #names)
print(table.concat(names, ", "))
for i, n in ipairs(names) do
  local p = pile{{n, 2}, {"lead white", 1}, medium=0.2}
  work(rect((i - 1) * 1000 / 14, 0, 1000 / 14, 125), {hand="body", pile=p, angle=0.3, coverage=2})
end

--@ chunk 3
wait(120)
b = brush("filbert", 6); b:load(pile{{"Prussian blue", 1}, {"chrome yellow", 2}, {"green earth", 1}}, 0.8)
for i = 1, 8 do b:stroke({{60 + i * 100, 200}, {120 + i * 100, 140}}) end
work(rect(0, 150, 1000, 100), {hand="glaze", pile=pile{{"raw umber", 1}, {"copper green", 1}, medium=0.7}})
stipple(rect(100, 170, 800, 60), {width=3, pile=pile{{"vermilion", 1}, {"Rinmann's green", 1}}, coverage=1.2})
