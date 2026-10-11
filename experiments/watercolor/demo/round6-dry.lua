paper{kind="cold-press",size=16,aspect=2/3,cell=0.05,seed=71}
wait(24)
local b=brush{name="wash",width=2.4}
b:load{water=4.032,pigments={ultramarine=0.008*33.6}}
b:wash{x=1,y=8.8,width=14,height=2.4,grade=0.85}
wait(200)
assert(balance().conserved)
assert(balance().dry)
look{path="swatches-gallery.png",mode="gallery",width=320}
look{path="swatches-color.png",mode="color",width=320}
