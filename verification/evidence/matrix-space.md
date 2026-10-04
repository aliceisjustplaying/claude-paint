# Remaining space matrix: runtime evidence

Agent-driven probes on 2026-10-04 against unchanged commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. Every command below ran through the actual CLI or unmodified runner script. Temporary paths are replaced by `<fixture>`; script-building source is `<source>`. Failures used disposable sessions or outputs. [Machine-readable record](matrix-space.json).

Reference geometry, terrain boundaries, camera/light variants, masks, depth shares, accepted/queued request behavior and process/persistence interruptions were exercised. The full-width exploratory union of a body and an unbounded half-space was stopped after prolonged computation; the identical16px replay completed in0.14sec, so no infinite-loop defect is claimed. A mistyped file invocation initially failed as inline Lua; corrected `do -f` cases below are the channel evidence. Hard-mask interpolation can produce fractional queried coverage at an edge; `lit_at(...,0)` was separately sampled to test the actual hard-light predicate.

## space open

```text
<binaries>/easel-default open space
```

Exit: 0. Elapsed: 0.135 seconds.

```text
easel "space" open: 0 chunks · 2400px · no canvas yet
```

## space canvas

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}\;p\=pile\{\{\"red\ earth\",1\}\}
```

Exit: 0. Elapsed: 0.206 seconds.

```text
ok · chunk 1 (0.18 s to compute)
```

## default world

```text
<binaries>/easel-default do wd\=world\{\}\;vd\=wd:view\(\)\;print\(wd,wd.eye,wd.horizon,vd:water\(\):area\(\),vd:land\(\):area\(\),vd:sky\(\):area\(\)\)\;for\ _,p\ in\ ipairs\{\{0,0,10\},\{1,0,10\},\{0,1,10\},\{0,0,20\}\}\ do\ print\(table.unpack\(wd:project\(table.unpack\(p\)\)\)\)\ end
```

Exit: 0. Elapsed: 0.059 seconds.

```text
world(horizon y 100, eye 1.6 m, sun az -120° el 35°, 0 bodies)	1.6000000238418579	99.999992370605469	0.0	99999.9920527145	99999.9920527145
500.0	293.1370849609375
620.710693359375	293.1370849609375
500.0	172.4263916015625
500.0	196.56854248046875
ok · chunk 2 (0.04 s to compute)
```

## terrain holes facet

```text
<binaries>/easel-default do t\=terrain\{area\=\{400,60,440,100\},step\=1,facet\=7,height\=function\(x,y\)if\ x\>412\ and\ x\<428\ and\ y\>72\ and\ y\<88\ then\ return\ nil\ end\;return\ 5\ end\}\;tf\=form\{\{t\}\}\;print\(tf:sample\(420,80\)\=\=nil,tf:sample\(405,65\).z,tf:sample\(405,65\).facet\)\;print\(pcall\(function\(\)t:turn\(\{0,0,0\},1\)end\)\)\;print\(pcall\(function\(\)wd:place\(wd:spot_at\(0,10\),t\)end\)\)
```

Exit: 0. Elapsed: 0.029 seconds.

```text
true	5.0	7
false	runtime error: only bodies turn, cut, weather and combine (not terrain)
stack traceback:
	[C]: in method 'turn'
	[string "chunk 3"]:1: in function <[string "chunk 3"]:1>
	[C]: in global 'pcall'
	[string "chunk 3"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: only bodies (body.ellipsoid, body.block) stand in a world
stack traceback:
	[C]: in method 'place'
	[string "chunk 3"]:1: in function <[string "chunk 3"]:1>
	[C]: in global 'pcall'
	[string "chunk 3"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

ok · chunk 3 (0.00 s to compute)
```

## terrain step counts

```text
<binaries>/easel-default do for\ _,step\ in\ ipairs\{1,0.1,0.25\}\ do\ local\ count\=0\;terrain\{area\=\{0,0,2,2\},step\=step,height\=function\(\)count\=count+1\;return\ 0\ end\}\;print\(step,count\)\ end\;local\ count\=0\;terrain\{area\=\{0,0,2,2\},height\=function\(\)count\=count+1\;return\ 0\ end\}\;print\(\"default\",count\)\;print\(pcall\(function\(\)terrain\{area\=\{0,0,1001,1001\},step\=.25,height\=function\(\)error\(\"should\ not\ sample\"\)end\}end\)\)
```

Exit: 0. Elapsed: 0.033 seconds.

```text
1	9
0.1	81
0.25	81
default	9
false	runtime error: terrain: at most 4001 x 4001 samples (a larger step or a smaller area)
stack traceback:
	[C]: in global 'terrain'
	[string "chunk 4"]:1: in function <[string "chunk 4"]:1>
	[C]: in global 'pcall'
	[string "chunk 4"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

ok · chunk 4 (0.00 s to compute)
```

## terrain invalid cases

```text
<binaries>/easel-default do for\ _,a\ in\ ipairs\{\{0,0,0,1\},\{0,0,1\},\{0,0,math.huge,2\}\}\ do\ print\(pcall\(function\(\)terrain\{area\=a,height\=function\(\)return\ 0\ end\}end\)\)\ end\;print\(pcall\(function\(\)world\{unknown\=true\}end\)\)
```

Exit: 0. Elapsed: 0.025 seconds.

```text
false	runtime error: terrain: area = {x0, y0, x1, y1} with x1 - x0 and y1 - y0 at least one step
stack traceback:
	[C]: in global 'terrain'
	[string "chunk 5"]:1: in function <[string "chunk 5"]:1>
	[C]: in global 'pcall'
	[string "chunk 5"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: terrain: area = {x0, y0, x1, y1}
stack traceback:
	[C]: in global 'terrain'
	[string "chunk 5"]:1: in function <[string "chunk 5"]:1>
	[C]: in global 'pcall'
	[string "chunk 5"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: terrain: area = {x0, y0, x1, y1} with x1 - x0 and y1 - y0 at least one step
stack traceback:
	[C]: in global 'terrain'
	[string "chunk 5"]:1: in function <[string "chunk 5"]:1>
	[C]: in global 'pcall'
	[string "chunk 5"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: world: unknown option "unknown" (options: horizon, eye, fov, view, ground, water, sun, visibility, backdrop)
stack traceback:
	[C]: in global 'world'
	[string "chunk 5"]:1: in function <[string "chunk 5"]:1>
	[C]: in global 'pcall'
	[string "chunk 5"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

ok · chunk 5 (0.00 s to compute)
```

## terrain reversed corners

```text
<binaries>/easel-default do ta\=terrain\{area\=\{400,60,440,100\},height\=function\(x,y\)return\ x\*.1+y\*.2\ end\}\;tb\=terrain\{area\=\{440,100,400,60\},height\=function\(x,y\)return\ x\*.1+y\*.2\ end\}\;fa\=form\{\{ta\}\}\;fb\=form\{\{tb\}\}\;print\(fa:sample\(420,80\).z,fb:sample\(420,80\).z\)
```

Exit: 0. Elapsed: 0.032 seconds.

```text
58.062496185302734	58.062496185302734
ok · chunk 6 (0.01 s to compute)
```

## space recover

```text
<binaries>/easel-default open space
```

Exit: 0. Elapsed: 0.544 seconds.

```text
resuming chunk 1/6
resumed chunk 1/6 0.43s
resuming chunk 2/6
resumed chunk 2/6 0.05s
resuming chunk 3/6
resumed chunk 3/6 0.00s
resuming chunk 4/6
resumed chunk 4/6 0.00s
resuming chunk 5/6
resumed chunk 5/6 0.00s
resuming chunk 6/6
resumed chunk 6/6 0.01s
resumed 6 chunks from <fixture>/space/paintings/lua/space.lua
easel "space" open: 6 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## body derivations

```text
<binaries>/easel-default do s\=body.ellipsoid\(\{400,100,0\},\{40,20,10\}\)\;k\=body.block\(\{420,100,0\},\{30,30,30\},1\)\;f\=form\{\{s\}\}\;original\=f:parts_mask\{1\}:area\(\)\;for\ _,b\ in\ ipairs\{s:turn\(\{400,100,0\},0.8,0.5\),s:cut\(\{400,100,0\},\{0,-1,0\}\),s:rough\(4,12,3\),s:union\(k\),s:subtract\(k\)\}do\ local\ ff\=form\{\{b\}\}\;print\(ff:parts_mask\{1\}:area\(\),f:parts_mask\{1\}:area\(\)\)end\;assert\(original\=\=f:parts_mask\{1\}:area\(\)\)
```

Exit: 0. Elapsed: 0.234 seconds.

```text
1698.6109761176367	2517.3609110492366
1258.3332333299909	2517.3609110492366
2615.2775699341864	2517.3609110492366
2557.6386856260242	2517.3609110492366
1658.3332015408489	2517.3609110492366
ok · chunk 7 (0.21 s to compute)
```

## form depth directions

```text
<binaries>/easel-default do fdepth\=form\{\{body.ellipsoid\(\{400,100,-10\},\{20,20,10\}\)\},\{body.ellipsoid\(\{400,100,10\},\{20,20,10\}\)\}\}\;print\(fdepth:part\(400,100\),fdepth:sample\(400,100\).z\)
```

Exit: 0. Elapsed: 0.035 seconds.

```text
2	19.99891471862793
ok · chunk 8 (0.01 s to compute)
```

## light soft defaults

```text
<binaries>/easel-default do print\(f:lit\(\):area\(\),f:lit\{soft\=.12\}:area\(\),f:shadow\(\):area\(\),f:shadow\{soft\=.12\}:area\(\),f:lit\{soft\=0\}:at\(400,100\)\)\;for\ _,x\ in\ ipairs\{-1,0/0,math.huge\}do\ print\(pcall\(function\(\)f:lit\{soft\=x\}end\)\)\;print\(pcall\(function\(\)f:shadow\{soft\=x\}end\)\)\;print\(pcall\(function\(\)f:lit_at\(400,100,x\)end\)\)end
```

Exit: 0. Elapsed: 0.089 seconds.

```text
0.0	0.0	2517.3609110492366	2517.3609110492366	0.0
false	runtime error: lit: soft must be a finite width of 0 or more, got -1
stack traceback:
	[C]: in method 'lit'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: shadow: soft must be a finite width of 0 or more, got -1
stack traceback:
	[C]: in method 'shadow'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: lit_at: soft must be a finite width of 0 or more, got -1
stack traceback:
	[C]: in method 'lit_at'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: lit: soft must be a finite width of 0 or more, got NaN
stack traceback:
	[C]: in method 'lit'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: shadow: soft must be a finite width of 0 or more, got NaN
stack traceback:
	[C]: in method 'shadow'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: lit_at: soft must be a finite width of 0 or more, got NaN
stack traceback:
	[C]: in method 'lit_at'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: lit: soft must be a finite width of 0 or more, got inf
stack traceback:
	[C]: in method 'lit'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: shadow: soft must be a finite width of 0 or more, got inf
stack traceback:
	[C]: in method 'shadow'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: lit_at: soft must be a finite width of 0 or more, got inf
stack traceback:
	[C]: in method 'lit_at'
	[string "chunk 9"]:1: in function <[string "chunk 9"]:1>
	[C]: in global 'pcall'
	[string "chunk 9"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

ok · chunk 9 (0.06 s to compute)
```

## silhouette haze

```text
<binaries>/easel-default do fh\=form\{\{s,dist\=.9\}\}\;print\(fh:silhouette\(\):area\(\),fh:silhouette\{soft\=.4\}:area\(\),fh:silhouette\{haze\=10\}:area\(\)\)\;for\ x\=435,445\ do\ print\(x,fh:silhouette\(\):at\(x,100\),fh:silhouette\{haze\=10\}:at\(x,100\)\)end
```

Exit: 0. Elapsed: 1.546 seconds.

```text
2517.3609110492366	2517.3609110492366	2517.3682388602397
435	1.0	1.0
436	1.0	1.0
437	1.0	1.0
438	1.0	1.0
439	1.0	1.0
440	0.5	0.5
441	0.0	0.0
442	0.0	0.0
443	0.0	0.0
444	0.0	0.0
445	0.0	0.0
ok · chunk 10 (1.52 s to compute)
```

## world variants

```text
<binaries>/easel-default do w\=world\{view\=\{300,20,400,160\},horizon\=80,eye\=1.6,fov\=45,ground\=function\(x,z\)return\ x\*.05\ end,water\=\{level\=0\},sun\=\{azimuth\=-120,elevation\=35\},visibility\=100,backdrop\=500\}\;v\=w:view\(\)\;w2\=world\{view\=\{300,20,400,160\},horizon\=60,eye\=2,fov\=60,sun\=\{azimuth\=80,elevation\=15\},visibility\=10,backdrop\=50\}\;v2\=w2:view\(\)\;print\(w,w2\)\;print\(table.unpack\(w:project\(0,0,10\)\)\)\;print\(table.unpack\(w2:project\(0,0,10\)\)\)\;print\(table.unpack\(w:project\(0,0,10\)\)\)\;print\(w:shadow_angle\(500,120\),w2:shadow_angle\(500,120\),w:aerial\(20\),w2:aerial\(20\),w:aerial\(20\)\)\;print\(v:water\(\):area\(\),v:land\(\):area\(\),v:sky\(\):area\(\),w:is_water\(-2,10\),w:is_water\(2,10\),w:ground_at\(-2,10\),w:ground_at\(2,10\)\)
```

Exit: 0. Elapsed: 0.271 seconds.

```text
world(horizon y 80, eye 1.6 m, sun az -120° el 35°, 0 bodies)	world(horizon y 60, eye 2 m, sun az 80° el 15°, 0 bodies)
500.0	157.25483703613281
500.0	129.28202819824219
500.0	157.25483703613281
-0.097524888813495636	3.1110615730285645	0.18126922845840454	0.86466473340988159	0.18126922845840454
19999.9984105429	20991.664998398985	23008.331504795395	true	false	-0.10002210736274719	0.10002211481332779
ok · chunk 11 (0.25 s to compute)
```

## proxy shadow alone

```text
<binaries>/easel-default do sp\=w:spot_at\(-1,12\)\;solid\=body.block\(sp:p\(0,.5,0\),sp:size\(1,1,1\),sp:m\(.05\)\)\;wp,proxy\=w:proxy\(sp,solid\)\;vp\=wp:view\(\)\;wv,visible\=w:place\(sp,solid\)\;vv\=wv:view\(\)\;print\(\"empty\",v:shadows\(\):area\(\),v:bodies_mask\(\):area\(\)\)\;print\(\"proxy\",vp:shadows\(\):area\(\),vp:bodies_mask\(\):area\(\),vp:reflections\(\):area\(\),vp:part\(proxy\)\)\;print\(\"placed\",vv:shadows\(\):area\(\),vv:bodies_mask\(\):area\(\),vv:part\(visible\)\)\;print\(\"old\",v:shadows\(\):area\(\),v:bodies_mask\(\):area\(\)\)
```

Exit: 0. Elapsed: 0.647 seconds.

```text
empty	0.0	0.0
proxy	554.28623409198428	0.0	1640.104036322906	0
placed	402.3762654797714	1629.1665371921404	1
old	0.0	0.0
ok · chunk 12 (0.63 s to compute)
```

## selectors immutable

```text
<binaries>/easel-default do fparts\=form\{\{s\},\{k\}\}\;m1\=fparts:parts_mask\{1\}\;print\(m1:area\(\),fparts:parts_mask\{2\}:area\(\),fparts:parts_mask\{1,2\}:area\(\),m1:area\(\)\)\;wb,n\=wv:place\(w:spot_at\(1,12\),solid\)\;vb\=wb:view\(\)\;old\=vb:bodies_mask\(1\)\;print\(old:area\(\),vb:bodies_mask\(2\):area\(\),vb:bodies_mask\{1,2\}:area\(\),old:area\(\)\)
```

Exit: 0. Elapsed: 0.226 seconds.

```text
1657.2915349569664	899.30548408517552	2556.5970190421422	1657.2915349569664
1629.1665371921404	0.0	1629.1665371921404	1629.1665371921404
ok · chunk 13 (0.20 s to compute)
```

## depth soft shares

```text
<binaries>/easel-default do near\=rect\(440,90,60,50\):map\(function\(\)return\ .25\ end\)\;far\=rect\(440,90,60,50\):map\(function\(\)return\ .5\ end\)\;wl\=w:layer\(\"near\",near,3\):layer\(\"far\",far,6\)\;vl\=wl:view\(\)\;for\ _,a\ in\ ipairs\(vl:seen\(450,100\)\)do\ print\(a.what,a.layer,a.depth,a.share\)end\;print\(vl:visible\(\"near\"\):at\(450,100\),vl:visible\(\"far\"\):at\(450,100\),vl:front\(\"far\"\):at\(450,100\),vl:behind\(\"near\"\):at\(450,100\),vl:at_depth\(5\):at\(450,100\),vl:between\(2,7\):at\(450,100\)\)
```

Exit: 0. Elapsed: 0.231 seconds.

```text
layer	near	3.0	0.25
layer	far	6.0	0.375
water	nil	38.229206085205078	0.375
0.25	0.375	0.125	0.75	0.75	0.625
ok · chunk 14 (0.21 s to compute)
```

## reserved names

```text
<binaries>/easel-default do for\ _,name\ in\ ipairs\{\"ground\",\"land\",\"water\",\"surface\",\"sky\",\"bodies\",\"layers\"\}do\ print\(name,pcall\(function\(\)w:layer\(name,rect\(400,100,10,10\),3\)end\)\)end\;print\(v:visible\(\"land\"\):area\(\),v:visible\(\"ground\"\):area\(\),v:visible\(\"surface\"\):area\(\),v:land\(\):area\(\)+v:water\(\):area\(\),vl:visible\(\"layers\"\):area\(\),vb:visible\(\"bodies\"\):area\(\)\)
```

Exit: 0. Elapsed: 0.165 seconds.

```text
ground	false	runtime error: layer name "ground" is taken (it names a part of the world)
stack traceback:
	[C]: in method 'layer'
	[string "chunk 15"]:1: in function <[string "chunk 15"]:1>
	[C]: in global 'pcall'
	[string "chunk 15"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

land	false	runtime error: layer name "land" is taken (it names a part of the world)
stack traceback:
	[C]: in method 'layer'
	[string "chunk 15"]:1: in function <[string "chunk 15"]:1>
	[C]: in global 'pcall'
	[string "chunk 15"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

water	false	runtime error: layer name "water" is taken (it names a part of the world)
stack traceback:
	[C]: in method 'layer'
	[string "chunk 15"]:1: in function <[string "chunk 15"]:1>
	[C]: in global 'pcall'
	[string "chunk 15"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

surface	false	runtime error: layer name "surface" is taken (it names a part of the world)
stack traceback:
	[C]: in method 'layer'
	[string "chunk 15"]:1: in function <[string "chunk 15"]:1>
	[C]: in global 'pcall'
	[string "chunk 15"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

sky	false	runtime error: layer name "sky" is taken (it names a part of the world)
stack traceback:
	[C]: in method 'layer'
	[string "chunk 15"]:1: in function <[string "chunk 15"]:1>
	[C]: in global 'pcall'
	[string "chunk 15"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

bodies	false	runtime error: layer name "bodies" is taken (it names a part of the world)
stack traceback:
	[C]: in method 'layer'
	[string "chunk 15"]:1: in function <[string "chunk 15"]:1>
	[C]: in global 'pcall'
	[string "chunk 15"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

layers	false	runtime error: layer name "layers" is taken (it names a part of the world)
stack traceback:
	[C]: in method 'layer'
	[string "chunk 15"]:1: in function <[string "chunk 15"]:1>
	[C]: in global 'pcall'
	[string "chunk 15"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

20991.664998398985	20991.664998398985	40991.663408941888	40991.66340894188	39999.9968210858	1625.7702701009405
ok · chunk 15 (0.15 s to compute)
```

## shadow defaults

```text
<binaries>/easel-default do print\(vv:cast_shadow\(\):area\(\),vv:cast_shadow\{soft\=1\}:area\(\),vv:contact_shadow\(\):area\(\),vv:contact_shadow\{reach\=.4\}:area\(\),vv:contact\(\):area\(\),vv:contact\(.25\):area\(\)\)\;print\(vl:cast_shadow\{from\=\"near\"\}:area\(\),vl:contact_shadow\{from\=\"near\"\}:area\(\)\)
```

Exit: 0. Elapsed: 0.367 seconds.

```text
410.12930402539342	410.12930402539342	367.47334849715054	367.47334849715054	284.00198810577569	284.00198810577569
0.0	0.0
ok · chunk 16 (0.35 s to compute)
```

## at forms

```text
<binaries>/easel-default do spot\=w:spot_at\(0,10\)\;pt\=w:project\(0,0,10\)\;print\(vl:at_depth\(10\):area\(\),vl:at_depth\(spot\):area\(\),vl:at_depth\(pt\):area\(\)\)\;for\ _,d\ in\ ipairs\{10,spot,pt\}do\ work\(rect\(400,100,100,40\),\{pile\=p,coverage\=.1,at\=d,view\=vl\}\)end
```

Exit: 0. Elapsed: 0.143 seconds.

```text
20199.99839464833	20199.99839464833	20199.99839464833
ok · chunk 17 (0.11 s to compute)
```

## nested mask selectors

```text
<binaries>/easel-default do box\=rect\(500,100,20,20\)\;for\ _,opts\ in\ ipairs\{\{visible\=\{box,\"water\"\}\},\{visible\=\{\{box\},\"water\"\}\},\{behind\=\{box,\"water\"\}\},\{behind\=\{\{box\},\"water\"\}\}\}do\ opts.pile\=p\;opts.coverage\=.01\;opts.view\=v\;print\(pcall\(function\(\)work\(rect\(500,100,20,20\),opts\)end\)\)end\;print\(pcall\(function\(\)v:front\(box\)end\)\)\;print\(pcall\(function\(\)v:behind\(box\)end\)\)
```

Exit: 0. Elapsed: 0.042 seconds.

```text
true
true
true
false	runtime error: want a body number, a layer name, "ground", "water", "sky" or a list, got userdata
stack traceback:
	[C]: in global 'work'
	[string "chunk 18"]:1: in function <[string "chunk 18"]:1>
	[C]: in global 'pcall'
	[string "chunk 18"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: want a body number, a layer name, "ground", "water", "sky" or a list, got userdata
stack traceback:
	[C]: in method 'front'
	[string "chunk 18"]:1: in function <[string "chunk 18"]:1>
	[C]: in global 'pcall'
	[string "chunk 18"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

false	runtime error: want a body number, a layer name, "ground", "water", "sky" or a list, got userdata
stack traceback:
	[C]: in method 'behind'
	[string "chunk 18"]:1: in function <[string "chunk 18"]:1>
	[C]: in global 'pcall'
	[string "chunk 18"]:1: in main chunk
	[C]: in upvalue 'pcall'
	[string "prelude.lua"]:362: in function <[string "prelude.lua"]:361>

ok · chunk 18 (0.02 s to compute)
```

## reference rollback

```text
<binaries>/easel-default do retained\=f\;print\(retained:parts_mask\{1\}:area\(\)\)
```

Exit: 0. Elapsed: 0.023 seconds.

```text
2517.3609110492366
ok · chunk 19 (0.00 s to compute)
```

## reference failed chunk

```text
<binaries>/easel-default do newref\=form\{\{k\}\}\;error\(\"reference\ rollback\"\)
```

Exit: 1. Elapsed: 0.022 seconds.

```text
runtime error: [string "chunk 20"]:1: reference rollback
(the chunk failed and changed nothing)
```

## reference retained

```text
<binaries>/easel-default do print\(newref\=\=nil,retained:parts_mask\{1\}:area\(\)\)
```

Exit: 0. Elapsed: 0.023 seconds.

```text
true	2517.3609110492366
ok · chunk 20 (0.00 s to compute)
```

## space check

```text
<binaries>/easel-default check
```

Exit: 0. Elapsed: 2.334 seconds.

```text
replay matches the live canvas exactly (20 chunks, 2.3s)
```

## space close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.092 seconds.

```text
the live canvas is in <fixture>/space/out/easel/space/live.png
closed; the session is in <fixture>/space/paintings/lua/space.lua
```

## space reopen extras

```text
<binaries>/easel-default open space
```

Exit: 0. Elapsed: 2.148 seconds.

```text
resuming chunk 1/20
resumed chunk 1/20 0.07s
resuming chunk 2/20
resumed chunk 2/20 0.01s
resuming chunk 3/20
resumed chunk 3/20 0.00s
resuming chunk 4/20
resumed chunk 4/20 0.00s
resuming chunk 5/20
resumed chunk 5/20 0.00s
resuming chunk 6/20
resumed chunk 6/20 0.00s
resuming chunk 7/20
resumed chunk 7/20 0.04s
resuming chunk 8/20
resumed chunk 8/20 0.00s
resuming chunk 9/20
resumed chunk 9/20 0.01s
resuming chunk 10/20
resumed chunk 10/20 0.21s
resuming chunk 11/20
resumed chunk 11/20 0.17s
resuming chunk 12/20
resumed chunk 12/20 0.57s
resuming chunk 13/20
resumed chunk 13/20 0.21s
resuming chunk 14/20
resumed chunk 14/20 0.21s
resuming chunk 15/20
resumed chunk 15/20 0.16s
resuming chunk 16/20
resumed chunk 16/20 0.34s
resuming chunk 17/20
resumed chunk 17/20 0.06s
resuming chunk 18/20
resumed chunk 18/20 0.01s
resuming chunk 19/20
resumed chunk 19/20 0.00s
resuming chunk 20/20
resumed chunk 20/20 0.00s
resumed 20 chunks from <fixture>/space/paintings/lua/space.lua
easel "space" open: 20 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## space reopen extras

```text
<binaries>/easel-default open space
```

Exit: 0. Elapsed: 1.522 seconds.

```text
resuming chunk 1/20
resumed chunk 1/20 0.06s
resuming chunk 2/20
resumed chunk 2/20 0.01s
resuming chunk 3/20
resumed chunk 3/20 0.00s
resuming chunk 4/20
resumed chunk 4/20 0.00s
resuming chunk 5/20
resumed chunk 5/20 0.00s
resuming chunk 6/20
resumed chunk 6/20 0.00s
resuming chunk 7/20
resumed chunk 7/20 0.03s
resuming chunk 8/20
resumed chunk 8/20 0.00s
resuming chunk 9/20
resumed chunk 9/20 0.01s
resuming chunk 10/20
resumed chunk 10/20 0.14s
resuming chunk 11/20
resumed chunk 11/20 0.14s
resuming chunk 12/20
resumed chunk 12/20 0.37s
resuming chunk 13/20
resumed chunk 13/20 0.15s
resuming chunk 14/20
resumed chunk 14/20 0.16s
resuming chunk 15/20
resumed chunk 15/20 0.11s
resuming chunk 16/20
resumed chunk 16/20 0.24s
resuming chunk 17/20
resumed chunk 17/20 0.05s
resuming chunk 18/20
resumed chunk 18/20 0.01s
resuming chunk 19/20
resumed chunk 19/20 0.00s
resuming chunk 20/20
resumed chunk 20/20 0.00s
resumed 20 chunks from <fixture>/space/paintings/lua/space.lua
easel "space" open: 20 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## explicitly lit soft boundary

```text
<binaries>/easel-default do fl\=form\{\{s\},light\=\{from\=\{-1,-.7\},front\=.5\}\}\;local\ hard\=fl:lit\{soft\=0\}\;local\ def\=fl:lit\(\)\;local\ explicit\=fl:lit\{soft\=.12\}\;local\ bad\=0\;local\ diff\=0\;for\ y\=80,120,.5\ do\ for\ x\=360,440,.5\ do\ local\ a\=hard:at\(x,y\)\;if\ a\~\=0\ and\ a\~\=1\ then\ bad\=bad+1\ end\;diff\=diff+math.abs\(def:at\(x,y\)-explicit:at\(x,y\)\)end\ end\;print\(\"binary\ nonbinary\",bad,\"default\ difference\",diff\)
```

Exit: 0. Elapsed: 0.029 seconds.

```text
binary nonbinary	390	default difference	0.0
ok · chunk 21 (0.01 s to compute)
```

## haze precise edge

```text
<binaries>/easel-default do local\ a\=fh:silhouette\{soft\=.4\}\;local\ b\=fh:silhouette\{soft\=.4,haze\=40\}\;for\ x\=439,441,.1\ do\ print\(x,a:at\(x,100\),b:at\(x,100\)\)end
```

Exit: 0. Elapsed: 0.027 seconds.

```text
439.0	1.0	1.0
439.1	1.0	1.0
439.20000000000005	1.0	1.0
439.30000000000007	1.0	1.0
439.40000000000009	1.0	0.99999749660491943
439.50000000000011	1.0	0.99998748302459717
439.60000000000014	1.0	0.99997752904891968
439.70000000000016	1.0	0.99996751546859741
439.80000000000018	0.97998046875	0.97994047403335571
439.9000000000002	0.739990234375	0.73997020721435547
440.00000000000023	0.5	0.5
440.10000000000025	0.2598876953125	0.25990772247314453
440.20000000000027	0.0198974609375	0.019937455654144287
440.3000000000003	0.0	3.2515916245756671e-05
440.40000000000032	0.0	2.2506714230985381e-05
440.50000000000034	0.0	1.2502599929575808e-05
440.60000000000036	0.0	2.4984838091768324e-06
440.70000000000039	0.0	0.0
440.80000000000041	0.0	0.0
440.90000000000043	0.0	0.0
ok · chunk 22 (0.01 s to compute)
```

## two opaque foregrounds

```text
<binaries>/easel-default do wa\=w:layer\(\"one\",rect\(400,80,30,60\),3\):layer\(\"two\",rect\(480,80,30,60\),4\)\;va\=wa:view\(\)\;print\(va:behind\{\"one\",\"two\"\}:at\(410,100\),va:behind\{\"one\",\"two\"\}:at\(490,100\),va:at_depth\(5\):at\(410,100\),va:at_depth\(5\):at\(490,100\)\)\;work\(rect\(390,70,130,80\),\{pile\=p,coverage\=.1,behind\=\{\"one\",\"two\"\},view\=va\}\)\;work\(rect\(390,70,130,80\),\{pile\=p,coverage\=.1,at\=5,view\=va\}\)
```

Exit: 0. Elapsed: 0.137 seconds.

```text
0.0	0.0	0.0	0.0
ok · chunk 23 (0.12 s to compute)
```

## fixed visible accepted

```text
<binaries>/easel-default do local\ n\=0\;for\ i\=1,100000000\ do\ n\=n+i\ end\;work\(rect\(390,70,130,80\),\{pile\=p,coverage\=.1,visible\=\"one\",view\=va\}\)\;print\(\"first\ visible\",va:visible\(\"one\"\):area\(\)\)
```

Exit: 0. Elapsed: 0.416 seconds.

```text
first visible	1799.999856948861
ok · chunk 24 (0.40 s to compute)
```

## queued visible replacement

```text
<binaries>/easel-default do w2:view\(\)\;print\(\"second\ default\ changed\"\)
```

Exit: 0. Elapsed: 0.326 seconds.

```text
second default changed
ok · chunk 25 (0.00 s to compute)
```

## fixed behind accepted

```text
<binaries>/easel-default do local\ n\=0\;for\ i\=1,100000000\ do\ n\=n+i\ end\;work\(rect\(390,70,130,80\),\{pile\=p,coverage\=.1,behind\=\"one\",view\=va\}\)\;print\(\"first\ behind\",va:visible\(\"one\"\):area\(\)\)
```

Exit: 0. Elapsed: 0.425 seconds.

```text
first behind	1799.999856948861
ok · chunk 26 (0.41 s to compute)
```

## queued behind replacement

```text
<binaries>/easel-default do w2:view\(\)\;print\(\"second\ default\ changed\"\)
```

Exit: 0. Elapsed: 0.334 seconds.

```text
second default changed
ok · chunk 27 (0.00 s to compute)
```

## fixed at accepted

```text
<binaries>/easel-default do local\ n\=0\;for\ i\=1,100000000\ do\ n\=n+i\ end\;work\(rect\(390,70,130,80\),\{pile\=p,coverage\=.1,at\=5,view\=va\}\)\;print\(\"first\ at\",va:visible\(\"one\"\):area\(\)\)
```

Exit: 0. Elapsed: 0.424 seconds.

```text
first at	1799.999856948861
ok · chunk 28 (0.41 s to compute)
```

## queued at replacement

```text
<binaries>/easel-default do w2:view\(\)\;print\(\"second\ default\ changed\"\)
```

Exit: 0. Elapsed: 0.333 seconds.

```text
second default changed
ok · chunk 29 (0.00 s to compute)
```

## view alone does not constrain

```text
<binaries>/easel-default do work\(rect\(600,100,10,10\),\{pile\=p,coverage\=.1,view\=va\}\)\;print\(\"view-only\ accepted\ away\ from\ layers\"\)
```

Exit: 0. Elapsed: 0.017 seconds.

```text
view-only accepted away from layers
ok · chunk 30 (0.00 s to compute)
```

## channel inline

```text
<binaries>/easel-default do channelForm\=form\{\{body.ellipsoid\(\{400,100,0\},\{20,20,10\}\)\}\}\;print\(channelForm:parts_mask\{1\}:area\(\)\)
```

Exit: 0. Elapsed: 0.018 seconds.

```text
1255.5554557729711
ok · chunk 31 (0.00 s to compute)
```

## channel file

```text
<binaries>/easel-default do <fixture>/channel.lua
```

Exit: 1. Elapsed: 0.012 seconds.

```text
syntax error: [string "chunk 32"]:1: unexpected symbol near '/'
(the chunk failed and changed nothing)
```

## channel stdin

```text
<binaries>/easel-default do -
```

Exit: 0. Elapsed: 0.018 seconds.

```text
1255.5554557729711
ok · chunk 32 (0.00 s to compute)
```

## accepted file retained

```text
<binaries>/easel-default do <fixture>/channel.lua
```

Exit: 1. Elapsed: 0.008 seconds.

```text
syntax error: [string "chunk 33"]:1: unexpected symbol near '/'
(the chunk failed and changed nothing)
```

## queue occupying command

```text
<binaries>/easel-default do local\ n\=0\;for\ i\=1,100000000\ do\ n\=n+i\ end\;print\(\"occupier\ done\"\)
```

Exit: 0. Elapsed: 0.411 seconds.

```text
occupier done
ok · chunk 33 (0.39 s to compute)
```

## queued disconnected state

```text
<binaries>/easel-default do print\(\"queued\ nil\",queuedReference\=\=nil\)
```

Exit: 0. Elapsed: 0.017 seconds.

```text
queued nil	true
ok · chunk 34 (0.00 s to compute)
```

## running disconnected state

```text
<binaries>/easel-default do print\(\"running\ retained\",runningReference\~\=nil\)
```

Exit: 0. Elapsed: 0.309 seconds.

```text
running retained	true
ok · chunk 36 (0.00 s to compute)
```

## space extras close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.089 seconds.

```text
the live canvas is in <fixture>/space/out/easel/space/live.png
closed; the session is in <fixture>/space/paintings/lua/space.lua
```

## missing reference session

```text
<binaries>/easel-default do world\{\}
```

Exit: 1. Elapsed: 0.004 seconds.

```text
no session: easel open <name> first (or pass -s <name>)
```

## edit open before

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.113 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## edit canvas before

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}
```

Exit: 0. Elapsed: 0.083 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## retained reference before

```text
<binaries>/easel-default do reference\=world\{\}
```

Exit: 0. Elapsed: 0.02 seconds.

```text
ok · chunk 2 (0.00 s to compute)
```

## edit before rejected

```text
<binaries>/easel-default do world\{\}
```

Exit: 1. Elapsed: 0.003 seconds.

```text
session integrity: <fixture>/space-edit-before/paintings/lua/painting.lua differs from the log the session wrote. The easel goes on only from the log it wrote; nothing ran.
```

## close edited before

```text
<binaries>/easel-default close
```

Exit: 1. Elapsed: 0.003 seconds.

```text
session integrity: <fixture>/space-edit-before/paintings/lua/painting.lua differs from the log the session wrote. The easel goes on only from the log it wrote; nothing ran.
```

## edit open during

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.106 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## edit canvas during

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}
```

Exit: 0. Elapsed: 0.082 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## retained reference during

```text
<binaries>/easel-default do reference\=world\{\}
```

Exit: 0. Elapsed: 0.02 seconds.

```text
ok · chunk 2 (0.00 s to compute)
```

## edit during accepted

```text
<binaries>/easel-default do local\ n\=0\;for\ i\=1,100000000\ do\ n\=n+i\ end\;newreference\=world\{\}
```

Exit: 1. Elapsed: 0.396 seconds.

```text
session integrity: <fixture>/space-edit-during/paintings/lua/painting.lua differs from the log the session wrote. The easel goes on only from the log it wrote; nothing ran.
```

## edit during restricted

```text
<binaries>/easel-default do print\(reference\)
```

Exit: 1. Elapsed: 0.004 seconds.

```text
session integrity: <fixture>/space-edit-during/paintings/lua/painting.lua differs from the log the session wrote. The easel goes on only from the log it wrote; nothing ran.
```

## close edited during

```text
<binaries>/easel-default close
```

Exit: 1. Elapsed: 0.003 seconds.

```text
session integrity: <fixture>/space-edit-during/paintings/lua/painting.lua differs from the log the session wrote. The easel goes on only from the log it wrote; nothing ran.
```

## space final reopen

```text
<binaries>/easel-default open space
```

Exit: 0. Elapsed: 3.761 seconds.

```text
resuming chunk 1/36
resumed chunk 1/36 0.07s
resuming chunk 2/36
resumed chunk 2/36 0.01s
resuming chunk 3/36
resumed chunk 3/36 0.00s
resuming chunk 4/36
resumed chunk 4/36 0.00s
resuming chunk 5/36
resumed chunk 5/36 0.00s
resuming chunk 6/36
resumed chunk 6/36 0.00s
resuming chunk 7/36
resumed chunk 7/36 0.03s
resuming chunk 8/36
resumed chunk 8/36 0.00s
resuming chunk 9/36
resumed chunk 9/36 0.01s
resuming chunk 10/36
resumed chunk 10/36 0.14s
resuming chunk 11/36
resumed chunk 11/36 0.15s
resuming chunk 12/36
resumed chunk 12/36 0.36s
resuming chunk 13/36
resumed chunk 13/36 0.14s
resuming chunk 14/36
resumed chunk 14/36 0.17s
resuming chunk 15/36
resumed chunk 15/36 0.11s
resuming chunk 16/36
resumed chunk 16/36 0.25s
resuming chunk 17/36
resumed chunk 17/36 0.05s
resuming chunk 18/36
resumed chunk 18/36 0.01s
resuming chunk 19/36
resumed chunk 19/36 0.00s
resuming chunk 20/36
resumed chunk 20/36 0.00s
resuming chunk 21/36
resumed chunk 21/36 0.01s
resuming chunk 22/36
resumed chunk 22/36 0.01s
resuming chunk 23/36
resumed chunk 23/36 0.12s
resuming chunk 24/36
resumed chunk 24/36 0.40s
resuming chunk 25/36
resumed chunk 25/36 0.00s
resuming chunk 26/36
resumed chunk 26/36 0.41s
resuming chunk 27/36
resumed chunk 27/36 0.00s
resuming chunk 28/36
resumed chunk 28/36 0.41s
resuming chunk 29/36
resumed chunk 29/36 0.00s
resuming chunk 30/36
resumed chunk 30/36 0.00s
resuming chunk 31/36
resumed chunk 31/36 0.00s
resuming chunk 32/36
resumed chunk 32/36 0.00s
resuming chunk 33/36
resumed chunk 33/36 0.40s
resuming chunk 34/36
resumed chunk 34/36 0.00s
resuming chunk 35/36
resumed chunk 35/36 0.40s
resuming chunk 36/36
resumed chunk 36/36 0.00s
resumed 36 chunks from <fixture>/space/paintings/lua/space.lua
easel "space" open: 36 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## channel corrected file

```text
<binaries>/easel-default do -f <fixture>/channel.lua
```

Exit: 0. Elapsed: 0.037 seconds.

```text
1255.5554557729711
ok · chunk 37 (0.00 s to compute)
```

## accepted corrected file

```text
<binaries>/easel-default do -f <fixture>/channel.lua
```

Exit: 0. Elapsed: 0.419 seconds.

```text
1255.5554557729711
ok · chunk 38 (0.40 s to compute)
```

## hard light direct samples

```text
<binaries>/easel-default do local\ bad\=0\;for\ y\=80,120\ do\ for\ x\=360,440\ do\ local\ a\=fl:lit_at\(x,y,0\)\;if\ a\~\=0\ and\ a\~\=1\ then\ bad\=bad+1\ end\ end\ end\;print\(\"nonbinary\",bad\)
```

Exit: 0. Elapsed: 0.018 seconds.

```text
nonbinary	0
ok · chunk 39 (0.00 s to compute)
```

## paint field fall

```text
<binaries>/easel-default do work\(rect\(370,85,60,30\),\{pile\=p,seed\=19,hand\=\"detail\",coverage\=.8,angle\=f:field\(\"fall\"\)\}\)
```

Exit: 0. Elapsed: 0.058 seconds.

```text
ok · chunk 40 (0.04 s to compute)
```

## save field fall

```text
<binaries>/easel-default save <fixture>/field-fall.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/field-fall.png
```

## paint field across

```text
<binaries>/easel-default do work\(rect\(370,85,60,30\),\{pile\=p,seed\=19,hand\=\"detail\",coverage\=.8,angle\=f:field\(\"across\"\)\}\)
```

Exit: 0. Elapsed: 0.061 seconds.

```text
ok · chunk 41 (0.04 s to compute)
```

## save field across

```text
<binaries>/easel-default save <fixture>/field-across.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/field-across.png
```

## paint field edge

```text
<binaries>/easel-default do work\(rect\(370,85,60,30\),\{pile\=p,seed\=19,hand\=\"detail\",coverage\=.8,angle\=f:field\(\"edge\"\)\}\)
```

Exit: 0. Elapsed: 0.053 seconds.

```text
ok · chunk 42 (0.04 s to compute)
```

## save field edge

```text
<binaries>/easel-default save <fixture>/field-edge.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/field-edge.png
```

## space final close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.101 seconds.

```text
the live canvas is in <fixture>/space/out/easel/space/live.png
closed; the session is in <fixture>/space/paintings/lua/space.lua
```

## restored reference before

```text
<binaries>/easel-default do print\(reference\~\=nil,newreference\~\=nil\)
```

Exit: 0. Elapsed: 0.016 seconds.

```text
true	false
ok · chunk 3 (0.00 s to compute)
```

## repaired close before

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.072 seconds.

```text
the live canvas is in <fixture>/space-edit-before/out/easel/painting/live.png
closed; the session is in <fixture>/space-edit-before/paintings/lua/painting.lua
```

## restored reference during

```text
<binaries>/easel-default do print\(reference\~\=nil,newreference\~\=nil\)
```

Exit: 1. Elapsed: 0.004 seconds.

```text
session integrity: uncommitted state; restart required
```

## repaired close during

```text
<binaries>/easel-default close
```

Exit: 1. Elapsed: 0.003 seconds.

```text
session integrity: uncommitted state; restart required
```

## kill open

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.109 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## kill setup

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}\;old\=world\{\}
```

Exit: 0. Elapsed: 0.079 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## killed reference compute

```text
<binaries>/easel-default do local\ n\=0\;for\ i\=1,1000000000\ do\ n\=n+i\ end\;lost\=world\{\}
```

Exit: 1. Elapsed: 0.205 seconds.

```text
the easel closed without answering "do"
```

## kill recover

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.112 seconds.

```text
resuming chunk 1/1
resumed chunk 1/1 0.06s
resumed 1 chunks from <fixture>/space-kill/paintings/lua/painting.lua
easel "painting" open: 1 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## kill retained

```text
<binaries>/easel-default do print\(old\~\=nil,lost\=\=nil\)
```

Exit: 0. Elapsed: 0.021 seconds.

```text
true	true
ok · chunk 2 (0.00 s to compute)
```

## kill close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.079 seconds.

```text
the live canvas is in <fixture>/space-kill/out/easel/painting/live.png
closed; the session is in <fixture>/space-kill/paintings/lua/painting.lua
```

## depth pixels open

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.113 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## depth pixels setup

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}\;p\=pile\{\{\"red\ earth\",1\}\}\;w\=world\{view\=\{300,20,400,160\},horizon\=80\}\;w\=w:layer\(\"foreground\",rect\(440,80,30,70\),3\)\;v\=w:view\(\)
```

Exit: 0. Elapsed: 0.086 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## depth before

```text
<binaries>/easel-default save <fixture>/depth-before.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/depth-before.png
```

## depth clip combined

```text
<binaries>/easel-default do work\(rect\(400,90,100,50\),\{pile\=p,hand\=\"broad\",coverage\=2,clip\=true,visible\=\"ground\",behind\=\"foreground\",at\=5,view\=v\}\)
```

Exit: 0. Elapsed: 0.157 seconds.

```text
ok · chunk 2 (0.13 s to compute)
```

## depth after

```text
<binaries>/easel-default save <fixture>/depth-after.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/depth-after.png
```

## depth pixels close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.107 seconds.

```text
the live canvas is in <fixture>/depth-pixels/out/easel/painting/live.png
closed; the session is in <fixture>/depth-pixels/paintings/lua/painting.lua
```

## Depth pixel comparison

For `depth clip combined`, decoded PNG differences at2400px: logical clip400,90,100,50 and foreground440,80,30,70 multiplied by2.4.

```json
{"changed": 18785, "outside_clip": 0, "inside_foreground": 0}
```

[Result](matrix-depth.png). Independent fall/across/edge recipes use the same blank canvas, pigment, seed and mask: [fall](matrix-field-fall.png), [across](matrix-field-across.png), [edge](matrix-field-edge.png). The images were inspected for stroke orientation.

## Independent direction fields

Three actual replay recipes began with the same blank canvas, ellipsoid, pigment, passage mask and seed. Only the form field changed. The fall image has mainly vertical marks; across and edge turn the marks laterally along their fields. All retain the requested red-earth pigment. The linked images were visually inspected.
