-- the same marks on this canvas's ground: a thin wash, body paint, a dark band, a medium-rich glaze-like band
local Wash = pile{{"cobalt blue",1},{"carmine lake",0.5},{"lead white",0.5}, turps=0.6}
local Body = pile{{"lead white",3},{"cadmium yellow",1},{"vermilion",0.2}}
local Dark = pile{{"ultramarine blue",2},{"viridian",1},{"carmine lake",0.5}}
local Rich = pile({{"viridian",1},{"lead white",0.3}, medium=0.5})
local band = function(i) return rect(40, 40 + i*150, 920, 120) end
-- The printed clock is rounded to a minute; stage checks sample every 30 minutes.
local function hours()
  local day, hour, minute = wait(0):match("day (%d+), (%d+):(%d+)")
  return (tonumber(day)-1)*24 + tonumber(hour) + tonumber(minute)/60
end
local laid = {}
work(band(0), {hand="broad", pile=Wash, coverage=1.0, angle=0.03, load=0.5})
laid[0] = hours()
work(band(1), {hand="body", pile=Body, coverage=1.8, angle=0.03, length={25,60}, tool="filbert 10"})
laid[1] = hours()
work(band(2), {hand="body", pile=Dark, coverage=1.8, angle=0.03, length={25,60}, tool="filbert 10", fill=true})
laid[2] = hours()
work(band(3), {hand="body", pile=Rich, coverage=1.4, angle=0.03, length={30,70}, tool="filbert 12"})
laid[3] = hours()
local first = {}
for step = 1, 96 do
  wait(30)
  for i = 0, 3 do local s = drying(500, 100 + i*150); first[i] = first[i] or {}; if not first[i][s] then first[i][s] = hours() - laid[i] end end
end
for i = 0, 3 do print(({"wash","body","dark","rich"})[i+1], "setting first sampled", first[i].setting or "not sampled", "h after laying, tacky first sampled", first[i].tacky or "not sampled", "h after laying (30-minute samples over 48 hours)") end
print(wait(10*24*60))
