--@ chunk 1 · clock 0
canvas{style="friedrich_early", size=440, aspect=1.4, seed=7}
--@ chunk 2 · clock 0
grades = {"2H", "HB", "2B", "4B", "chalk"}
for i, g in ipairs(grades) do
  for j, pr in ipairs({0.15, 0.35, 0.6, 0.9}) do
    local p = (g == "chalk") and chalk() or pencil(g)
    local x0, y = 60 + (j - 1) * 230, 60 + (i - 1) * 40
    for k = 0, 2 do
      p:line({{x0, y + k * 6}, {x0 + 90, y + k * 6 - 3}, {x0 + 190, y + k * 6 + 2}}, {pressure=pr})
    end
    p:sketch({{x0, y + 24}, {x0 + 190, y + 26}}, {pressure=pr})
  end
end
local h = pencil("2B")
h:hatch(rect(60, 300, 190, 80), {pressure=0.5})
h:hatch(rect(290, 300, 190, 80), {pressure=0.5})
h:hatch(rect(290, 300, 190, 80), {pressure=0.5, angle=0.7})
chalk():hatch(rect(520, 300, 190, 80), {pressure=0.4, angle=-1.2})
local e = pencil("4B")
for k = 0, 12 do e:line({{750, 300 + k * 3}, {940, 300 + k * 3}}, {pressure=0.7}) end
erase(rect(800, 290, 80, 100), {strength=0.9})
erase({{760, 330}, {930, 330}}, {strength=0.9, width=8})
