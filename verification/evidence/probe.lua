-- easel session "probe": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 3

--@ chunk 1
canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; b=brush("round",4); b:load(p,0.8); b:stroke({{100,100},{300,80}}); print(W,H,b:fullness())

--@ chunk 2
print(marker == nil, b:fullness())

--@ chunk 3
m=rect(400,40,80,80); work(m,{hand="detail",pile=p,coverage=0.3,clip=true}); r=rag{width=20}; r:dip(0.5); r:blot(440,80); h=pencil("HB"); h:line({{600,100},{700,80}},{pressure=0.4}); print(r.load,r.damp,h:width(),drying(440,80))
