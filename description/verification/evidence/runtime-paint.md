# Painting runtime probes

Agent-driven CLI checks on 2026-10-04 against clean source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`, default release build. Disposable EASEL_ROOT; no user painting. Each section records actual source and output. These runs do not establish unexercised appearance, concurrency or failure cases.

## Open

```text
open paint
```

Exit 0:

```text
easel "paint" open: 0 chunks · 2400px · no canvas yet

```

## Precanvas constructors

```text
do b=brush("round",4); h=pencil(); h:sharpen(); print(b,h)
```

Exit 0:

```text
brush(round 4, 0% full)	pencil HB (worn 0 mm)
ok · chunk 1 (0.00 s to compute)

```

## Precanvas rejection 1

```text
do b:load(pile{{"red earth",1}})
```

Exit 1:

```text
runtime error: no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}
(the chunk failed and changed nothing)

```

## Precanvas rejection 2

```text
do b:touch(20,20)
```

Exit 1:

```text
runtime error: no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}
(the chunk failed and changed nothing)

```

## Precanvas rejection 3

```text
do h:width()
```

Exit 1:

```text
runtime error: no canvas yet: call canvas{} first
(the chunk failed and changed nothing)

```

## Precanvas rejection 4

```text
do h:line({{10,10},{20,20}})
```

Exit 1:

```text
runtime error: no canvas yet: call canvas{} first
(the chunk failed and changed nothing)

```

## Precanvas rejection 5

```text
do erase(rect(10,10,20,20))
```

Exit 1:

```text
runtime error: no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}
(the chunk failed and changed nothing)

```

## Precanvas rejection 6

```text
do fix()
```

Exit 1:

```text
runtime error: no canvas yet: call canvas{} first
(the chunk failed and changed nothing)

```

## Precanvas rejection 7

```text
do drawing_guide()
```

Exit 1:

```text
runtime error: no canvas yet: call canvas{} first
(the chunk failed and changed nothing)

```

## Precanvas rejection 8

```text
do rag()
```

Exit 1:

```text
runtime error: no canvas yet: call canvas{} first
(the chunk failed and changed nothing)

```

## Precanvas rejection 9

```text
do everywhere()
```

Exit 1:

```text
runtime error: no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}
(the chunk failed and changed nothing)

```

## Canvas

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}
```

Exit 0:

```text
ok · chunk 2 (0.07 s to compute)

```

## Save blank

```text
save <isolated-root>/blank.png
```

Exit 0:

```text
<isolated-root>/blank.png

```

## Brush constructors

```text
do for _,k in ipairs{"round","flat","filbert","fan","rigger","badger","stippler","sable","hog","liner","blender"} do local z=brush(k,4); print(k,z,z:fullness()) end; print(brush("round 4"),brush{kind="round",width=4})
```

Exit 0:

```text
round	brush(round 4, 0% full)	0.0
flat	brush(flat 4, 0% full)	0.0
filbert	brush(filbert 4, 0% full)	0.0
fan	brush(fan 4, 0% full)	0.0
rigger	brush(rigger 4, 0% full)	0.0
badger	brush(blender 4, 0% full)	0.0
stippler	brush(round 4, 0% full)	0.0
sable	brush(round 4, 0% full)	0.0
hog	brush(flat 4, 0% full)	0.0
liner	brush(rigger 4, 0% full)	0.0
blender	brush(blender 4, 0% full)	0.0
brush(round 4, 0% full)	brush(round 4, 0% full)
ok · chunk 3 (0.00 s to compute)

```

## Brush invalid 1

```text
do brush("round",0)
```

Exit 1:

```text
runtime error: brush width 0: want > 0 (canvas units; the canvas is 1000 wide)
(the chunk failed and changed nothing)

```

## Brush invalid 2

```text
do brush("round",-1)
```

Exit 1:

```text
runtime error: brush width -1: want > 0 (canvas units; the canvas is 1000 wide)
(the chunk failed and changed nothing)

```

## Brush invalid 3

```text
do brush("round",0/0)
```

Exit 1:

```text
runtime error: brush width NaN: want > 0 (canvas units; the canvas is 1000 wide)
(the chunk failed and changed nothing)

```

## Brush invalid 4

```text
do brush("wat",4)
```

Exit 1:

```text
runtime error: brush kind "wat": use one of round, flat, filbert, fan, rigger, badger, stippler
(the chunk failed and changed nothing)

```

## Brush invalid 5

```text
do brush{kind="round",width=4,wat=1}
```

Exit 0:

```text
ok · chunk 4 (0.00 s to compute)

```

## Load alias copy wipe

```text
do b=brush("round",4); b:load(p,0.8); print("one",b:fullness()); b:load(p,0.8); print("two",b:fullness()); alias=b; copy=brush(b); alias:wipe(1); print("alias",b:fullness(),copy:fullness()); b:load(p); print("independent",b:fullness(),copy:fullness()); b:wipe(-1); print("negative wipe",b:fullness()); b:wipe(2); print("over wipe",b:fullness())
```

Exit 0:

```text
one	0.77146703004837036
two	1.5429340600967407
alias	0.0	0.0
independent	0.77146703004837036	0.0
negative wipe	0.77146703004837036
over wipe	0.0
ok · chunk 5 (0.00 s to compute)

```

## Negative loading

```text
do neg=brush("round",4); neg:load(p,-1); print(neg:fullness()); neg:touch(950,100); print(neg:fullness())
```

Exit 0:

```text
0.0
0.0
ok · chunk 6 (0.02 s to compute)

```

## Shapes numeric

```text
do m=rect(100,20,80,60); print("primitives",everywhere():area(),m:area(),ellipse(400,80,30):area(),poly({{100,20},{180,20},{150,100}}):area()); a=mask(function()return 0.4 end); z=mask(function()return 0.6 end); print("combine",(a+z):at(400,80),(a*z):at(400,80),(a-z):at(400,80),(-a):at(400,80)); c=mask(function(x)return x<500 and -1 or 2 end); print("clamp",c:at(100,80),c:at(900,80)); half=mask(function()return 0.5 end); print("map",half:map(function(v)return v*4 end):at(400,80)); print("distance",m:distance():at(140,50),m:distance():at(400,80)); print("outside",half:at(-100,-100),half:at(5000,5000)); old=m; grown=m:grow(5); print("immutable",old:area(),grown:area()); print("times",half:times(function()return 0.5 end):at(400,80),half:times(half):at(400,80),half:map(function(v)return v*v end):at(400,80))
```

Exit 0:

```text
primitives	199999.984105429	4799.9996185302962	2823.4188941192756	3200.1468052349987
combine	0.60000002384185791	0.24000000953674316	0.15999999642372131	0.60000002384185791
clamp	0.0	1.0
map	2.0
distance	29.791666030883789	-220.00021362304688
outside	0.5	0.5
immutable	4799.9996185302962	6276.7260040129986
times	0.25	0.25	0.25
ok · chunk 7 (0.33 s to compute)

```

## Mask callback rollback

```text
do m=mask(function(x)if x>500 then error("callback fails") end return 1 end)
```

Exit 1:

```text
runtime error: [string "chunk 8"]:1: callback fails
(the chunk failed and changed nothing)

```

## Mask retained

```text
do print(m:area(),old:area())
```

Exit 0:

```text
4799.9996185302962	4799.9996185302962
ok · chunk 8 (0.00 s to compute)

```

## Shape invalid 1

```text
do poly({{1,1},{2,2}})
```

Exit 1:

```text
runtime error: poly: needs at least three points
(the chunk failed and changed nothing)

```

## Shape invalid 2

```text
do ribbon({{1,1}},2)
```

Exit 1:

```text
runtime error: ribbon: needs >= 2 points and one width per point (or one width)
(the chunk failed and changed nothing)

```

## Shape invalid 3

```text
do ribbon({{1,1},{2,2},{3,3}},{1,2})
```

Exit 1:

```text
runtime error: ribbon: needs >= 2 points and one width per point (or one width)
(the chunk failed and changed nothing)

```

## Shape invalid 4

```text
do outline({{1,1}})
```

Exit 1:

```text
runtime error: outline: needs at least two points
(the chunk failed and changed nothing)

```

## Outline closure

```text
do o=outline({{100,20},{180,20},{150,80}},{amount=0}); op=outline({{100,20},{180,20}}); both=outline({{100,20},{180,20},{150,80}},{open=true,closed=true}); print(o.closed,op.closed,both.closed); print(o,o:mask():area()); tiny=outline({{100,20},{110,20},{110,30},{100,30}},{amount=0}):inset(7); print("consumed",#tiny:paths(),tiny:mask():area())
```

Exit 0:

```text
true	false	true
outline(1 line(s), closed, length 226, 6 strokes, 3 corners, scale 100)	2604.0432307475721
consumed	0	0.0
ok · chunk 9 (0.00 s to compute)

```

## Open mask rejection

```text
do op:mask()
```

Exit 1:

```text
runtime error: outline:mask(): this line is open; use :below() or :above() (or closed=true)
(the chunk failed and changed nothing)

```

## Save constructed

```text
save <isolated-root>/constructed.png
```

Exit 0:

```text
<isolated-root>/constructed.png

```

PNG byte equality blank/constructed: **true**.

## Pencil constructors

```text
do print(pencil(),pencil("2H"),pencil{grade="2B"},chalk(),pencil{kind="chalk"}); for _,g in ipairs{"9H","H","F","HB","B","9B"," 2h "} do print(pencil(g)) end
```

Exit 0:

```text
pencil HB (worn 0 mm)	pencil 2H (worn 0 mm)	pencil 2B (worn 0 mm)	black chalk (worn 0 mm)	black chalk (worn 0 mm)
pencil 9H (worn 0 mm)
pencil H (worn 0 mm)
pencil F (worn 0 mm)
pencil HB (worn 0 mm)
pencil B (worn 0 mm)
pencil 9B (worn 0 mm)
pencil  2h  (worn 0 mm)
ok · chunk 10 (0.00 s to compute)

```

## Pencil invalid 1

```text
do pencil("10B")
```

Exit 1:

```text
runtime error: pencil grade "10B": 9H..H, F, HB, B..9B (e.g. "2H", "HB", "2B", "4B")
(the chunk failed and changed nothing)

```

## Pencil invalid 2

```text
do pencil{kind="wat"}
```

Exit 1:

```text
runtime error: pencil kind "wat": graphite or chalk
(the chunk failed and changed nothing)

```

## Pencil invalid 3

```text
do h:line({{1,1}})
```

Exit 1:

```text
runtime error: line: needs at least two points
(the chunk failed and changed nothing)

```

## Pencil invalid 4

```text
do h:sketch({{1,1}})
```

Exit 1:

```text
runtime error: sketch: needs at least two points
(the chunk failed and changed nothing)

```

## Pencil invalid 5

```text
do h:rule({{1,1},{2,2}})
```

Exit 1:

```text
runtime error: points: want {{x, y}, ...} or {x1, y1, x2, y2, ...}
(the chunk failed and changed nothing)

```

## Pencil invalid 6

```text
do h:line({{1,1},{2,2}},{pressure={}})
```

Exit 1:

```text
runtime error: pressure: a number 0..1 or a list of them along the line
(the chunk failed and changed nothing)

```

## Pencil invalid 7

```text
do h:line({{1,1},{2,2}},{wat=1})
```

Exit 1:

```text
runtime error: line: unknown option "wat" (options: pressure, smooth, ruler, tremor, seed)
(the chunk failed and changed nothing)

```

## Pencil invalid 8

```text
do h:hatch(m,{spacing=0})
```

Exit 1:

```text
runtime error: hatch: spacing and length want > 0 (units)
(the chunk failed and changed nothing)

```

## Pencil invalid 9

```text
do h:hatch(m,{length=-1})
```

Exit 1:

```text
runtime error: hatch: spacing and length want > 0 (units)
(the chunk failed and changed nothing)

```

## Drawing empty

```text
do erase(everywhere()); fix(); print(drawing_guide():area())
```

Exit 0:

```text
0.0
ok · chunk 11 (0.00 s to compute)

```

## Wear guide snapshot

```text
do h=pencil("HB"); alias=h; g=drawing_guide(); print("before",h:width(),h.worn,g:area()); print("distance",h:rule({100,40},{800,40},{pressure=0.2})); print("after",alias:width(),alias.worn,g:area(),drawing_guide():area()); print("coverage",drawing_guide():at(400,40)); h:rule({100,70},{800,70},{pressure=0.9}); print("firm",drawing_guide():at(400,70))
```

Exit 0:

```text
before	3.1999998092651367	0.0	0.0
distance	70.0
after	3.3110253810882568	70.0	0.0	1318.6127963701317
coverage	0.65535998344421387
firm	0.97791999578475952
ok · chunk 12 (0.04 s to compute)

```

## Save drawn

```text
save <isolated-root>/drawn.png
```

Exit 0:

```text
<isolated-root>/drawn.png

```

## Drawing rollback

```text
do h:line({{100,100},{800,100}}); h:sharpen(); error("rollback")
```

Exit 1:

```text
runtime error: [string "chunk 13"]:1: rollback
(the chunk failed and changed nothing)

```

## Save drawing-rollback

```text
save <isolated-root>/drawing-rollback.png
```

Exit 0:

```text
<isolated-root>/drawing-rollback.png

```

PNG byte equality drawn/drawing-rollback: **true**.

## Sharpen

```text
do print("retained",h.worn,h:width()); h:sharpen(); print("fresh",h.worn,h:width())
```

Exit 0:

```text
retained	140.0	3.4201252460479736
fresh	0.0	3.1999998092651367
ok · chunk 13 (0.00 s to compute)

```

## Save sharpened

```text
save <isolated-root>/sharpened.png
```

Exit 0:

```text
<isolated-root>/sharpened.png

```

PNG byte equality drawn/sharpened: **true**.

## Sealing setup

```text
do work(rect(300,20,200,70),{hand="detail",pile=p,coverage=3,clip=true}); print("guide before",drawing_guide():area())
```

Exit 0:

```text
guide before	4003.0022138439158
ok · chunk 14 (0.75 s to compute)

```

## Save sealed

```text
save <isolated-root>/sealed.png
```

Exit 0:

```text
<isolated-root>/sealed.png

```

## Sealed erase

```text
do erase(rect(310,25,180,60),{strength=1}); print("guide after",drawing_guide():area())
```

Exit 0:

```text
guide after	3133.6021911087414
ok · chunk 15 (0.00 s to compute)

```

## Save sealed-erased

```text
save <isolated-root>/sealed-erased.png
```

Exit 0:

```text
<isolated-root>/sealed-erased.png

```

PNG byte equality sealed/sealed-erased: **true**.

## Rag constructors

```text
do r=rag(); r2=rag{width=20}; print(r,r.width,r.load,r.soaked,r.damp,r.fold); print(r2,r2.width,r2.load,r2.soaked,r2.damp,r2.fold); print(rag{width=0.01}.width)
```

Exit 0:

```text
rag(width 400, load 0.00)	400.0	0.0	0.0	0.0	0
rag(width 20, load 0.00)	20.0	0.0	0.0	0.0	0
0.10000000149011612
ok · chunk 16 (0.00 s to compute)

```

## Rag invalid 1

```text
do rag{width=0}
```

Exit 1:

```text
runtime error: rag: width 0: want > 0 (canvas units)
(the chunk failed and changed nothing)

```

## Rag invalid 2

```text
do rag{width=-1}
```

Exit 1:

```text
runtime error: rag: width -1: want > 0 (canvas units)
(the chunk failed and changed nothing)

```

## Rag invalid 3

```text
do rag{width=0/0}
```

Exit 1:

```text
runtime error: rag: width NaN: want > 0 (canvas units)
(the chunk failed and changed nothing)

```

## Rag invalid 4

```text
do r.width=2
```

Exit 1:

```text
runtime error: a rag's width can't be set: the cloth changes only as it is used; r:refold() turns a cleaner part outward, rag() takes a fresh one
(the chunk failed and changed nothing)

```

## Rag invalid 5

```text
do r.load=1
```

Exit 1:

```text
runtime error: a rag's load can't be set: the cloth changes only as it is used; r:refold() turns a cleaner part outward, rag() takes a fresh one
(the chunk failed and changed nothing)

```

## Rag invalid 6

```text
do r:wipe({{1,1}})
```

Exit 0:

```text
ok · chunk 17 (0.01 s to compute)

```

## Rag invalid 7

```text
do r:wipe(m,{passes=0})
```

Exit 1:

```text
runtime error: wipe: passes 0: a whole number 1..20
(the chunk failed and changed nothing)

```

## Rag invalid 8

```text
do r:wipe(m,{passes=21})
```

Exit 1:

```text
runtime error: wipe: passes 21: a whole number 1..20
(the chunk failed and changed nothing)

```

## Rag invalid 9

```text
do r:wipe(m,{passes=1.5})
```

Exit 1:

```text
runtime error: wipe: passes 1.5: a whole number 1..20
(the chunk failed and changed nothing)

```

## Rag invalid 10

```text
do r:wipe({{1,1},{2,2}},{angle=1})
```

Exit 1:

```text
runtime error: wipe: angle= is for wiping a mask; along a path, give pressure= (and seed=)
(the chunk failed and changed nothing)

```

## Dip clamp decay refold

```text
do r:dip(); print("default",r.damp); r:dip(-1); print("negative",r.damp); r:dip(2); print("high",r.damp); wait(3); print("decay",r.damp); r:refold(); print("fold",r.fold,r.damp,r.load,r.soaked)
```

Exit 0:

```text
default	0.5
negative	0.49520957469940186
high	1.0
decay	0.5
fold	1	0.0	0.0	0.0
ok · chunk 18 (0.00 s to compute)

```

## Rag wet lifting

```text
do print("before",r2.load,r2.soaked); r2:dip(0.5); r2:blot(400,50); print("after",r2.load,r2.soaked,r2.damp); r2:wipe({{320,60},{480,60}},{pressure={0.2,0.9}}); print("wipe",r2.load,r2.soaked,r2.fold)
```

Exit 0:

```text
before	0.0	0.0
after	0.19632697105407715	0.016360580921173096	0.49819093942642212
wipe	1.0	0.083338379859924316	0
ok · chunk 19 (0.02 s to compute)

```

## Save ragged

```text
save <isolated-root>/ragged.png
```

Exit 0:

```text
<isolated-root>/ragged.png

```

## Rag rollback baseline

```text
do print(r2.load,r2.soaked,r2.damp,r2.fold)
```

Exit 0:

```text
1.0	0.083338379859924316	0.49725189805030823	0
ok · chunk 20 (0.00 s to compute)

```

## Rag rollback

```text
do r2:refold(); r2:blot(400,50); error("rollback")
```

Exit 1:

```text
runtime error: [string "chunk 21"]:1: rollback
(the chunk failed and changed nothing)

```

## Rag rollback fields

```text
do print(r2.load,r2.soaked,r2.damp,r2.fold)
```

Exit 0:

```text
1.0	0.083338379859924316	0.49725189805030823	0
ok · chunk 21 (0.00 s to compute)

```

## Save rag-rollback

```text
save <isolated-root>/rag-rollback.png
```

Exit 0:

```text
<isolated-root>/rag-rollback.png

```

PNG byte equality ragged/rag-rollback: **true**.

## Brush mark baseline

```text
do b=brush("round",4); b:load(p); print(b:fullness()); b:stroke({{550,110},{850,110}}); print(b:fullness())
```

Exit 0:

```text
0.77146703004837036
0.030642397701740265
ok · chunk 22 (0.02 s to compute)

```

## Save brush

```text
save <isolated-root>/brush.png
```

Exit 0:

```text
<isolated-root>/brush.png

```

## Brush rollback

```text
do b:touch(700,100); error("rollback")
```

Exit 1:

```text
runtime error: [string "chunk 23"]:1: rollback
(the chunk failed and changed nothing)

```

## Brush rollback fields

```text
do print(b:fullness())
```

Exit 0:

```text
0.030642397701740265
ok · chunk 23 (0.00 s to compute)

```

## Save brush-rollback

```text
save <isolated-root>/brush-rollback.png
```

Exit 0:

```text
<isolated-root>/brush-rollback.png

```

PNG byte equality brush/brush-rollback: **true**.

## Brush mark invalid 1

```text
do b:stroke({{1,1}})
```

Exit 1:

```text
runtime error: stroke: needs at least two points
(the chunk failed and changed nothing)

```

## Brush mark invalid 2

```text
do b:stroke({{1,1},{2,2}},{wat=1})
```

Exit 1:

```text
runtime error: stroke: unknown option "wat" (options: pressure, ramps, orient, shake, swell, clip)
(the chunk failed and changed nothing)

```

## Brush mark invalid 3

```text
do b:stroke({{1,1},{2,2}},{orient="wat"})
```

Exit 1:

```text
runtime error: orient "wat": "across", "along" or an angle
(the chunk failed and changed nothing)

```

## Brush mark invalid 4

```text
do b:touch(5,5,{wat=1})
```

Exit 1:

```text
runtime error: touch: unknown option "wat" (options: pressure, drag, twist, angle, clip)
(the chunk failed and changed nothing)

```

## Brush mark invalid 5

```text
do b:load(p,0.8,0.2)
```

Exit 1:

```text
runtime error: b:load(pile, amount): a pile carries its own medium; mix another pile for other paint
(the chunk failed and changed nothing)

```

## Replay check

```text
check
```

Exit 0:

```text
replay matches the live canvas exactly (23 chunks, 1.3s)

```

## Close

```text
close
```

Exit 0:

```text
the live canvas is in <isolated-root>/out/easel/paint/live.png
closed; the session is in <isolated-root>/paintings/lua/paint.lua

```
# Painting runtime probes

Agent-driven CLI checks on 2026-10-04 against clean source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`, default release build. Disposable EASEL_ROOT; no user painting. Each section records actual source and output. These runs do not establish unexercised appearance, concurrency or failure cases.

## Reopen for space

```text
open paint
```

Exit 0:

```text
resuming chunk 1/23
resumed chunk 1/23 0.00s
resuming chunk 2/23
resumed chunk 2/23 0.07s
resuming chunk 3/23
resumed chunk 3/23 0.00s
resuming chunk 4/23
resumed chunk 4/23 0.00s
resuming chunk 5/23
resumed chunk 5/23 0.00s
resuming chunk 6/23
resumed chunk 6/23 0.02s
resuming chunk 7/23
resumed chunk 7/23 0.32s
resuming chunk 8/23
resumed chunk 8/23 0.00s
resuming chunk 9/23
resumed chunk 9/23 0.00s
resuming chunk 10/23
resumed chunk 10/23 0.00s
resuming chunk 11/23
resumed chunk 11/23 0.00s
resuming chunk 12/23
resumed chunk 12/23 0.04s
resuming chunk 13/23
resumed chunk 13/23 0.00s
resuming chunk 14/23
resumed chunk 14/23 0.74s
resuming chunk 15/23
resumed chunk 15/23 0.00s
resuming chunk 16/23
resumed chunk 16/23 0.00s
resuming chunk 17/23
resumed chunk 17/23 0.01s
resuming chunk 18/23
resumed chunk 18/23 0.00s
resuming chunk 19/23
resumed chunk 19/23 0.02s
resuming chunk 20/23
resumed chunk 20/23 0.00s
resuming chunk 21/23
resumed chunk 21/23 0.00s
resuming chunk 22/23
resumed chunk 22/23 0.02s
resuming chunk 23/23
resumed chunk 23/23 0.00s
resumed 23 chunks from <isolated-root>/paintings/lua/paint.lua
easel "paint" open: 23 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1

```

## Save before-space

```text
save <isolated-root>/before-space.png
```

Exit 0:

```text
<isolated-root>/before-space.png

```

## World depth layer setup

```text
do w=world{view={100,20,300,150},horizon=80}; v0=w:view(); w2=w:layer("near",rect(200,80,80,80),3); v=w2:view(); print("visible",v:visible("near"):area()); print("nested",v:visible({{rect(200,80,80,80)}}):area()); print("sky",v:sky():area()); print("projection",w:project(0,0,-1)==nil)
```

Exit 0:

```text
visible	6399.9994913737282
nested	6399.9994913737282
sky	17999.998569488609
projection	true
ok · chunk 24 (0.02 s to compute)

```

## Old world retains no layer

```text
do v0:visible("near")
```

Exit 1:

```text
runtime error: no layer "near" in this view's world (layers: none; also "ground", "water", "surface", "sky", "bodies", "layers" or a body number). Register it with w = w:layer("near", mask, depth), then v = w:view()
(the chunk failed and changed nothing)

```

## Top level behind mask

```text
do work(rect(200,80,10,10),{pile=p,coverage=0,behind={rect(200,80,80,80)}})
```

Exit 0:

```text
ok · chunk 25 (0.00 s to compute)

```

## Nested behind mask

```text
do work(rect(200,80,10,10),{pile=p,coverage=0,behind={{rect(200,80,80,80)}}})
```

Exit 1:

```text
runtime error: want a body number, a layer name, "ground", "water", "sky" or a list, got userdata
(the chunk failed and changed nothing)

```

## Nested visible mask

```text
do work(rect(200,80,10,10),{pile=p,coverage=0,visible={{rect(200,80,80,80)}}})
```

Exit 0:

```text
ok · chunk 26 (0.00 s to compute)

```

## Unknown layer

```text
do v:visible("absent")
```

Exit 1:

```text
runtime error: no layer "absent" in this view's world (layers: near; also "ground", "water", "surface", "sky", "bodies", "layers" or a body number). Register it with w = w:layer("absent", mask, depth), then v = w:view()
(the chunk failed and changed nothing)

```

## Sky ground lookup

```text
do w:spot(200,30)
```

Exit 0:

```text
ok · chunk 27 (0.00 s to compute)

```

## Body and form

```text
do solid=body.ellipsoid({300,100,0},{40,30,20}); turned=solid:turn({300,100,0},0.2,0,0); f=form{{solid},light={from={-1,-0.7},front=0.5,ambient=0.2}}; print("part",f:part(300,100),f:part(100,100)); print("value",f:value(300,100)); print("custom clamp",f:mask(function(s)return 2 end):at(300,100))
```

Exit 0:

```text
part	1	0
value	0.44665133953094482
custom clamp	1.0
ok · chunk 28 (0.05 s to compute)

```

## Terrain restriction

```text
do t=terrain{area={200,80,240,120},step=4,height=function(x,y)return 1 end}; t:turn({220,100,0},0,0,0)
```

Exit 1:

```text
runtime error: only bodies turn, cut, weather and combine (not terrain)
(the chunk failed and changed nothing)

```

## World invalid key

```text
do world{wat=1}
```

Exit 1:

```text
runtime error: world: unknown option "wat" (options: horizon, eye, fov, view, ground, water, sun, visibility, backdrop)
(the chunk failed and changed nothing)

```

## Form negative softness

```text
do f:lit{soft=-1}
```

Exit 1:

```text
runtime error: lit: soft must be a finite width of 0 or more, got -1
(the chunk failed and changed nothing)

```

## Save after-space

```text
save <isolated-root>/after-space.png
```

Exit 0:

```text
<isolated-root>/after-space.png

```

PNG byte equality before-space/after-space: **true**.

## Palette retention

```text
do kept=pile{{"red earth",1}}; for i=1,18 do local q=pile{{"lead white",1}} end; print(kept); b:load(kept); print(b:fullness())
```

Exit 0:

```text
pile(red earth 1; medium 0)
0.8021092414855957
ok · chunk 29 (0.00 s to compute)

```

## Rag alias fresh

```text
do alias=r2; fresh=rag{width=20}; alias:refold(); print(r2.fold,alias.fold,fresh.fold,r2.load,r2.soaked,fresh.load,fresh.soaked)
```

Exit 0:

```text
1	1	0	0.083338379859924316	0.083338379859924316	0.0	0.0
ok · chunk 30 (0.00 s to compute)

```

## Brush reload arithmetic

```text
do b:wipe(1); b:load(p,0.8); local before=b:fullness(); b:reload(p,0.8); print(before,b:fullness(),before*1.15); pointed=brush{kind="round",width=10,point=1}; print("point",pointed:mark_width(0.1),pointed:mark_width(0.9),pointed:pressure_for(1))
```

Exit 0:

```text
0.77146703004837036	0.88718694448471069	0.88718708455562589
point	1.4534441232681274	11.150999069213867	4.6566128730773926e-10
ok · chunk 31 (0.00 s to compute)

```

## Replay after space

```text
check
```

Exit 0:

```text
replay matches the live canvas exactly (31 chunks, 1.3s)

```

## Close second pass

```text
close
```

Exit 0:

```text
the live canvas is in <isolated-root>/out/easel/paint/live.png
closed; the session is in <isolated-root>/paintings/lua/paint.lua

```
# Painting runtime probes

Agent-driven CLI checks on 2026-10-04 against clean source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`, default release build. Disposable EASEL_ROOT; no user painting. Each section records actual source and output. These runs do not establish unexercised appearance, concurrency or failure cases.

## Open visual atlas

```text
open visual
```

Exit 0:

```text
easel "visual" open: 0 chunks · 2400px · no canvas yet

```

## Atlas canvas

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}
```

Exit 0:

```text
ok · chunk 1 (0.08 s to compute)

```

## Drawing grade atlas

```text
do for i,g in ipairs{"9H","HB","9B"} do local h=pencil(g); h:rule({50,30*i},{350,30*i},{pressure=0.5}); print(g,h.worn,h:width()) end; c=chalk(); c:rule({50,120},{350,120},{pressure=0.5}); print(c,c:width())
```

Exit 0:

```text
9H	30.0	3.2024955749511719
HB	30.0	3.2478201389312744
9B	30.0	8.62668514251709
black chalk (worn 30 mm)	10.478583335876465
ok · chunk 2 (0.07 s to compute)

```

## Brush pressure atlas

```text
do for i,pres in ipairs{{0.9,0},{0,0.9},{0.8,0.8}} do local b=brush{kind="round",width=12,point=1}; b:load(p); b:stroke({{450,30*i},{850,30*i}},{pressure=pres,ramps={0,0},shake=0}); print(b:fullness()) end
```

Exit 0:

```text
0.14048917591571808
0.14394892752170563
0.041140541434288025
ok · chunk 3 (0.06 s to compute)

```

## Save atlas

```text
save <isolated-root>/atlas.png
```

Exit 0:

```text
<isolated-root>/atlas.png

```

## Fix baseline

```text
do h=pencil("9B"); h:rule({50,155},{350,155},{pressure=0.8}); fix(rect(40,140,150,30)); print("before",drawing_guide():at(100,155),drawing_guide():at(300,155))
```

Exit 0:

```text
before	1.0	1.0
ok · chunk 4 (0.03 s to compute)

```

## Save fix-before

```text
save <isolated-root>/fix-before.png
```

Exit 0:

```text
<isolated-root>/fix-before.png

```

## Fix erase

```text
do erase(rect(40,140,330,30),{strength=1}); print("after",drawing_guide():at(100,155),drawing_guide():at(300,155))
```

Exit 0:

```text
after	1.0	0.25115996599197388
ok · chunk 5 (0.01 s to compute)

```

## Save fix-after

```text
save <isolated-root>/fix-after.png
```

Exit 0:

```text
<isolated-root>/fix-after.png

```

## Dry rag protection setup

```text
do work(rect(450,130,200,40),{hand="detail",pile=p,coverage=2,clip=true}); wait(5259600); print(drying(550,150)); r=rag{width=20}; r:dip(1)
```

Exit 0:

```text
dry
ok · chunk 6 (0.31 s to compute)

```

## Save dry-before

```text
save <isolated-root>/dry-before.png
```

Exit 0:

```text
<isolated-root>/dry-before.png

```

## Dry rag protection

```text
do r:wipe({{470,150},{630,150}},{pressure=1}); print(r.load,r.soaked)
```

Exit 0:

```text
0.0	0.0
ok · chunk 7 (0.01 s to compute)

```

## Save dry-after

```text
save <isolated-root>/dry-after.png
```

Exit 0:

```text
<isolated-root>/dry-after.png

```

PNG byte equality dry-before/dry-after: **true**.

## Ground rag protection

```text
do fresh=rag{width=20}; fresh:blot(950,150); print(fresh.load,fresh.soaked)
```

Exit 0:

```text
0.0	0.0
ok · chunk 8 (0.00 s to compute)

```

## Close visual

```text
close
```

Exit 0:

```text
the live canvas is in <isolated-root>/out/easel/visual/live.png
closed; the session is in <isolated-root>/paintings/lua/visual.lua

```

## Visual inspection and limits

The [atlas](paint-atlas.png) was visually inspected: left rows are 9H, HB, 9B and chalk; right rows are declining, increasing and constant pressure with a pointed brush. Hard graphite is faint, softer graphite is darker and broader, and the brush rows taper in the submitted direction. This does not establish every tool's texture or sheen under alternate lighting.

The [sealed image](paint-sealed.png) and [erased sealed image](paint-sealed-erased.png) are byte-identical despite the recorded guide-area reduction. The [rag result](paint-ragged.png) visibly exposes pale substrate within the painted patch. [Before fixing/erasing](paint-fix-before.png) and [after erasing](paint-fix-after.png) preserve the fixed portion while reducing the loose portion.

Unknown brush constructor key `wat` was accepted, contradicting checklist BR-006's expectation. This is a checklist correction unless the project requires strict constructor keys; stroke/touch unknown keys still rejected. A one-point rag wipe also succeeded as documented short-path blot behavior. Neither is classified as an application bug merely because a negative probe accepted it.

No provider, concurrency, abrupt process termination or storage-failure scenarios were run in this lane. Rows with multiple conditions retain blocked—partial when only a subset was established. All disposable sessions were closed.
