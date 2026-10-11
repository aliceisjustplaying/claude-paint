paper{kind="cold-press",size=16,aspect=2/3,cell=0.05,seed=71}
local prewet=brush{name="prewet",width=2.4}
prewet:load{water=8.4}
prewet:wash{x=1,y=7.5,width=14,height=5}
wait(24)
assert(balance().max_surface_depth_mm < 1e-10)
assert(balance().pore_water_mm3 > 0)
local b=brush{name="wash",width=2.4}
b:load{water=4.032,pigments={ultramarine=0.008*33.6}}
b:wash{x=1,y=8.8,width=14,height=2.4,grade=0.85}
wait(200)
assert(balance().conserved)
assert(balance().dry)
look{path="swatches-gallery.png",mode="gallery",width=320}
look{path="swatches-color.png",mode="color",width=320}
