paper{kind="cold-press",size=16,aspect=2/3,cell=0.05,seed=71}
local samples={
    {"phthalo",0.0012},
    {"ultramarine",0.0007},
    {"ultramarine",0.008},
    {"earth",0.0018}
}
for row,sample in ipairs(samples) do
    local b=brush{name="wash-"..row,width=2.4}
    b:load{water=4.032,pigments={[sample[1]]=sample[2]*33.6}}
    b:wash{x=1,y=(row-1)*4+0.8,width=14,height=2.4,grade=0.85}
end
local blue=brush{name="join-blue",width=2.4}
blue:load{water=2.304,pigments={ultramarine=0.0015*19.2}}
blue:wash{x=1,y=16.8,width=8,height=2.4,grade=0.85}
local earth=brush{name="join-earth",width=2.4}
earth:load{water=2.304,pigments={earth=0.0015*19.2}}
earth:wash{x=7,y=16.8,width=8,height=2.4,grade=0.85}
wait(200)
assert(balance().conserved)
assert(balance().dry)
local dry=brush{name="dry-control",width=1.8}
dry:load{pigments={earth=0.05,ultramarine=0.04}}
dry:stroke({{1.4,22},{6,21.8},{10,22.15},{14.4,21.9}},{dry=true,pressure=0.28})
assert(balance().conserved)
look{path="swatches-gallery.png",mode="gallery",width=320}
look{path="swatches-color.png",mode="color",width=320}
