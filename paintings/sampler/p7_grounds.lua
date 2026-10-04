-- the same marks on this canvas's ground: a thin wash, body paint, a dark band, a medium-rich glaze-like band
local Wash = pile{{"cobalt blue",1},{"carmine lake",0.5},{"lead white",0.5}, turps=0.6}
local Body = pile{{"lead white",3},{"cadmium yellow",1},{"vermilion",0.2}}
local Dark = pile{{"ultramarine blue",2},{"viridian",1},{"carmine lake",0.5}}
local Rich = pile({{"viridian",1},{"lead white",0.3}, medium=0.5})
local band = function(i) return rect(40, 40 + i*150, 920, 120) end
-- the painting's clock in hours (wait(0) gives the time of day, "day 1, 09:00"); painting a band takes time
local function hours() local d, h, m = wait(0):match("day (%d+), (%d+):(%d+)"); return ((tonumber(d) * 24 + tonumber(h)) * 60 + tonumber(m)) / 60 end
local laid = {}
work(band(0), {hand="broad", pile=Wash, coverage=1.0, angle=0.03, load=0.5}); laid[0] = hours()
work(band(1), {hand="body", pile=Body, coverage=1.8, angle=0.03, length={25,60}, tool="filbert 10"}); laid[1] = hours()
work(band(2), {hand="body", pile=Dark, coverage=1.8, angle=0.03, length={25,60}, tool="filbert 10", fill=true}); laid[2] = hours()
work(band(3), {hand="body", pile=Rich, coverage=1.4, angle=0.03, length={30,70}, tool="filbert 12"}); laid[3] = hours()
-- each stage as first seen, in hours after that band was laid
local first, last = {}, laid[3]
for _ = 1, 96 do
  wait(30)
  last = hours()
  for i = 0, 3 do local s = drying(500, 100 + i*150); first[i] = first[i] or {}; if not first[i][s] then first[i][s] = last - laid[i] end end
end
for i = 0, 3 do
  local at = function(h) return h and string.format("%.1f h", h) or string.format(">%.0f h", last - laid[i]) end
  print(({"wash","body","dark","rich"})[i+1], "setting at", at(first[i].setting), ", tacky at", at(first[i].tacky))
end
print(wait(10*24*60))
