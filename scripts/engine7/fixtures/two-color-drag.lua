--@ engine 5
--@ box tube box
--@ chunk 1
canvas{size=440, aspect=2.5, linen={18,16}, seed=652,
  ground={{pile={{"lead white",10},{"yellow ochre",0.2}}, um=100, apply="knife"}}}
red = pile{{"vermilion",3},{"lead white",1}}
blue = pile{{"cobalt blue",3},{"lead white",1}}
--@ chunk 2
b = brush("flat",28)
for _,y in ipairs{100,200,300} do
  b:wipe(1)
  b:load(red,0.9,{side=-1,share=0.5})
  b:load(blue,0.9,{side=1,share=0.5})
  b:stroke({{100,y},{470,y-35},{880,y+15}}, {pressure=0.65, ramps={0.05,0.1}, orient="across"})
end
print(b:fullness(), drying(500,200))
