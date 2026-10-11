paper{kind="cold-press", size=36, aspect=1.5, seed=71}
sky=brush{name="sky", width=6}
sky:load{water=65, pigments={ultramarine=1.3}}
sky:wash{x=0, y=0, width=36, height=14, grade=0.15}
wait(0.5)
distance=brush{name="distance", width=4}
distance:load{water=20, pigments={ultramarine=0.23, earth=0.065, phthalo=0.015}}
distance:stroke{{0,14},{6,11.5},{10,13},{17,10},{23,12.5},{29,11},{36,14}}
wait(1)
drop{x=19.2,y=12.5,water=3.2,radius=0.7}
look{path="bloom-start.png",width=1200}
frames{prefix="bloom",every=0.25,width=480}
wait(8)
frames(false)
look{path="bloom-end.png",width=1200}
save("bloom.json")
wait(300)
save("wash-layer.json")
assert(balance().conserved)
