--@ engine 3
--@ chunk 1
canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}
p=pile{{"red earth",1}}
stipple(rect(300,40,200,100),{pile=p,width=2,coverage=1,seed=7})
print(wait(0))
