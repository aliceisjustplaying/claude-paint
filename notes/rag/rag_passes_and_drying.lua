--@ engine 3

--@ chunk
canvas{size=440, aspect=1.5, linen=15, seed=4, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=40, apply="brush"}}}
sky = pile{{"lead white", 3}, {"smalt", 2}, {"Prussian blue", 0.25}}
-- the middles of the three test patches in each band
xs = {190, 500, 810}
function states(y) local t = {} for i, x in ipairs(xs) do t[i] = drying(x, y) end return t end
function all(y, ok) for _, s in ipairs(states(y)) do if not ok[s] then return false end end return true end
--@ chunk
work(rect(0, 40, 1000, 160), {hand="broad", pile=sky, angle=0, coverage=3, fill=true})
-- wait until band 1 is past its gel point everywhere the rag will go
local n = 0
repeat wait(5); n = n + 1 until all(120, {tacky=true, dry=true}) or n > 20000
--@ chunk
work(rect(0, 255, 1000, 160), {hand="broad", pile=sky, angle=0, coverage=3, fill=true})
-- wait until band 2 is setting, short of its gel point (paint is uneven: stop
-- when most of it sets, or as soon as any of it passes the gel point)
local function count(y, s) local k = 0 for _, v in ipairs(states(y)) do if v == s then k = k + 1 end end return k end
local n = 0
repeat wait(5); n = n + 1 until count(335, "setting") >= 2 or count(335, "tacky") + count(335, "dry") > 0 or n > 20000
--@ chunk
work(rect(0, 470, 1000, 160), {hand="broad", pile=sky, angle=0, coverage=3, fill=true})
-- the state of each patch as the rag goes over it
for b, y in ipairs({120, 335, 550}) do print("STATE", b, table.concat(states(y), " ")) end
--@ chunk
-- columns: 1, 2 and 3 passes of the rag, a clean face for each pass
for c = 1, 3 do
  local x = 90 + (c - 1) * 310
  for _, y in ipairs({50, 265, 480}) do
    local r = rag()
    for k = 1, c do
      if k > 1 then r:refold() end
      r:wipe(rect(x, y, 200, 140), {pressure=0.6, angle=0, refold=0.5})
    end
  end
end
