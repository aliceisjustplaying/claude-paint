# Isolated CLI observations

2026-10-04. Default release build compiled from source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. These are agent-driven probes, not a complete human verification pass.

## Initial status

Input: `easel status`

Exit: 0

```text
1 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1

```

## Save before failure

Input: `easel save <isolated-root>/before.png`

Exit: 0

```text
<isolated-root>/before.png

```

## Chunk failure

Input: `easel do marker=999; b:touch(400,80); error("probe rollback")`

Exit: 1

```text
runtime error: [string "chunk 2"]:1: probe rollback
(the chunk failed and changed nothing)

```

## State after failure

Input: `easel do print(marker == nil, b:fullness())`

Exit: 0

```text
true	0.074084550142288208
ok · chunk 2 (0.00 s to compute)

```

## Save after failure

Input: `easel save <isolated-root>/after.png`

Exit: 0

```text
<isolated-root>/after.png

```

Before/after PNG equality: true.

## Crop and modes

Input: `easel look --crop 300,150,100,50 --mode value,mirror --grid 50`

Exit: 0

```text
<isolated-root>/out/easel/probe/look-0002.png (480x240, 0.01s)

```

## Palette conflict

Input: `easel look --palette --grid`

Exit: 1

```text
look: --palette takes no other option

```

## Oversize crop

Input: `easel look --crop 0,0,900,150`

Exit: 1

```text
--crop exceeds 1200 pixels per side; choose a smaller crop (crops stay 1:1)

```

## Palette image

Input: `easel look --palette`

Exit: 0

```text
<isolated-root>/out/easel/probe/look-0003.png (1124x96, 0.01s)

```

## Canvas reset rejection

Input: `easel do canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}`

Exit: 1

```text
runtime error: the canvas is already set up (canvas{} is the first chunk)
(the chunk failed and changed nothing)

```

## Area and rag drawing

Input: `easel do m=rect(400,40,80,80); work(m,{hand="detail",pile=p,coverage=0.3,clip=true}); r=rag{width=20}; r:dip(0.5); r:blot(440,80); h=pencil("HB"); h:line({{600,100},{700,80}},{pressure=0.4}); print(r.load,r.damp,h:width(),drying(440,80))`

Exit: 0

```text
0.006089281290769577	0.49712607264518738	3.216341495513916	dry
ok · chunk 3 (0.05 s to compute)

```

## Second look

Input: `easel look`

Exit: 0

```text
<isolated-root>/out/easel/probe/look-0004.png (1000x200, 0.01s)

```

## Journal

Input: `easel note Documentation probe`

Exit: 0

```text
noted in <isolated-root>/notes/journal.md

```

## Globals

Input: `easel globals`

Exit: 0

```text
1	H	number 199.99998474121094
1	W	number 1000
1	b	brush(round 4, 7% full)
1	p	pile(red earth 1; medium 0)
3	h	table with 3 entries
3	m	mask(6400 sq units)
3	r	rag(width 20, load 0.01)

```

## Replay check

Input: `easel check`

Exit: 0

```text
replay matches the live canvas exactly (3 chunks, 0.2s)

```

## Close

Input: `easel close`

Exit: 0

```text
the live canvas is in <isolated-root>/out/easel/probe/live.png
closed; the session is in <isolated-root>/paintings/lua/probe.lua

```

## Reopen

Input: `easel open probe`

Exit: 0

```text
resuming chunk 1/3
resumed chunk 1/3 0.10s
resuming chunk 2/3
resumed chunk 2/3 0.00s
resuming chunk 3/3
resumed chunk 3/3 0.05s
resumed 3 chunks from <isolated-root>/paintings/lua/probe.lua
easel "probe" open: 3 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1

```

## Final status

Input: `easel status`

Exit: 0

```text
3 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1

```

## Final close

Input: `easel close`

Exit: 0

```text
the live canvas is in <isolated-root>/out/easel/probe/live.png
closed; the session is in <isolated-root>/paintings/lua/probe.lua

```
