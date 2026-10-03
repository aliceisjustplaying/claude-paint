canvas{size=600, aspect=1.5, linen={16,17}, seed=77, ground={{pile={{"lead white",12},{"yellow ochre",0.25}}, um=130, apply="brush"}}}
horizon = 380
poplars = {}
treesM = nil
for i, x in ipairs({118, 160, 196, 612, 650, 695, 735, 782}) do
  poplars[#poplars+1] = {x = x + rand(-8, 8), h = 200 + 55 * math.sin(i * 1.7) + (i % 3) * 18, w = 15 + rand(0, 7)}
end
local function tree(p) return poly({{p.x - p.w, horizon + 4}, {p.x - p.w*0.85, horizon - p.h*0.5}, {p.x - p.w*0.4, horizon - p.h*0.88}, {p.x, horizon - p.h}, {p.x + p.w*0.45, horizon - p.h*0.86}, {p.x + p.w*0.9, horizon - p.h*0.48}, {p.x + p.w, horizon + 4}}, true) end
for _, p in ipairs(poplars) do local t = tree(p); treesM = treesM and (treesM + t) or t end
treesM = treesM:roughen(4, 18, 3)
bankM = rect(0, horizon - 18, 1000, 28):roughen(5, 40, 5)
sky = above(function(x) return horizon - 6 end) - treesM - bankM
water = below(function(x) return horizon + 8 end)
-- reflections: under each poplar, a column fading downward, broken by ripples
reflM = mask(function(x, y)
  if y < horizon + 8 then return 0 end
  local v = 0
  for _, p in ipairs(poplars) do
    local d = math.abs(x - p.x) / (p.w * 0.9)
    local depth = (y - horizon) / (p.h * 0.8)
    if d < 1 and depth < 1 then v = math.max(v, (1 - d * d) * (1 - depth) ) end
  end
  return v
end)
-- lay-in: lean, with turpentine
work(sky, {hand="broad", pile=pile{{"lead white",2},{"carmine lake",0.35},{"barium yellow",0.35}, turps=0.6}, coverage=1, angle=0})
work(water, {hand="broad", pile=pile{{"lead white",1.2},{"cobalt blue",1},{"carmine lake",0.35}, turps=0.6}, coverage=1, angle=0})
work(treesM + bankM, {hand="body", pile=pile{{"ultramarine blue",1},{"viridian",0.8},{"carmine lake",0.5},{"lead white",0.3}, turps=0.6}, coverage=1.2, angle=math.pi/2, tool="filbert 8", length={30,80}})
print(wait(24*60))
