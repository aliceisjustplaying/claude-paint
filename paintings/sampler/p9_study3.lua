-- the light: broken warm touches on the poplars' sunward edges, thick lights low in the sky
local warmLeaf = pile({{"cadmium yellow",0.4},{"viridian",0.5},{"yellow ochre",0.4},{"carmine lake",0.1},{"lead white",1.2}, blot=0.2})
local d = brush{kind="filbert", width=3, stiffness=0.8, lay=3}
for _, p in ipairs(poplars) do
  for k = 1, 10 do
    local t = rand(0.08, 0.9)
    local y = horizon - p.h * t
    local halfw = p.w * (1 - t * 0.85)
    local x = p.x - halfw * rand(0.35, 0.95)
    d:reload(warmLeaf, 0.9)
    d:stroke({{x, y}, {x + rand(-1, 1), y + rand(8, 16)}}, {pressure={0.7, 0.2}})
  end
end
local sunL = pile({{"lead white",5},{"barium yellow",1},{"vermilion",0.06}, blot=0.3})
local b = brush{kind="filbert", width=6, stiffness=0.85, lay=8}
for i = 1, 30 do b:reload(sunL, 1.0); local x, y = randn(410, 55), horizon - rand(14, 60); b:stroke({{x, y}, {x + rand(12, 26), y + rand(-1.5, 1.5)}}, {pressure={1.0, 0.35}}) end
-- the sun's path on the water: thick horizontal dabs, narrowing toward us
for i = 1, 70 do
  local y = horizon + 14 + rand(0, 1) ^ 1.6 * 300
  local spread = 40 + (y - horizon) * 0.18
  local x = 430 + randn(0, spread * 0.45)
  b:reload(sunL, 1.0)
  b:stroke({{x, y}, {x + rand(10, 26), y + rand(-1, 1)}}, {pressure={0.95, 0.3}})
end
-- a glaze deepens the far bank; a thin warm veil of haze along the horizon
work(bankM:grow(3), {hand="glaze", pile=pile({{"ultramarine blue",0.7},{"carmine lake",0.25},{"viridian",0.35}, medium=0.85}), coverage=0.6, angle=0, clip=bankM:grow(3)})
local hazeM = sky * rect(0, horizon - 70, 1000, 75):soften(30)
work(hazeM, {hand="glaze", pile=pile({{"lead white",1},{"carmine lake",0.08},{"barium yellow",0.06}, medium=0.7}), coverage=0.6, angle=0, clip=sky})
print(wait(30*24*60))
