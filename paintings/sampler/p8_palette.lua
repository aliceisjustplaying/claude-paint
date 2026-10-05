-- The giverny box: each tube, then with white
local names = tubes()
for i, n in ipairs(names) do
  local col, row = (i-1) % 8, (i-1) // 8
  local x, y = 30 + col*120, 30 + row*330
  work(rect(x, y, 105, 140), {hand="body", pile=pile{{n, 1}}, coverage=2.2, angle=0, fill=true, tool="filbert 8"})
  if n ~= "lead white" and n ~= "zinc white" then
    work(rect(x, y + 155, 105, 140), {hand="body", pile=pile{{"lead white",3},{n, 1}}, coverage=2.2, angle=0, fill=true, tool="filbert 8"})
  end
end
print(wait(60))
