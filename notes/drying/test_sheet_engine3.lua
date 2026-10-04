-- easel session "sheet": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 3

--@ chunk 1
canvas{size=300, aspect=2, linen=20, seed=7, ground={{pile={{"red earth", 1}, {"lead white", 2}, {"raw umber", 1}}, um=80, apply="knife"}}}

--@ chunk 2
-- a passage: lead white over the top half, bone black under it, in column i
-- (columns left to right: lifted 1 h, 4 h, 12 h, 24 h, 3 days after laying)
function passage(i)
  local x0 = 40 + (i - 1) * 190
  local b = brush("flat", 36)
  for k, spec in ipairs({{"lead white", 50, 225}, {"bone black", 275, 450}}) do
    local p = pile{{spec[1], 1}}
    for y = spec[2], spec[3], 25 do
      b:reload(p, 0.9)
      b:stroke({{x0, y}, {x0 + 150, y + 4}}, {pressure={0.8, 0.8}})
    end
  end
end
function lift(i)
  local x0 = 40 + (i - 1) * 190
  local r = brush("flat", 30)
  for _, y in ipairs({140, 365}) do
    for k = 1, 3 do
      r:wipe(1)
      r:stroke({{x0 + 15, y}, {x0 + 135, y}}, {pressure={0.9, 0.9}})
    end
  end
end

--@ chunk 3
passage(5)
wait(48 * 60)

--@ chunk 4
passage(4)
wait(12 * 60)

--@ chunk 5
passage(3)
wait(8 * 60)

--@ chunk 6
passage(2)
wait(3 * 60)

--@ chunk 7
passage(1)
print(wait(60))

--@ chunk 8
for i = 1, 5 do lift(i) end
print(drying(115, 140), drying(115, 365), drying(875, 140), drying(875, 365))
