-- Broken colour: one flat mixture / five close piles as strokes / the same blended
local flat = pile{{"lead white",3},{"cobalt blue",1},{"carmine lake",0.3},{"viridian",0.2}}
-- five neighbors of the flat mixture, about its value: bluer, violet, greener, rosier, warmer
local close = {
  pile{{"lead white",2.6},{"cobalt blue",1.3},{"carmine lake",0.1}},
  pile{{"lead white",2.2},{"cobalt violet",1.2},{"cobalt blue",0.3}},
  pile{{"lead white",2.6},{"cobalt blue",0.7},{"viridian",0.45}},
  pile{{"lead white",2.6},{"cobalt blue",0.6},{"carmine lake",0.55}},
  pile{{"lead white",2.8},{"cobalt blue",0.7},{"barium yellow",0.35},{"carmine lake",0.15}},
}
local cell = function(i) return rect(30 + i * 320, 40, 290, 580) end
work(cell(0), {hand="body", pile=flat, coverage=2.2, angle=0.05, length={30,70}, tool="filbert 10", fill=true, clip=true})
for k = 1, 2 do
  for _, p in ipairs(close) do
    work(cell(k), {hand="body", pile=p, coverage=0.55, angle=0.05, length={18,45}, tool="filbert 8", mix_jitter=0.3, streak=0.4, clip=true})
  end
end
blend(cell(2), {angle=0.05})
print(wait(60))
