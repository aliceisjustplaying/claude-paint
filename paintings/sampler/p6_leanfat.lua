-- Lean and fat, matte and gloss: one dark paint four ways, left to dry
local base = {{"ultramarine blue",2},{"viridian",1},{"carmine lake",0.5}}
local function P(o) local t = {table.unpack(base)}; for k, v in pairs(o) do t[k] = v end; return pile(t) end
local ways = { P{turps=0.7}, P{blot=0.4}, P{}, P{medium=0.5} }
for i, p in ipairs(ways) do
  work(rect(30 + (i-1)*240, 40, 220, 360), {hand="body", pile=p, coverage=2, angle=0.1, length={30,70}, load=0.9, tool="filbert 10", fill=true})
end
-- a lean turpentine lay-in sketch, and body paint over part of it
local wash = pile({{"ultramarine blue",1},{"carmine lake",0.6}, turps=0.75})
work(ellipse(260, 530, 220, 90), {hand="broad", pile=wash, coverage=1.2, angle=0.2})
work(ellipse(700, 530, 220, 90), {hand="broad", pile=wash, coverage=1.2, angle=0.2})
print(wait(2*60))
work(ellipse(700, 520, 150, 55), {hand="body", pile=pile{{"lead white",3},{"cadmium yellow",1},{"vermilion",0.2}}, coverage=1.8, angle=0.2, length={15,40}, tool="filbert 8"})
print(wait(20*24*60))
