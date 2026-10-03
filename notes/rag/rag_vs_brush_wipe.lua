--@ engine 3

--@ chunk
canvas{size=440, aspect=2, linen=15, seed=4, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=40, apply="brush"}}}
--@ chunk
sky = pile{{"lead white", 3}, {"smalt", 2}, {"Prussian blue", 0.25}}
work(rect(0, 0, 1000, 500), {hand="broad", pile=sky, angle=0, coverage=3, fill=true})
print(wait(0), drying(250, 250), drying(750, 250))
--@ chunk
cloudL = ellipse(250, 250, 170, 95):roughen(30, 70, 3, 8)
cloudR = ellipse(750, 250, 170, 95):roughen(30, 70, 3, 8)
r = rag()
r:wipe(cloudL, {pressure=0.6, angle=0.15, passes=2, refold=0.5})
print("rag", r, r.load, wait(0))
--@ chunk
local b = brush{kind="flat", width=16, stiffness=0.6}
local n = 0
for i = 1, 900 do
  local x, y = rand(560, 940), rand(140, 360)
  if cloudR:at(x, y) > 0.5 then
    b:wipe(1)
    b:stroke({{x - 30, y}, {x + 30, y}}, {pressure={0.7, 0.6}, orient="across"})
    n = n + 1
  end
end
print("brush wipes", n, wait(0))
