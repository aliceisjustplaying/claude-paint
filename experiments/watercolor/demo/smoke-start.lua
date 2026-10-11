paper{solver="current", kind="cold-press", size=36, aspect=1.5, seed=71}
sky=brush{name="sky", width=6}
sky:load{water=65, pigments={ultramarine=1.3}}
sky:wash{x=0, y=0, width=36, height=14, grade=0.15}
wait(0.5)
distance=brush{name="distance", width=4}
distance:load{water=20, pigments={ultramarine=0.23, earth=0.065, phthalo=0.015}}
local ridge={}
for i=0,36 do ridge[#ridge+1]={i,12.5+0.8*math.sin(i/3.5)-1.8*math.exp(-((i-18)/6)^2)} end
distance:stroke(ridge)
wait(1)
drop{x=19.2,y=12.5,water=3.2,radius=0.7}
look{path="bloom-start.png",width=1200,mode="gallery"}
frames{prefix="bloom",every=0.25,width=480}
wait(8)
frames(false)
look{path="bloom-end.png",width=1200,mode="gallery"}
save("bloom.json")
wait(300)
save("wash-layer.json")
assert(balance().conserved)
