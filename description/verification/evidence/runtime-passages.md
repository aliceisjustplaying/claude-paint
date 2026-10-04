# Passages and time runtime evidence

Agent CLI run on 2026-10-04, source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`, preserved default release build. Isolated disposable sessions, no provider needed. Fixture uses aspect5 and mask rect(400,60,160,80) to bound computation. This is an explicitly recorded narrower fixture than the checklist's square canvas. All images have 2400×480 pixels.

## Open empty

```text
open empty
```

Exit 0; wall 0.108 seconds:

```text
easel "empty" open: 0 chunks · 2400px · no canvas yet

```

## No canvas wait

```text
do wait(1)
```

Exit 1; wall 0.004 seconds:

```text
runtime error: no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}
(the chunk failed and changed nothing)

```

## No canvas drying

```text
do drying(500,100)
```

Exit 1; wall 0.004 seconds:

```text
runtime error: no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}
(the chunk failed and changed nothing)

```

## No canvas passage

```text
do work(nil,{})
```

Exit 1; wall 0.003 seconds:

```text
runtime error: want a mask, got nil (make one with mask(fn), ellipse, poly, rect, below, above, ribbon, everywhere)
(the chunk failed and changed nothing)

```

## Close

```text
close
```

Exit 0; wall 0.015 seconds:

```text
closed; the session is in <isolated-root>/paintings/lua/empty.lua

```

## Open validation

```text
open validation
```

Exit 0; wall 0.111 seconds:

```text
easel "validation" open: 0 chunks · 2400px · no canvas yet

```

## Fixture validation

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.095 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save baseline

```text
save <isolated-root>/baseline.png
```

Exit 0; wall 0.028 seconds:

```text
<isolated-root>/baseline.png

```

## Validation 1

```text
do work(m,{pile=p,covrage=1})
```

Exit 1; wall 0.019 seconds:

```text
runtime error: work: unknown option "covrage" (options: hand, pile, tool, length, coverage, angle, angle_jitter, load_at, cut_in, pressure, orient, dips, blender, scrub, clip, threshold, ramps, shake, curve, cross, drift, tail, broken, swell, clump, order, mix_jitter, seed, ruler, load, hug, fill, visible, behind, at, view, edge)
(the chunk failed and changed nothing)

```

## Validation 2

```text
do work(m,{pile=p,hand="not-a-hand"})
```

Exit 1; wall 0.011 seconds:

```text
runtime error: hand "not-a-hand": broad, body, detail, hatch, glaze, scumble or blend
(the chunk failed and changed nothing)

```

## Validation 3

```text
do work(m,{pile=p,edge="soft",cut_in="round 2"})
```

Exit 1; wall 0.01 seconds:

```text
runtime error: work: edge= and cut_in= both say how the edge is made; give one
(the chunk failed and changed nothing)

```

## Validation 4

```text
do work(m,{})
```

Exit 1; wall 0.009 seconds:

```text
runtime error: work: needs a pile (p = pile{{"tube name", parts}, ...})
(the chunk failed and changed nothing)

```

## Validation 5

```text
do work(7,{pile=p})
```

Exit 1; wall 0.009 seconds:

```text
runtime error: want a mask, got integer (make one with mask(fn), ellipse, poly, rect, below, above, ribbon, everywhere)
(the chunk failed and changed nothing)

```

## Validation 6

```text
do work(m,{pile=p,freeze_time=true})
```

Exit 1; wall 0.009 seconds:

```text
runtime error: work: unknown option "freeze_time" (options: hand, pile, tool, length, coverage, angle, angle_jitter, load_at, cut_in, pressure, orient, dips, blender, scrub, clip, threshold, ramps, shake, curve, cross, drift, tail, broken, swell, clump, order, mix_jitter, seed, ruler, load, hug, fill, visible, behind, at, view, edge)
(the chunk failed and changed nothing)

```

## Validation 7

```text
do wait(-1)
```

Exit 1; wall 0.009 seconds:

```text
runtime error: wait(-1): want minutes from 0 to 5259600 (10 years); days are fine: wait(3 * 24 * 60)
(the chunk failed and changed nothing)

```

## Validation 8

```text
do wait(0/0)
```

Exit 1; wall 0.009 seconds:

```text
runtime error: wait(NaN): want minutes from 0 to 5259600 (10 years); days are fine: wait(3 * 24 * 60)
(the chunk failed and changed nothing)

```

## Validation 9

```text
do wait(5259601)
```

Exit 1; wall 0.009 seconds:

```text
runtime error: wait(5259601): want minutes from 0 to 5259600 (10 years); days are fine: wait(3 * 24 * 60)
(the chunk failed and changed nothing)

```

## Rollback clock before

```text
do print(wait(0))
```

Exit 0; wall 0.016 seconds:

```text
day 1, 09:00
ok · chunk 2 (0.00 s to compute)

```

## Malformed second operation

```text
do work(m,{pile=p,coverage=1,seed=7}); wait(90); marker=12; work(m,{pile=p,covrage=1})
```

Exit 1; wall 0.289 seconds:

```text
runtime error: work: unknown option "covrage" (options: hand, pile, tool, length, coverage, angle, angle_jitter, load_at, cut_in, pressure, orient, dips, blender, scrub, clip, threshold, ramps, shake, curve, cross, drift, tail, broken, swell, clump, order, mix_jitter, seed, ruler, load, hug, fill, visible, behind, at, view, edge)
(the chunk failed and changed nothing)

```

## Rollback clock after

```text
do print(wait(0),marker==nil)
```

Exit 0; wall 0.016 seconds:

```text
day 1, 09:00	true
ok · chunk 3 (0.00 s to compute)

```

## Save rollback

```text
save <isolated-root>/rollback.png
```

Exit 0; wall 0.027 seconds:

```text
<isolated-root>/rollback.png

```

PNG equality baseline/rollback: **true**.

## Wait arithmetic

```text
do print(wait(0)); print(wait(1)); print(wait(2)); print(wait(90)); print(wait(0))
```

Exit 0; wall 0.015 seconds:

```text
day 1, 09:00
day 1, 09:01
day 1, 09:03
day 1, 10:33
day 1, 10:33
ok · chunk 4 (0.00 s to compute)

```

## Idle baseline

```text
do r=rag{width=20};r:dip(1);print(wait(0),r.damp)
```

Exit 0; wall 0.015 seconds:

```text
day 1, 10:33	1.0
ok · chunk 5 (0.00 s to compute)

```

## Idle after two wall seconds

```text
do print(wait(0),r.damp)
```

Exit 0; wall 0.018 seconds:

```text
day 1, 10:33	1.0
ok · chunk 6 (0.00 s to compute)

```

## Damp after three painting minutes

```text
do print(wait(3),r.damp)
```

Exit 0; wall 0.016 seconds:

```text
day 1, 10:36	0.5
ok · chunk 7 (0.00 s to compute)

```

## Close

```text
close
```

Exit 0; wall 0.081 seconds:

```text
the live canvas is in <isolated-root>/out/easel/validation/live.png
closed; the session is in <isolated-root>/paintings/lua/validation.lua

```

## Open default

```text
open default
```

Exit 0; wall 0.115 seconds:

```text
easel "default" open: 0 chunks · 2400px · no canvas yet

```

## Fixture default

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.092 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save default-before

```text
save <isolated-root>/default-before.png
```

Exit 0; wall 0.028 seconds:

```text
<isolated-root>/default-before.png

```

## Hand default

```text
do print(work(m,{pile=p,coverage=0.5,seed=7}));print(wait(0))
```

Exit 0; wall 0.17 seconds:

```text

day 1, 09:01
ok · chunk 2 (0.15 s to compute)

```

## Save default

```text
save <isolated-root>/default.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/default.png

```

Changed pixels default-before→default: inside [400, 60, 560, 140] = **39483**, outside = **9686**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.085 seconds:

```text
the live canvas is in <isolated-root>/out/easel/default/live.png
closed; the session is in <isolated-root>/paintings/lua/default.lua

```

## Open body

```text
open body
```

Exit 0; wall 0.115 seconds:

```text
easel "body" open: 0 chunks · 2400px · no canvas yet

```

## Fixture body

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.096 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save body-before

```text
save <isolated-root>/body-before.png
```

Exit 0; wall 0.028 seconds:

```text
<isolated-root>/body-before.png

```

## Hand body

```text
do print(work(m,{pile=p,coverage=0.5,seed=7,hand="body"}));print(wait(0))
```

Exit 0; wall 0.173 seconds:

```text

day 1, 09:01
ok · chunk 2 (0.15 s to compute)

```

## Save body

```text
save <isolated-root>/body.png
```

Exit 0; wall 0.03 seconds:

```text
<isolated-root>/body.png

```

Changed pixels body-before→body: inside [400, 60, 560, 140] = **39483**, outside = **9686**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.09 seconds:

```text
the live canvas is in <isolated-root>/out/easel/body/live.png
closed; the session is in <isolated-root>/paintings/lua/body.lua

```

## Open broad

```text
open broad
```

Exit 0; wall 0.118 seconds:

```text
easel "broad" open: 0 chunks · 2400px · no canvas yet

```

## Fixture broad

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.097 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save broad-before

```text
save <isolated-root>/broad-before.png
```

Exit 0; wall 0.03 seconds:

```text
<isolated-root>/broad-before.png

```

## Hand broad

```text
do print(work(m,{pile=p,coverage=0.5,seed=7,hand="broad"}));print(wait(0))
```

Exit 0; wall 0.162 seconds:

```text

day 1, 09:00
ok · chunk 2 (0.14 s to compute)

```

## Save broad

```text
save <isolated-root>/broad.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/broad.png

```

Changed pixels broad-before→broad: inside [400, 60, 560, 140] = **44424**, outside = **9039**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.1 seconds:

```text
the live canvas is in <isolated-root>/out/easel/broad/live.png
closed; the session is in <isolated-root>/paintings/lua/broad.lua

```

## Open detail

```text
open detail
```

Exit 0; wall 0.115 seconds:

```text
easel "detail" open: 0 chunks · 2400px · no canvas yet

```

## Fixture detail

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.095 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save detail-before

```text
save <isolated-root>/detail-before.png
```

Exit 0; wall 0.036 seconds:

```text
<isolated-root>/detail-before.png

```

## Hand detail

```text
do print(work(m,{pile=p,coverage=0.5,seed=7,hand="detail"}));print(wait(0))
```

Exit 0; wall 0.161 seconds:

```text

day 1, 09:08
ok · chunk 2 (0.14 s to compute)

```

## Save detail

```text
save <isolated-root>/detail.png
```

Exit 0; wall 0.032 seconds:

```text
<isolated-root>/detail.png

```

Changed pixels detail-before→detail: inside [400, 60, 560, 140] = **54885**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.096 seconds:

```text
the live canvas is in <isolated-root>/out/easel/detail/live.png
closed; the session is in <isolated-root>/paintings/lua/detail.lua

```

## Open hatch

```text
open hatch
```

Exit 0; wall 0.116 seconds:

```text
easel "hatch" open: 0 chunks · 2400px · no canvas yet

```

## Fixture hatch

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.097 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save hatch-before

```text
save <isolated-root>/hatch-before.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/hatch-before.png

```

## Hand hatch

```text
do print(work(m,{pile=p,coverage=0.5,seed=7,hand="hatch"}));print(wait(0))
```

Exit 0; wall 0.14 seconds:

```text

day 1, 09:07
ok · chunk 2 (0.12 s to compute)

```

## Save hatch

```text
save <isolated-root>/hatch.png
```

Exit 0; wall 0.034 seconds:

```text
<isolated-root>/hatch.png

```

Changed pixels hatch-before→hatch: inside [400, 60, 560, 140] = **52202**, outside = **2387**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.098 seconds:

```text
the live canvas is in <isolated-root>/out/easel/hatch/live.png
closed; the session is in <isolated-root>/paintings/lua/hatch.lua

```

## Open glaze

```text
open glaze
```

Exit 0; wall 0.116 seconds:

```text
easel "glaze" open: 0 chunks · 2400px · no canvas yet

```

## Fixture glaze

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.099 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.09 s to compute)

```

## Save glaze-before

```text
save <isolated-root>/glaze-before.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/glaze-before.png

```

## Hand glaze

```text
do print(work(m,{pile=p,coverage=0.5,seed=7,hand="glaze"}));print(wait(0))
```

Exit 0; wall 0.081 seconds:

```text

day 1, 09:00
ok · chunk 2 (0.06 s to compute)

```

## Save glaze

```text
save <isolated-root>/glaze.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/glaze.png

```

Changed pixels glaze-before→glaze: inside [400, 60, 560, 140] = **12945**, outside = **7636**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.09 seconds:

```text
the live canvas is in <isolated-root>/out/easel/glaze/live.png
closed; the session is in <isolated-root>/paintings/lua/glaze.lua

```

## Open scumble

```text
open scumble
```

Exit 0; wall 0.107 seconds:

```text
easel "scumble" open: 0 chunks · 2400px · no canvas yet

```

## Fixture scumble

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.097 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save scumble-before

```text
save <isolated-root>/scumble-before.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/scumble-before.png

```

## Hand scumble

```text
do print(work(m,{pile=p,coverage=0.5,seed=7,hand="scumble"}));print(wait(0))
```

Exit 0; wall 0.77 seconds:

```text

day 1, 09:03
ok · chunk 2 (0.75 s to compute)

```

## Save scumble

```text
save <isolated-root>/scumble.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/scumble.png

```

Changed pixels scumble-before→scumble: inside [400, 60, 560, 140] = **62021**, outside = **8323**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.091 seconds:

```text
the live canvas is in <isolated-root>/out/easel/scumble/live.png
closed; the session is in <isolated-root>/paintings/lua/scumble.lua

```

PNG equality default/body: **true**.

## Open clip

```text
open clip
```

Exit 0; wall 0.108 seconds:

```text
easel "clip" open: 0 chunks · 2400px · no canvas yet

```

## Fixture clip

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.095 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save clip-before

```text
save <isolated-root>/clip-before.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/clip-before.png

```

## Variant clip

```text
do work(m,{pile=p,hand="glaze",seed=7,clip=true});print(wait(0),samples)
```

Exit 0; wall 0.581 seconds:

```text
day 1, 09:01	nil
ok · chunk 2 (0.56 s to compute)

```

## Save clip

```text
save <isolated-root>/clip.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/clip.png

```

Changed pixels clip-before→clip: inside [400, 60, 560, 140] = **73674**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.093 seconds:

```text
the live canvas is in <isolated-root>/out/easel/clip/live.png
closed; the session is in <isolated-root>/paintings/lua/clip.lua

```

## Open separate-clip

```text
open separate-clip
```

Exit 0; wall 0.116 seconds:

```text
easel "separate-clip" open: 0 chunks · 2400px · no canvas yet

```

## Fixture separate-clip

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.103 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.09 s to compute)

```

## Save separate-clip-before

```text
save <isolated-root>/separate-clip-before.png
```

Exit 0; wall 0.03 seconds:

```text
<isolated-root>/separate-clip-before.png

```

## Variant separate-clip

```text
do work(m,{pile=p,hand="glaze",seed=7,clip=rect(450,60,50,80)});print(wait(0),samples)
```

Exit 0; wall 0.307 seconds:

```text
day 1, 09:00	nil
ok · chunk 2 (0.28 s to compute)

```

## Save separate-clip

```text
save <isolated-root>/separate-clip.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/separate-clip.png

```

Changed pixels separate-clip-before→separate-clip: inside [450, 60, 500, 140] = **23040**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.09 seconds:

```text
the live canvas is in <isolated-root>/out/easel/separate-clip/live.png
closed; the session is in <isolated-root>/paintings/lua/separate-clip.lua

```

## Open found

```text
open found
```

Exit 0; wall 0.116 seconds:

```text
easel "found" open: 0 chunks · 2400px · no canvas yet

```

## Fixture found

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.099 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save found-before

```text
save <isolated-root>/found-before.png
```

Exit 0; wall 0.03 seconds:

```text
<isolated-root>/found-before.png

```

## Variant found

```text
do work(m,{pile=p,hand="glaze",seed=7,edge="found"});print(wait(0),samples)
```

Exit 0; wall 0.833 seconds:

```text
day 1, 09:01	nil
ok · chunk 2 (0.81 s to compute)

```

## Save found

```text
save <isolated-root>/found.png
```

Exit 0; wall 0.032 seconds:

```text
<isolated-root>/found.png

```

Changed pixels found-before→found: inside [400, 60, 560, 140] = **73673**, outside = **11477**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.094 seconds:

```text
the live canvas is in <isolated-root>/out/easel/found/live.png
closed; the session is in <isolated-root>/paintings/lua/found.lua

```

## Open lost

```text
open lost
```

Exit 0; wall 0.112 seconds:

```text
easel "lost" open: 0 chunks · 2400px · no canvas yet

```

## Fixture lost

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.113 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.10 s to compute)

```

## Save lost-before

```text
save <isolated-root>/lost-before.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/lost-before.png

```

## Variant lost

```text
do work(m,{pile=p,hand="glaze",seed=7,edge="lost"});print(wait(0),samples)
```

Exit 0; wall 1.179 seconds:

```text
day 1, 09:01	nil
ok · chunk 2 (1.16 s to compute)

```

## Save lost

```text
save <isolated-root>/lost.png
```

Exit 0; wall 0.034 seconds:

```text
<isolated-root>/lost.png

```

Changed pixels lost-before→lost: inside [400, 60, 560, 140] = **73665**, outside = **85615**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.101 seconds:

```text
the live canvas is in <isolated-root>/out/easel/lost/live.png
closed; the session is in <isolated-root>/paintings/lua/lost.lua

```

## Open sparse

```text
open sparse
```

Exit 0; wall 0.117 seconds:

```text
easel "sparse" open: 0 chunks · 2400px · no canvas yet

```

## Fixture sparse

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.094 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save sparse-before

```text
save <isolated-root>/sparse-before.png
```

Exit 0; wall 0.03 seconds:

```text
<isolated-root>/sparse-before.png

```

## Variant sparse

```text
do work(m,{pile=p,hand="glaze",seed=7,coverage=0.1,fill=false});print(wait(0),samples)
```

Exit 0; wall 0.025 seconds:

```text
day 1, 09:00	nil
ok · chunk 2 (0.00 s to compute)

```

## Save sparse

```text
save <isolated-root>/sparse.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/sparse.png

```

Changed pixels sparse-before→sparse: inside [400, 60, 560, 140] = **0**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.092 seconds:

```text
the live canvas is in <isolated-root>/out/easel/sparse/live.png
closed; the session is in <isolated-root>/paintings/lua/sparse.lua

```

## Open filled

```text
open filled
```

Exit 0; wall 0.107 seconds:

```text
easel "filled" open: 0 chunks · 2400px · no canvas yet

```

## Fixture filled

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.094 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save filled-before

```text
save <isolated-root>/filled-before.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/filled-before.png

```

## Variant filled

```text
do work(m,{pile=p,hand="glaze",seed=7,coverage=0.1,fill=true});print(wait(0),samples)
```

Exit 0; wall 0.029 seconds:

```text
day 1, 09:00	nil
ok · chunk 2 (0.00 s to compute)

```

## Save filled

```text
save <isolated-root>/filled.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/filled.png

```

Changed pixels filled-before→filled: inside [400, 60, 560, 140] = **0**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.107 seconds:

```text
the live canvas is in <isolated-root>/out/easel/filled/live.png
closed; the session is in <isolated-root>/paintings/lua/filled.lua

```

## Open field

```text
open field
```

Exit 0; wall 0.117 seconds:

```text
easel "field" open: 0 chunks · 2400px · no canvas yet

```

## Fixture field

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.096 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Save field-before

```text
save <isolated-root>/field-before.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/field-before.png

```

## Variant field

```text
do work(m,{pile=p,hand="glaze",seed=7,angle=function(x,y) samples=(samples or 0)+1;return x/1000 end,load_at=function(x,y)return x<480 and 0.1 or 0.8 end});print(wait(0),samples)
```

Exit 0; wall 0.337 seconds:

```text
day 1, 09:00	43834
ok · chunk 2 (0.31 s to compute)

```

## Save field

```text
save <isolated-root>/field.png
```

Exit 0; wall 0.033 seconds:

```text
<isolated-root>/field.png

```

Changed pixels field-before→field: inside [400, 60, 560, 140] = **62405**, outside = **53196**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.108 seconds:

```text
the live canvas is in <isolated-root>/out/easel/field/live.png
closed; the session is in <isolated-root>/paintings/lua/field.lua

```

## Open touches

```text
open touches
```

Exit 0; wall 0.117 seconds:

```text
easel "touches" open: 0 chunks · 2400px · no canvas yet

```

## Fixture touches

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.089 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.07 s to compute)

```

## Stipple

```text
do stipple(m,{pile=p,width=4,coverage=1,seed=7});print(wait(0))
```

Exit 0; wall 0.091 seconds:

```text
day 1, 09:12
ok · chunk 2 (0.07 s to compute)

```

## Save stipple

```text
save <isolated-root>/stipple.png
```

Exit 0; wall 0.032 seconds:

```text
<isolated-root>/stipple.png

```

## Lose

```text
do print(lose(m,{pile=p,where=1,every=3,seed=7}));print(wait(0))
```

Exit 0; wall 0.035 seconds:

```text
40
day 1, 09:14
ok · chunk 3 (0.02 s to compute)

```

## Save lose

```text
save <isolated-root>/lose.png
```

Exit 0; wall 0.031 seconds:

```text
<isolated-root>/lose.png

```

## Close

```text
close
```

Exit 0; wall 0.101 seconds:

```text
the live canvas is in <isolated-root>/out/easel/touches/live.png
closed; the session is in <isolated-root>/paintings/lua/touches.lua

```

## Open wet

```text
open wet
```

Exit 0; wall 0.108 seconds:

```text
easel "wet" open: 0 chunks · 2400px · no canvas yet

```

## Fixture wet

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.092 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Paint wet

```text
do work(m,{pile=p,hand="detail",coverage=2,clip=true,seed=7});print(drying(480,100),drying(50,50))
```

Exit 0; wall 0.522 seconds:

```text
open	dry
ok · chunk 2 (0.50 s to compute)

```

## Save wet-before

```text
save <isolated-root>/wet-before.png
```

Exit 0; wall 0.032 seconds:

```text
<isolated-root>/wet-before.png

```

## Blend wet

```text
do blend(rect(430,80,100,40));print(wait(0))
```

Exit 0; wall 0.446 seconds:

```text
day 1, 09:30
ok · chunk 3 (0.43 s to compute)

```

## Save wet-blend

```text
save <isolated-root>/wet-blend.png
```

Exit 0; wall 0.032 seconds:

```text
<isolated-root>/wet-blend.png

```

Changed pixels wet-before→wet-blend: inside [430, 80, 530, 120] = **14080**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

PNG equality wet-before/wet-blend: **false**.

## Replay wet

```text
check
```

Exit 0; wall 0.923 seconds:

```text
replay matches the live canvas exactly (3 chunks, 0.9s)

```

## Close

```text
close
```

Exit 0; wall 0.11 seconds:

```text
the live canvas is in <isolated-root>/out/easel/wet/live.png
closed; the session is in <isolated-root>/paintings/lua/wet.lua

```

## Open dry

```text
open dry
```

Exit 0; wall 0.117 seconds:

```text
easel "dry" open: 0 chunks · 2400px · no canvas yet

```

## Fixture dry

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.093 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.08 s to compute)

```

## Paint dry

```text
do work(m,{pile=p,hand="detail",coverage=2,clip=true,seed=7});print(drying(480,100),drying(50,50))
```

Exit 0; wall 0.471 seconds:

```text
open	dry
ok · chunk 2 (0.45 s to compute)

```

## Age dry fixture

```text
do print(wait(1440*30));for _,x in ipairs{420,450,480,510,540} do for _,y in ipairs{80,100,120} do print(x,y,drying(x,y)) end end
```

Exit 0; wall 0.028 seconds:

```text
day 31, 09:30
420	80	dry
420	100	dry
420	120	dry
450	80	dry
450	100	dry
450	120	dry
480	80	dry
480	100	dry
480	120	dry
510	80	dry
510	100	dry
510	120	dry
540	80	dry
540	100	dry
540	120	dry
ok · chunk 3 (0.01 s to compute)

```

## Save dry-before

```text
save <isolated-root>/dry-before.png
```

Exit 0; wall 0.03 seconds:

```text
<isolated-root>/dry-before.png

```

## Blend dry

```text
do blend(rect(430,80,100,40));print(wait(0))
```

Exit 0; wall 0.163 seconds:

```text
day 31, 09:30
ok · chunk 4 (0.14 s to compute)

```

## Save dry-blend

```text
save <isolated-root>/dry-blend.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/dry-blend.png

```

Changed pixels dry-before→dry-blend: inside [430, 80, 530, 120] = **0**, outside = **0**. Pixel bounds use logical coordinates ×2.4.

PNG equality dry-before/dry-blend: **true**.

## Replay dry

```text
check
```

Exit 0; wall 0.68 seconds:

```text
replay matches the live canvas exactly (4 chunks, 0.7s)

```

## Close

```text
close
```

Exit 0; wall 0.1 seconds:

```text
the live canvas is in <isolated-root>/out/easel/dry/live.png
closed; the session is in <isolated-root>/paintings/lua/dry.lua

```
# Passages and time runtime evidence

Agent CLI run on 2026-10-04, source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`, preserved default release build. Isolated disposable sessions, no provider needed. Fixture uses aspect5 and mask rect(400,60,160,80) to bound computation. This is an explicitly recorded narrower fixture than the checklist's square canvas. All images have 2400×480 pixels.

## Open body-sparse

```text
open body-sparse
```

Exit 0; wall 0.112 seconds:

```text
easel "body-sparse" open: 0 chunks · 2400px · no canvas yet

```

## Fixture body-sparse

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.109 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.09 s to compute)

```

## More body-sparse

```text
do work(m,{pile=p,hand="body",coverage=0.1,fill=false,seed=7});print(wait(0))
```

Exit 0; wall 0.065 seconds:

```text
day 1, 09:00
ok · chunk 2 (0.04 s to compute)

```

## Save body-sparse

```text
save <isolated-root>/body-sparse.png
```

Exit 0; wall 0.029 seconds:

```text
<isolated-root>/body-sparse.png

```

Changed pixels baseline→body-sparse: inside [400, 60, 560, 140] = **8747**, outside = **1067**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.092 seconds:

```text
the live canvas is in <isolated-root>/out/easel/body-sparse/live.png
closed; the session is in <isolated-root>/paintings/lua/body-sparse.lua

```

## Open body-fill

```text
open body-fill
```

Exit 0; wall 0.111 seconds:

```text
easel "body-fill" open: 0 chunks · 2400px · no canvas yet

```

## Fixture body-fill

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.107 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.09 s to compute)

```

## More body-fill

```text
do work(m,{pile=p,hand="body",coverage=0.1,fill=true,seed=7});print(wait(0))
```

Exit 0; wall 0.843 seconds:

```text
day 1, 09:06
ok · chunk 2 (0.82 s to compute)

```

## Save body-fill

```text
save <isolated-root>/body-fill.png
```

Exit 0; wall 0.035 seconds:

```text
<isolated-root>/body-fill.png

```

Changed pixels baseline→body-fill: inside [400, 60, 560, 140] = **73728**, outside = **14170**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.096 seconds:

```text
the live canvas is in <isolated-root>/out/easel/body-fill/live.png
closed; the session is in <isolated-root>/paintings/lua/body-fill.lua

```

## Open seed8

```text
open seed8
```

Exit 0; wall 0.11 seconds:

```text
easel "seed8" open: 0 chunks · 2400px · no canvas yet

```

## Fixture seed8

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.11 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.10 s to compute)

```

## More seed8

```text
do work(m,{pile=p,hand="body",coverage=0.5,fill=false,seed=8});print(wait(0))
```

Exit 0; wall 0.143 seconds:

```text
day 1, 09:01
ok · chunk 2 (0.12 s to compute)

```

## Save seed8

```text
save <isolated-root>/seed8.png
```

Exit 0; wall 0.032 seconds:

```text
<isolated-root>/seed8.png

```

Changed pixels baseline→seed8: inside [400, 60, 560, 140] = **37787**, outside = **4621**. Pixel bounds use logical coordinates ×2.4.

## Close

```text
close
```

Exit 0; wall 0.096 seconds:

```text
the live canvas is in <isolated-root>/out/easel/seed8/live.png
closed; the session is in <isolated-root>/paintings/lua/seed8.lua

```

PNG equality body/seed8: **false**.

## Reopen wet for time

```text
open wet
```

Exit 0; wall 1.292 seconds:

```text
resuming chunk 1/3
resumed chunk 1/3 0.13s
resuming chunk 2/3
resumed chunk 2/3 0.54s
resuming chunk 3/3
resumed chunk 3/3 0.56s
resumed 3 chunks from <isolated-root>/paintings/lua/wet.lua
easel "wet" open: 3 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1

```

## Wet time baseline

```text
do print(wait(0),drying(480,100))
```

Exit 0; wall 0.033 seconds:

```text
day 1, 09:30	open
ok · chunk 4 (0.00 s to compute)

```

## Save time-before

```text
save <isolated-root>/time-before.png
```

Exit 0; wall 0.05 seconds:

```text
<isolated-root>/time-before.png

```

## Time failure

```text
do wait(60);error("time rollback")
```

Exit 1; wall 0.016 seconds:

```text
runtime error: [string "chunk 5"]:1: time rollback
(the chunk failed and changed nothing)

```

## Time after failure

```text
do print(wait(0),drying(480,100))
```

Exit 0; wall 0.031 seconds:

```text
day 1, 09:30	open
ok · chunk 5 (0.01 s to compute)

```

## Save time-after

```text
save <isolated-root>/time-after.png
```

Exit 0; wall 0.047 seconds:

```text
<isolated-root>/time-after.png

```

PNG equality time-before/time-after: **true**.

## Time wait success

```text
do print(wait(60));print(drying(480,100));marker=1
```

Exit 0; wall 0.024 seconds:

```text
day 1, 10:30
open
ok · chunk 6 (0.00 s to compute)

```

## Marker and clock

```text
do print(marker,wait(0))
```

Exit 0; wall 0.026 seconds:

```text
1	day 1, 10:30
ok · chunk 7 (0.00 s to compute)

```

## Time note baseline

```text
do r=rag{width=20};r:dip(1);print(wait(0),r.damp)
```

Exit 0; wall 0.026 seconds:

```text
day 1, 10:30	1.0
ok · chunk 8 (0.00 s to compute)

```

## Look no hand time

```text
look
```

Exit 0; wall 0.034 seconds:

```text
<isolated-root>/out/easel/wet/look-0001.png (1000x200, 0.03s)

```

## Note no hand time

```text
note isolated verification note
```

Exit 0; wall 0.008 seconds:

```text
noted in <isolated-root>/notes/journal.md

```

## Time after note look

```text
do print(wait(0),r.damp)
```

Exit 0; wall 0.023 seconds:

```text
day 1, 10:30	1.0
ok · chunk 9 (0.00 s to compute)

```

## Infinite wait rejection

```text
do wait(math.huge)
```

Exit 1; wall 0.018 seconds:

```text
runtime error: wait(inf): want minutes from 0 to 5259600 (10 years); days are fine: wait(3 * 24 * 60)
(the chunk failed and changed nothing)

```

## Replay with time

```text
check
```

Exit 0; wall 1.367 seconds:

```text
replay matches the live canvas exactly (9 chunks, 1.3s)

```

## Close

```text
close
```

Exit 0; wall 0.144 seconds:

```text
the live canvas is in <isolated-root>/out/easel/wet/live.png
closed; the session is in <isolated-root>/paintings/lua/wet.lua

```

## Open clock-b

```text
open clock-b
```

Exit 0; wall 0.13 seconds:

```text
easel "clock-b" open: 0 chunks · 2400px · no canvas yet

```

## Fixture clock-b

```text
do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; m=rect(400,60,160,80); print(wait(0))
```

Exit 0; wall 0.202 seconds:

```text
day 1, 09:00
ok · chunk 1 (0.17 s to compute)

```

## Independent b

```text
do print(wait(0))
```

Exit 0; wall 0.031 seconds:

```text
day 1, 09:00
ok · chunk 2 (0.00 s to compute)

```

## Close

```text
close
```

Exit 0; wall 0.108 seconds:

```text
the live canvas is in <isolated-root>/out/easel/clock-b/live.png
closed; the session is in <isolated-root>/paintings/lua/clock-b.lua

```

## Open a again

```text
open wet
```

Exit 0; wall 1.898 seconds:

```text
resuming chunk 1/9
resumed chunk 1/9 0.31s
resuming chunk 2/9
resumed chunk 2/9 0.75s
resuming chunk 3/9
resumed chunk 3/9 0.71s
resuming chunk 4/9
resumed chunk 4/9 0.02s
resuming chunk 5/9
resumed chunk 5/9 0.00s
resuming chunk 6/9
resumed chunk 6/9 0.00s
resuming chunk 7/9
resumed chunk 7/9 0.01s
resuming chunk 8/9
resumed chunk 8/9 0.00s
resuming chunk 9/9
resumed chunk 9/9 0.00s
resumed 9 chunks from <isolated-root>/paintings/lua/wet.lua
easel "wet" open: 9 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1

```

## Advance a

```text
do print(wait(120))
```

Exit 0; wall 0.045 seconds:

```text
day 1, 12:30
ok · chunk 10 (0.01 s to compute)

```

## Close

```text
close
```

Exit 0; wall 0.124 seconds:

```text
the live canvas is in <isolated-root>/out/easel/wet/live.png
closed; the session is in <isolated-root>/paintings/lua/wet.lua

```

## Open b again

```text
open clock-b
```

Exit 0; wall 0.223 seconds:

```text
resuming chunk 1/2
resumed chunk 1/2 0.17s
resuming chunk 2/2
resumed chunk 2/2 0.00s
resumed 2 chunks from <isolated-root>/paintings/lua/clock-b.lua
easel "clock-b" open: 2 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1

```

## B unchanged

```text
do print(wait(0))
```

Exit 0; wall 0.035 seconds:

```text
day 1, 09:00
ok · chunk 3 (0.00 s to compute)

```

## Close

```text
close
```

Exit 0; wall 0.089 seconds:

```text
the live canvas is in <isolated-root>/out/easel/clock-b/live.png
closed; the session is in <isolated-root>/paintings/lua/clock-b.lua

```

## Visual findings and verification limits

Inspected [body](passages-body.png), [broad](passages-broad.png), [detail](passages-detail.png), [hatch](passages-hatch.png), [glaze](passages-glaze.png) and [scumble](passages-scumble.png): visible marks differ in size, distribution and boundary handling. [Stipple](passages-stipple.png) is visibly made of small touches. These are observations of this fixture, not a claim about every material combination.

[Found](passages-found.png) has a tighter edge than [lost](passages-lost.png), which extends farther and has more dispersed ends. [Clip](passages-clip.png) has straight rectangle boundaries. Decoded pixel comparisons establish zero changed pixels outside explicit clip masks and default detail/blend masks.

[Body sparse](passages-body-sparse.png) leaves large gaps; [body fill](passages-body-fill.png) covers those gaps. The initial low-coverage glaze fixture produced no strokes with either fill setting, so that fixture alone could not establish fill behavior; body was run as the positive case.

[Wet before](passages-wet-before.png) and [wet blend](passages-wet-blend.png) show local smoothing of painted gaps. Numeric comparison changed 14080 pixels inside its blend mask and zero outside. A separate 30-day-aged fixture reported dry at 15 sampled painted points and its blend PNG remained byte-identical. This establishes wet versus dry clean blending, not the unrun contrasting-color layer comparison.

No live option mutation, passage-specific interruption, storage failure, exact subminute timing, intermediate 15-minute aging slices, all drying stages or exhaustive numeric bounds were established in this lane. All disposable sessions were closed; selected evidence images are retained here.
