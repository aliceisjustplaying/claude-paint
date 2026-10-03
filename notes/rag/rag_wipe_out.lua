--@ engine 3

--@ chunk
canvas{size=440, aspect=1.5, linen=15, seed=6, ground={{pile={{"lead white", 3}, {"yellow ochre", 0.2}}, um=40, apply="brush"}}}
--@ chunk
-- the toning layer: raw umber, thin (oil medium stands in for the handouts' spirits)
tone = pile{{"raw umber", 1}, medium=0.5}
work(rect(0, 0, 1000, 667), {hand="broad", pile=tone, angle=0, coverage=1.5, load=0.5, fill=true})
-- "wipe the excess paint off the canvas lightly with a rag to leave an even tone" (Downing-White)
t = rag{width=140}
t:wipe(everywhere(), {pressure=0.15, angle=0, refold=0.6})
--@ chunk
-- three test panels: the tone as laid; a clean dry rag, three times; the rag dipped in
-- spirits, three times (dipped again each time a cleaner part is turned out), then a dry part.
-- Wiped by hand in rows, a cleaner part turned out whenever the one in use is half loaded.
function rows(r, x0, y0, w, h, times, dip)
  if dip then r:dip(dip) end
  for k = 1, times do
    local back = k % 2 == 0
    for y = y0 + 30, y0 + h - 25, 50 do
      if r.load > 0.5 then r:refold(); if dip then r:dip(dip) end end
      local a, b = x0 + 35, x0 + w - 35
      if back then a, b = b, a end
      r:wipe({{a, y + rand(-4, 4)}, {(a + b) / 2, y + rand(-6, 6)}, {b, y + rand(-4, 4)}}, {pressure=0.8})
    end
  end
end
r = rag()
rows(r, 380, 40, 240, 180, 3)
s = rag()
rows(s, 700, 40, 240, 180, 3, 0.5)
s:refold()
rows(s, 700, 40, 240, 180, 1)
--@ chunk
-- a wipe-out: a ball lit from the upper left on a table; shadows left alone,
-- the half-lights wiped dry, the lights with spirits
local ball = ellipse(420, 450, 150, 150)
local lit = ball * ellipse(370, 400, 125, 120)
local light = ball * ellipse(345, 375, 60, 55)
local table_top = rect(0, 560, 1000, 107) - ellipse(520, 600, 210, 45)
q = rag()
q:wipe(lit, {pressure=0.6, angle=-0.7, passes=2, refold=0.5})
q:wipe(table_top, {pressure=0.5, angle=0, refold=0.5})
q:refold()
q:dip(0.6)
q:wipe(light, {pressure=0.8, angle=-0.7, passes=2})
q:refold()
q:blot(330, 362, {pressure=0.7})
print(wait(0), q)
