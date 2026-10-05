-- Impasto: fat / tube / blotted paint, laid full; then fine sable vs coarse hog
local base = {{"lead white",4},{"cadmium yellow",1},{"deep cadmium",0.3}}
local function P(o) local t = {table.unpack(base)}; for k, v in pairs(o) do t[k] = v end; return pile(t) end
local variants = { P{medium=0.4}, P{}, P{blot=0.4} }
for i, p in ipairs(variants) do
  local x = 40 + (i-1) * 320
  local b = brush{kind="flat", width=16, stiffness=0.85, lay=8}
  for j = 0, 2 do b:reload(p, 1.0); b:stroke({{x, 60 + j*55}, {x + 130, 50 + j*55}, {x + 270, 64 + j*55}}, {pressure={1.0, 0.6}}) end
  work(rect(x, 240, 270, 170), {hand="body", pile=p, coverage=2, angle=0.35, length={20,50}, load=1.0, tool={kind="filbert", width=11, lay=5}})
end
local Wb = pile({{"lead white",5},{"cobalt blue",0.6}, blot=0.35})
local fine = brush{kind="round", width=10, stiffness=0.4, hair=0.03, lay=6}
local coarse = brush{kind="flat", width=14, stiffness=0.9, hair=0.12, lay=6}
for j = 0, 3 do fine:reload(Wb, 1.0); fine:stroke({{40, 460 + j*40}, {450, 450 + j*40}}, {pressure={1.0, 0.6}}) end
for j = 0, 3 do coarse:reload(Wb, 1.0); coarse:stroke({{520, 460 + j*40}, {960, 450 + j*40}}, {pressure={1.0, 0.6}}) end
print(wait(4*24*60))
