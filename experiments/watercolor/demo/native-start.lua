paper{kind="cold-press",size=60,aspect=1.5,cell=0.05,seed=71}
sky=brush{name="sky",width=10}
sky:load{water=90,pigments={ultramarine=1.8}}
sky:load{water=90,pigments={ultramarine=1.8}}
sky:wash{x=0,y=0,width=60,height=23.3333333333,grade=0.15}
wait(0.5)
distance=brush{name="distance",width=6.6666666667}
distance:load{water=55.5555555556,pigments={ultramarine=0.6388888889,earth=0.1805555556,phthalo=0.0416666667}}
local ridge={}
for i=0,60 do
    local u=i*0.6
    ridge[#ridge+1]={i,(12.5+0.8*math.sin(u/3.5)-1.8*math.exp(-((u-18)/6)^2))/0.6}
end
distance:stroke(ridge)
wait(1)
drop{x=32,y=20.8333333333,water=8.8888888889,radius=1.1666666667}
look{path="native-bloom-start.png",width=1200,mode="gallery"}
frames{prefix="native-bloom",every=0.25,width=480}
wait(8)
frames(false)
look{path="native-bloom-end.png",width=1200,mode="gallery"}
assert(balance().conserved)
