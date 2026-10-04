# Space runtime evidence

Agent-driven default release CLI probes at source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. Disposable EASEL_ROOT, aspect5 canvas. Pixel comparisons use logical coordinates times2.4.

## Empty session

```text
open space
```

Exit 0; wall 0.115 seconds:

```text
easel "space" open: 0 chunks · 2400px · no canvas yet

```

## World requires canvas

```text
do world{}
```

Exit 1; wall 0.004 seconds:

```text
runtime error: no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}
(the chunk failed and changed nothing)

```

## Form requires canvas

```text
do form{{body.ellipsoid({100,100,0},{10,10,10})}}
```

Exit 1; wall 0.003 seconds:

```text
runtime error: no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}
(the chunk failed and changed nothing)

```

## Canvas

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}};p=pile{{"red earth",1}};r=rag{width=20};r:dip(1);print(wait(0),r.damp)
```

Exit 0; wall 0.088 seconds:

```text
day 1, 09:00	1.0
ok · chunk 1 (0.07 s to compute)

```

## Save reference-before

```text
save <isolated-root>/reference-before.png
```

Exit 0; wall 0.028 seconds:

```text
<isolated-root>/reference-before.png

```

## Missing default view

```text
do work(rect(400,60,160,80),{pile=p,visible="ground"})
```

Exit 1; wall 0.027 seconds:

```text
runtime error: visible=, behind= and at= need a world view: v = w:view() first (the last view made is used), or pass view=v
(the chunk failed and changed nothing)

```

## Bodies

```text
do s=body.ellipsoid({400,100,0},{40,30,20}); k=body.block({500,100,0},{40,30,20},3); h=body.half_space({400,100,0},{0,-1,0}); turned=s:turn({400,100,0},0.3,0.1,0); cut=s:cut({400,100,0},{0,-1,0},1,3); rough=s:rough(2,20,1); joined=s:union(k); removed=s:subtract(k); print(s:bounds(),turned:bounds(),cut:bounds(),rough:bounds(),joined:bounds(),removed:bounds())
```

Exit 0; wall 0.024 seconds:

```text
table: (hidden)	table: (hidden)	table: (hidden)	table: (hidden)	table: (hidden)	table: (hidden)
ok · chunk 2 (0.00 s to compute)

```

## Forms and light

```text
do f=form{{s},{k},light={from={-1,-0.7},front=0.5,ambient=0.2}}; f2=form{{s},{k},light={from={1,0.7},front=0.1,ambient=0}};print("parts",f:part(400,100),f:part(500,100));print("light",f:value(380,90),f2:value(380,90),f:value(380,90));print("queries",f:sample(400,100),f:shade(400,100),f:dist(400,100),f:fall(400,100),f:across(400,100),f:bend(400,100),f:edge_angle(400,100));print("masks",f:parts_mask{1}:area(),f:lit():area(),f:shadow():area(),f:silhouette():area(),f:edges():area()); print("clamp",f:mask(function()return -1 end):at(400,100),f:mask(function()return 2 end):at(400,100))
```

Exit 0; wall 0.167 seconds:

```text
parts	1	2
light	0.684252142906189	0.058396779000759125	0.684252142906189
queries	table: (hidden)	table: (hidden)	0.0	1.5708084106445312	1.2040138244628906e-05	0.11133292317390442	3.1415653228759766
masks	3769.444144875933	3930.1851910406981	1032.3144145188351	4962.4996056159571	397.95708642976035
clamp	0.0	1.0
ok · chunk 3 (0.15 s to compute)

```

## Terrain sampled

```text
do height=5; t=terrain{area={600,60,640,100},step=4,height=function() return height end};tf=form{{t}};height=20;t2=terrain{area={640,100,600,60},step=4,height=function()return height end};tf2=form{{t2}};print("old-new",tf:sample(620,80).z,tf2:sample(620,80).z,tf:sample(620,80).z)
```

Exit 0; wall 0.02 seconds:

```text
old-new	5.0	20.0	5.0
ok · chunk 4 (0.00 s to compute)

```

## World camera and projection

```text
do w=world{view={300,20,400,160},horizon=80,eye=1.6,fov=45}; v0=w:view(); print(w.eye,w.horizon,w.bodies);for _,point in ipairs{{0,0,10},{1,0,10},{0,1,10},{0,0,20},{100,0,10},{0,0,0},{0,0,-1}} do local q=w:project(table.unpack(point));print(table.unpack(point)); if q then print(q[1],q[2]) else print("no projection") end end;print("sky",w:spot(400,30)==nil,w:spot(200,120)==nil)
```

Exit 0; wall 0.02 seconds:

```text
1.6000000238418579	80.0	0
0	0	10
500.0	157.25483703613281
1	0	10
548.2843017578125	157.25483703613281
0	1	10
500.0	108.97056579589844
0	0	20
500.0	118.62741851806641
100	0	10
5328.4267578125	157.25483703613281
0	0	0
no projection
0	0	-1
no projection
sky	true	true
ok · chunk 5 (0.00 s to compute)

```

## Depth from sky rejects

```text
do v0:at_depth({400,30})
```

Exit 1; wall 0.015 seconds:

```text
runtime error: (400, 30) is not on the ground (sky or outside the view): give a depth in meters or a spot
(the chunk failed and changed nothing)

```

## World immutable ground

```text
do gheight=0;wg=world{view={300,20,400,160},ground=function()return gheight end};gheight=0.5;wg2=world{view={300,20,400,160},ground=function()return gheight end};print(wg:ground_at(0,10),wg2:ground_at(0,10),wg:ground_at(0,10))
```

Exit 0; wall 0.026 seconds:

```text
0.0	0.5	0.0
ok · chunk 6 (0.01 s to compute)

```

## Water spots and helpers

```text
do ww=world{view={300,20,400,160},horizon=80,ground=function()return -1 end,water={level=0,ripple=0.01}};sp=ww:spot_at(0,10);bed=ww:spot_bed(0,10);print("surface-bed",sp.at[2],bed.at[2],sp.y,bed.y,sp:m(1));local q=ww:project(table.unpack(sp.at));local back=ww:to_ground(q[1],q[2]);print("roundtrip",back[1],back[2],back[3]); print("helpers",#ww:line({{0,10},{0,20}}),ww:ribbon({{0,10},{0,20}},1):area(),ww:height(sp.x,sp.y,1),ww:scale_at(10),ww:scale_at(20),ww:ground_at(0,10),ww:is_water(0,10),ww:shadow_angle(sp.x,sp.y),ww:sun_canvas(),ww:aerial(10),ww:aerial(20));for _,a in ipairs(ww:recede({0,10},{0,5},3)) do print(a.y,a.s) end
```

Exit 1; wall 0.009 seconds:

```text
error converting Lua number to table
(the chunk failed and changed nothing)

```

## Placement and proxy

```text
do ws=ww:spot_at(0,12); solid=body.block(ws:p(0,0.5,0),ws:size(1,1,1),ws:m(0.05));wp,proxy=ww:proxy(ws,solid);wp,visible=wp:place(ww:spot_at(1.5,12),solid);vp=wp:view();print("counts",ww.bodies,wp.bodies,"parts",vp:part(proxy),vp:part(visible));print("areas",vp:bodies_mask(proxy):area(),vp:bodies_mask(visible):area(),vp:shadows():area(),vp:reflections(proxy):area(),vp:water():area(),vp:land():area(),vp:sky():area(),vp:contact():area()); print("mirror sky",vp:mirror(400,30)==nil)
```

Exit 1; wall 0.009 seconds:

```text
runtime error: [string "chunk 7"]:1: attempt to index a nil value (global 'ww')
(the chunk failed and changed nothing)

```

## Depth layer views

```text
do fg=rect(440,60,40,80);wA=w:layer("foreground",fg,3);vA=wA:view();wB=w:layer("other",rect(520,60,40,80),3);vB=wB:view(); print("old",v0:visible("ground"):area(),"A",vA:visible("foreground"):area()); print("seen",vA:seen(450,100)[1].layer,vA:seen(450,100)[1].share);print("depth",vA:front("foreground"):area(),vA:behind("foreground"):at(450,100),vA:at_depth(5):at(450,100),vA:between(2,5):area())
```

Exit 0; wall 0.035 seconds:

```text
old	39999.9968210858	A	3199.9997456868641
seen	foreground	1.0
depth	0.0	0.0	0.0	3199.9997456868641
ok · chunk 7 (0.02 s to compute)

```

## Reference time unchanged

```text
do print(wait(0),r.damp)
```

Exit 0; wall 0.017 seconds:

```text
day 1, 09:00	1.0
ok · chunk 8 (0.00 s to compute)

```

## Save reference-after

```text
save <isolated-root>/reference-after.png
```

Exit 0; wall 0.028 seconds:

```text
<isolated-root>/reference-after.png

```

PNG equality reference-before/reference-after: **true**.

## Explicit view

```text
do work(rect(420,80,80,40),{pile=p,hand="broad",coverage=2,visible="foreground",view=vA})
```

Exit 0; wall 0.059 seconds:

```text
ok · chunk 9 (0.04 s to compute)

```

## Save explicit

```text
save <isolated-root>/explicit.png
```

Exit 0; wall 0.027 seconds:

```text
<isolated-root>/explicit.png

```

Changed pixels reference-after→explicit: inside [440, 60, 480, 140] = **5649**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

## Default does not contain old layer

```text
do work(rect(420,80,80,40),{pile=p,visible="foreground"})
```

Exit 1; wall 0.016 seconds:

```text
runtime error: no layer "foreground" in this view's world (layers: other; also "ground", "water", "surface", "sky", "bodies", "layers" or a body number). Register it with w = w:layer("foreground", mask, depth), then v = w:view()
(the chunk failed and changed nothing)

```

## Failed view replacement

```text
do temp=vA; wA:view();error("restore default")
```

Exit 1; wall 0.014 seconds:

```text
runtime error: [string "chunk 10"]:1: restore default
(the chunk failed and changed nothing)

```

## Default restored

```text
do work(rect(420,80,80,40),{pile=p,visible="foreground"})
```

Exit 1; wall 0.01 seconds:

```text
runtime error: no layer "foreground" in this view's world (layers: other; also "ground", "water", "surface", "sky", "bodies", "layers" or a body number). Register it with w = w:layer("foreground", mask, depth), then v = w:view()
(the chunk failed and changed nothing)

```

## Default new layer works

```text
do work(rect(500,80,80,40),{pile=p,hand="broad",coverage=2,visible="other"})
```

Exit 0; wall 0.048 seconds:

```text
ok · chunk 10 (0.03 s to compute)

```

## Save implicit

```text
save <isolated-root>/implicit.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/implicit.png

```

Changed pixels explicit→implicit: inside [520, 60, 560, 140] = **7703**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

## Behind protects mask

```text
do work(rect(400,40,180,120),{pile=p,hand="broad",coverage=2,behind={fg},view=vA})
```

Exit 0; wall 0.614 seconds:

```text
ok · chunk 11 (0.59 s to compute)

```

## Save behind

```text
save <isolated-root>/behind.png
```

Exit 0; wall 0.035 seconds:

```text
<isolated-root>/behind.png

```

Changed pixels implicit→behind: inside [440, 60, 480, 140] = **0**, outside = **153622**. Pixel bounds use logical coordinates ×2.4.

## At and combined restrictions

```text
do work(rect(400,40,180,120),{pile=p,hand="broad",coverage=1,visible="ground",behind="foreground",at=5,view=vA})
```

Exit 0; wall 0.166 seconds:

```text
ok · chunk 12 (0.14 s to compute)

```

## Save combined

```text
save <isolated-root>/combined.png
```

Exit 0; wall 0.033 seconds:

```text
<isolated-root>/combined.png

```

Changed pixels behind→combined: inside [440, 60, 480, 140] = **0**, outside = **48602**. Pixel bounds use logical coordinates ×2.4.

## Reserved layer rejection

```text
do w:layer("ground",fg,3)
```

Exit 1; wall 0.011 seconds:

```text
runtime error: layer name "ground" is taken (it names a part of the world)
(the chunk failed and changed nothing)

```

## Direct mask front rejection

```text
do vA:front(fg)
```

Exit 1; wall 0.011 seconds:

```text
runtime error: want a body number, a layer name, "ground", "water", "sky" or a list, got userdata
(the chunk failed and changed nothing)

```

## Direct mask behind rejection

```text
do vA:behind(fg)
```

Exit 1; wall 0.011 seconds:

```text
runtime error: want a body number, a layer name, "ground", "water", "sky" or a list, got userdata
(the chunk failed and changed nothing)

```

## Replay references and paint

```text
check
```

Exit 0; wall 1.045 seconds:

```text
replay matches the live canvas exactly (12 chunks, 1.0s)

```

## Close

```text
close
```

Exit 0; wall 0.091 seconds:

```text
the live canvas is in <isolated-root>/out/easel/space/live.png
closed; the session is in <isolated-root>/paintings/lua/space.lua

```
# Space runtime evidence

Agent-driven default release CLI probes at source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. Disposable EASEL_ROOT, aspect5 canvas. Pixel comparisons use logical coordinates times2.4.

## Reopen

```text
open space
```

Exit 0; wall 1.082 seconds:

```text
resuming chunk 1/12
resumed chunk 1/12 0.08s
resuming chunk 2/12
resumed chunk 2/12 0.00s
resuming chunk 3/12
resumed chunk 3/12 0.15s
resuming chunk 4/12
resumed chunk 4/12 0.00s
resuming chunk 5/12
resumed chunk 5/12 0.00s
resuming chunk 6/12
resumed chunk 6/12 0.01s
resuming chunk 7/12
resumed chunk 7/12 0.02s
resuming chunk 8/12
resumed chunk 8/12 0.00s
resuming chunk 9/12
resumed chunk 9/12 0.04s
resuming chunk 10/12
resumed chunk 10/12 0.03s
resuming chunk 11/12
resumed chunk 11/12 0.59s
resuming chunk 12/12
resumed chunk 12/12 0.13s
resumed 12 chunks from <isolated-root>/paintings/lua/space.lua
easel "space" open: 12 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1

```

## Water spots and helpers

```text
do ww=world{view={300,20,400,160},horizon=80,ground=function()return -1 end,water={level=0,ripple={0.01,1.4,0.3,1}}};sp=ww:spot_at(0,10);bed=ww:spot_bed(0,10);print("surface-bed",sp.at[2],bed.at[2],sp.y,bed.y,sp:m(1));local q=ww:project(table.unpack(sp.at));local back=ww:to_ground(q[1],q[2]);print("roundtrip",back[1],back[2],back[3]); print("helpers",#ww:line({{0,10},{0,20}}),ww:ribbon({{0,10},{0,20}},1):area(),ww:height(sp.x,sp.y,1),ww:scale_at(10),ww:scale_at(20),ww:ground_at(0,10),ww:is_water(0,10),ww:shadow_angle(sp.x,sp.y),ww:sun_canvas(),ww:aerial(10),ww:aerial(20));for _,a in ipairs(ww:recede({0,10},{0,5},3)) do print(a.y,a.s) end
```

Exit 0; wall 0.03 seconds:

```text
surface-bed	0.0	-1.0	157.25483703613281	205.53909301757812	48.284271240234375
roundtrip	0.0	0.0	10.0
helpers	2	1400.1932447726078	48.284271240234375	48.284271240234375	24.142135620117188	-1.0	true	-0.092114485800266266	nil	0.0039920210838317871	0.00796806812286377
157.25483703613281	48.284271240234375
131.50321960449219	32.18951416015625
118.62741851806641	24.142135620117188
ok · chunk 13 (0.00 s to compute)

```

## Placement and proxy

```text
do ws=ww:spot_at(0,12); solid=body.block(ws:p(0,0.5,0),ws:size(1,1,1),ws:m(0.05));wp,proxy=ww:proxy(ws,solid);wp,visible=wp:place(ww:spot_at(1.5,12),solid);vp=wp:view();print("counts",ww.bodies,wp.bodies,"parts",vp:part(proxy),vp:part(visible));print("areas",vp:bodies_mask(proxy):area(),vp:bodies_mask(visible):area(),vp:shadows():area(),vp:reflections(proxy):area(),vp:water():area(),vp:land():area(),vp:sky():area(),vp:contact():area()); print("mirror sky",vp:mirror(400,30)==nil)
```

Exit 0; wall 0.367 seconds:

```text
counts	0	2	parts	0	1
areas	0.0	1613.1943162392763	402.81051856170882	1330.2082276178794	38386.802504846528	0.0	23999.99809265148	283.06948725698885
mirror sky	true
ok · chunk 14 (0.35 s to compute)

```

## Reference callback table

```text
do first=nil;copied=nil;f:mask(function(a)if first==nil then first=a;copied=a.z end;return 1 end);print(first.z,copied,first.z~=copied)
```

Exit 0; wall 0.082 seconds:

```text
0.86682093143463135	0.86658781766891479	true
ok · chunk 15 (0.06 s to compute)

```

## Reference callback sandbox

```text
do f:mask(function(a)assert(io==nil and os==nil and package==nil and require==nil);return 0 end);print("restricted callback")
```

Exit 0; wall 0.084 seconds:

```text
restricted callback
ok · chunk 16 (0.07 s to compute)

```

## Immutable body

```text
do original=form{{s}};derived=form{{s:cut({400,100,0},{0,-1,0})}};print(original:parts_mask{1}:area(),derived:parts_mask{1}:area(),original:parts_mask{1}:area())
```

Exit 0; wall 0.025 seconds:

```text
3769.444144875933	1886.8054056057313	3769.444144875933
ok · chunk 17 (0.01 s to compute)

```

## Old view without body

```text
do print(v0:bodies_mask():area(),vp:bodies_mask():area(),ww.bodies,wp.bodies)
```

Exit 0; wall 0.052 seconds:

```text
0.0	1613.1943162392763	0	2
ok · chunk 18 (0.04 s to compute)

```

## Painting form fields

```text
do for _,kind in ipairs{"fall","across","edge"} do work(rect(385,90,20,20),{hand="detail",pile=p,coverage=0.1,angle=f:field(kind)}) end;print("three fields accepted")
```

Exit 0; wall 0.038 seconds:

```text
three fields accepted
ok · chunk 19 (0.02 s to compute)

```

## Layer depth forms

```text
do spot=w:spot_at(0,20);for i,depth in ipairs{5,spot,{500,120},"ground"} do local wt=w:layer("test",rect(450,100,20,20),depth);local vt=wt:view();local seen=vt:seen(455,105);print(i,#vt:layers(),vt:visible("test"):area(),seen[1].what,seen[1].depth) end
```

Exit 0; wall 0.048 seconds:

```text
1	1	399.999968210858	layer	5.0
2	1	374.99997019767937	layer	20.0
3	1	399.999968210858	layer	19.313709259033203
4	1	399.999968210858	layer	30.636550903320312
ok · chunk 20 (0.03 s to compute)

```

## Mirror scan

```text
do local count=0;local example=nil;for y=82,175,3 do for x=305,695,3 do local mi=vp:mirror(x,y);if mi and mi.body then count=count+1;example=mi end end end;print("body mirrors",count);if example then print(example.body,example.src[1],example.src[2],example.fresnel,example.travel) end
```

Exit 0; wall 0.032 seconds:

```text
body mirrors	151
1	520.91204833984375	117.86434936523438	0.34835171699523926	3.4770767688751221
ok · chunk 21 (0.02 s to compute)

```

## Replay final space

```text
check
```

Exit 0; wall 1.67 seconds:

```text
replay matches the live canvas exactly (21 chunks, 1.7s)

```

## Close

```text
close
```

Exit 0; wall 0.098 seconds:

```text
the live canvas is in <isolated-root>/out/easel/space/live.png
closed; the session is in <isolated-root>/paintings/lua/space.lua

```

## Image inspection and limits

The [combined result](space-combined.png) was visually inspected: the protected foreground rectangle retains its earlier sparse marks and bare portions while broad paint surrounds it. Earlier [explicit](space-explicit.png), [implicit](space-implicit.png) and [behind](space-behind.png) states are retained. Decoded pixel comparisons establish zero changed pixels beyond named visible rectangles and zero changed pixels within the protected foreground for behind/combined operations.

The first water fixture supplied an invalid scalar ripple and failed; its dependent placement therefore failed too. Corrected water input uses a four-element ripple table and the later successful sections supply the evidence. Those initial fixture failures are not application defects.

Uncovered: proxy-only shadow comparison, terrain holes/facet/ceiling, alternate-camera visual comparison, all reserved names, soft overlapping depth shares, all light defaults/haze, per-option queued mutation and process/storage interruption. Direction fields were accepted but their directional appearance was not independently compared. No exhaustive numerical robustness claim is made.

Replay matched exactly after21 committed chunks. All sessions closed.
