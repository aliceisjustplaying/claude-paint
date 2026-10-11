--@ engine 5

--@ chunk 1
canvas{size=160, aspect=1.5, linen=16, seed=1906, ground={{pile={{"lead white",3},{"raw umber",1}}, um=90, apply="knife"}}}
wall = pile{{"bone black",5},{"cobalt blue",2},{"raw umber",1}, medium=0.08}
tabletop = pile{{"red earth",3},{"yellow ochre",2},{"raw umber",2}, medium=0.1}
shadow = pile{{"bone black",3},{"raw umber",2}, medium=0.05}
body = pile{{"lead white",6},{"yellow ochre",1},{"red earth",0.2}, medium=0.02}
light = pile{{"lead white",8},{"yellow ochre",2}, medium=0.02}

--@ chunk 2
w = brush("flat",210)
w:load(wall,0.95)
w:stroke({{40,100},{960,108}}, {pressure={0.85,0.8}})
w:load(wall,0.95)
w:stroke({{950,220},{45,212}}, {pressure={0.8,0.85}})
w:load(wall,0.95)
w:stroke({{45,340},{950,350}}, {pressure={0.85,0.8}})
w:load(wall,0.95)
w:stroke({{960,460},{40,450}}, {pressure={0.8,0.85}})
w:load(wall,0.95)
w:stroke({{40,580},{950,590}}, {pressure={0.85,0.75}})

--@ chunk 3
t = brush("flat",110)
t:load(tabletop,0.9)
t:stroke({{60,570},{950,578}}, {pressure={0.8,0.75}})
t:load(tabletop,0.65)
t:stroke({{940,627},{70,617}}, {pressure={0.75,0.65}})
s = brush("filbert",90)
s:load(shadow,0.65)
s:stroke({{250,415},{235,490},{285,555}}, {pressure={0.55,0.8}})
s:stroke({{225,563},{340,574},{465,583}}, {pressure={0.8,0.25}})
b = brush("filbert",80)
b:load(body,0.95)
b:stroke({{315,401},{330,460},{326,535}}, {pressure={0.55,0.8}})
b:load(body,0.8)
b:stroke({{358,420},{390,480},{365,545}}, {pressure={0.4,0.75}})
b:stroke({{292,544},{337,560},{373,548}}, {pressure={0.6,0.25}})

--@ chunk 4
l = brush("flat",75)
l:load(light,0.9)
l:stroke({{660,145},{665,325}}, {pressure={0.8,0.75}})
l:load(light,0.9)
l:stroke({{702,148},{707,330}}, {pressure={0.8,0.75}})
l:load(light,0.9)
l:stroke({{744,153},{747,326}}, {pressure={0.75,0.7}})
soft = brush("blender",70)
soft:stroke({{633,132},{630,345}}, {pressure={0.4,0.25}})
soft:stroke({{781,345},{779,135}}, {pressure={0.35,0.25}})
d = brush("flat",35)
d:load(shadow,0.14)
d:stroke({{690,148},{693,335}}, {pressure={0.65,0.5}})
