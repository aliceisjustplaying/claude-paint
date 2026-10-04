# Remaining delivery matrix: runtime evidence

Agent-driven probes on 2026-10-04 against unchanged commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. Every command below ran through the actual CLI or unmodified runner script. Temporary paths are replaced by `<fixture>`; script-building source is `<source>`. Failures used disposable sessions or outputs. [Machine-readable record](matrix-delivery.json).

All seven explicit-current studio profiles exported successfully. The default historical export failed because its selected `round-16` binary has no `tubes` command, while the current exporter requires it. The historical ref resolved to `f48e722f8c9a916d3d91b54bc8be0320d0bfd16f`. This is a product defect, not a missing local dependency.

Actual finishing options, captured check replacement, save queuing, alternate replay widths, numeric/dynamic video pacing, permissions, isolated ENOSPC and process interruption were exercised. Width 9600 succeeded with aspect5. The bounded finishing-loop case overrides public Lua globals to establish exactly120 retries; it does not claim physical ten-year drying. PNG bit depth and mode were decoded. Actual Chromium then reported sRGB canvas interpretation, with five sampled pixels matching independent Pillow decoding. This tests display interpretation, not physical pigment color calibration.

## open

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.295 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## canvas

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}
```

Exit: 0. Elapsed: 0.085 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## paint

```text
<binaries>/easel-default do p\=pile\{\{\"red\ earth\",1\}\}\;b\=brush\(\"round\",4\)\;b:load\(p,0.8\)\;b:stroke\(\{\{100,80\},\{600,80\}\}\)
```

Exit: 0. Elapsed: 0.07 seconds.

```text
ok · chunk 2 (0.05 s to compute)
```

## save default

```text
<binaries>/easel-default save
```

Exit: 0. Elapsed: 0.028 seconds.

```text
<fixture>/live/out/easel/painting/painting.png
```

## save explicit

```text
<binaries>/easel-default save <fixture>/wet.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/wet.png
```

## check

```text
<binaries>/easel-default check
```

Exit: 0. Elapsed: 0.132 seconds.

```text
replay matches the live canvas exactly (2 chunks, 0.1s)
```

## close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.094 seconds.

```text
the live canvas is in <fixture>/live/out/easel/painting/live.png
closed; the session is in <fixture>/live/paintings/lua/painting.lua
```

## replay ordered

```text
<binaries>/easel-default run <fixture>/live/paintings/lua/painting.lua --out <fixture>/replay.png --state-digest <fixture>/state.tsv --dump-surface <fixture>/surface.bin
```

Exit: 0. Elapsed: 0.467 seconds.

```text
chunk   1     0.07s
  chunk   2     0.05s
wrote <fixture>/replay.png (2 chunks, painted in 0.4s, total 0.4s)
surface 2400x480 µm → <fixture>/surface.bin
```

## small replay

```text
<binaries>/easel-default run <fixture>/live/paintings/lua/painting.lua --out <fixture>/small.png --width 160
```

Exit: 0. Elapsed: 0.065 seconds.

```text
chunk   1     0.03s
  chunk   2     0.00s
wrote <fixture>/small.png (2 chunks, painted in 0.0s, total 0.0s)
```

## print finishing chunk

```text
<binaries>/easel-default finish --print-chunk
```

Exit: 0. Elapsed: 0.003 seconds.

```text
-- finishing, applied after the session by scripts/finish_painting
local function when_dry(f)
  for _ = 1, 120 do
    local ok, e = pcall(f)
    if ok then return end
    if not tostring(e):find('not all dry', 1, true) then error(e, 0) end
    wait(30 * 24 * 60)
  end
  error('still not dry after ten years', 0)
end
when_dry(function() varnish{coats=0.4} end)
when_dry(function() cracks{} end)
```

## bounded finishing loop

```text
<binaries>/easel-default run <fixture>/bounded.lua
```

Exit: 1. Elapsed: 0.004 seconds.

```text
false	still not dry after ten years	120
  chunk   1     0.00s
the program never made a canvas
```

## direct finish default

```text
<binaries>/easel-default finish <fixture>/live/out/easel/painting/live.ckpt <fixture>/finish-default.png --log <fixture>/live/paintings/lua/painting.lua
```

Exit: 0. Elapsed: 0.231 seconds.

```text
wrote <fixture>/finish-default.png (restored 2 chunks in 0.0s, finished in 0.2s, total 0.2s)
```

## direct finish no-varnish

```text
<binaries>/easel-default finish <fixture>/live/out/easel/painting/live.ckpt <fixture>/finish-no-varnish.png --log <fixture>/live/paintings/lua/painting.lua --no-varnish
```

Exit: 0. Elapsed: 0.125 seconds.

```text
wrote <fixture>/finish-no-varnish.png (restored 2 chunks in 0.0s, finished in 0.1s, total 0.1s)
```

## direct finish no-cracks

```text
<binaries>/easel-default finish <fixture>/live/out/easel/painting/live.ckpt <fixture>/finish-no-cracks.png --log <fixture>/live/paintings/lua/painting.lua --no-cracks
```

Exit: 0. Elapsed: 0.107 seconds.

```text
wrote <fixture>/finish-no-cracks.png (restored 2 chunks in 0.0s, finished in 0.1s, total 0.1s)
```

## direct finish relief

```text
<binaries>/easel-default finish <fixture>/live/out/easel/painting/live.ckpt <fixture>/finish-relief.png --log <fixture>/live/paintings/lua/painting.lua --relief
```

Exit: 0. Elapsed: 0.163 seconds.

```text
wrote <fixture>/finish-relief.png (restored 2 chunks in 0.0s, finished in 0.1s, total 0.2s)
```

## direct finish coats

```text
<binaries>/easel-default finish <fixture>/live/out/easel/painting/live.ckpt <fixture>/finish-coats.png --log <fixture>/live/paintings/lua/painting.lua --coats 1.2
```

Exit: 0. Elapsed: 0.163 seconds.

```text
wrote <fixture>/finish-coats.png (restored 2 chunks in 0.0s, finished in 0.1s, total 0.2s)
```

## reopen without checkpoint

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.229 seconds.

```text
resuming chunk 1/2
resumed chunk 1/2 0.07s
resuming chunk 2/2
resumed chunk 2/2 0.05s
resumed 2 chunks from <fixture>/live/paintings/lua/painting.lua
easel "painting" open: 2 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## save path A

```text
<binaries>/easel-default save <fixture>/a.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/a.png
```

## save overwrite

```text
<binaries>/easel-default save <fixture>/a.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/a.png
```

## save path B

```text
<binaries>/easel-default save <fixture>/b.png
```

Exit: 0. Elapsed: 0.026 seconds.

```text
<fixture>/b.png
```

## frames implicit

```text
<binaries>/easel-default frames
```

Exit: 0. Elapsed: 0.003 seconds.

```text
frames on (<fixture>/live/out/easel/painting/frames)
```

## captured

```text
<binaries>/easel-default do print\(\"frame\ one\"\)
```

Exit: 0. Elapsed: 0.028 seconds.

```text
frame one
ok · chunk 3 (0.00 s to compute)
```

## frames typo

```text
<binaries>/easel-default frames onn
```

Exit: 0. Elapsed: 0.004 seconds.

```text
frames off (<fixture>/live/out/easel/painting/frames)
```

## not captured

```text
<binaries>/easel-default do print\(\"frame\ two\"\)
```

Exit: 0. Elapsed: 0.015 seconds.

```text
frame two
ok · chunk 4 (0.00 s to compute)
```

## frames on

```text
<binaries>/easel-default frames on
```

Exit: 0. Elapsed: 0.003 seconds.

```text
frames on (<fixture>/live/out/easel/painting/frames)
```

## captured again

```text
<binaries>/easel-default do print\(\"frame\ three\"\)
```

Exit: 0. Elapsed: 0.021 seconds.

```text
frame three
ok · chunk 5 (0.00 s to compute)
```

## close capture

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.086 seconds.

```text
the live canvas is in <fixture>/live/out/easel/painting/live.png
closed; the session is in <fixture>/live/paintings/lua/painting.lua
```

## reopen capture reset

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.224 seconds.

```text
resuming chunk 1/5
resumed chunk 1/5 0.07s
resuming chunk 2/5
resumed chunk 2/5 0.05s
resuming chunk 3/5
resumed chunk 3/5 0.00s
resuming chunk 4/5
resumed chunk 4/5 0.00s
resuming chunk 5/5
resumed chunk 5/5 0.00s
resumed 5 chunks from <fixture>/live/paintings/lua/painting.lua
easel "painting" open: 5 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## no new capture

```text
<binaries>/easel-default do print\(\"frame\ four\"\)
```

Exit: 0. Elapsed: 0.022 seconds.

```text
frame four
ok · chunk 6 (0.00 s to compute)
```

## close reset

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.082 seconds.

```text
the live canvas is in <fixture>/live/out/easel/painting/live.png
closed; the session is in <fixture>/live/paintings/lua/painting.lua
```

## frames exclude drying wait

```text
<binaries>/easel-default run <fixture>/wait.lua --out <fixture>/wait.png --frames-every 0.2 --frames-dir <fixture>/frames --frame-width 160
```

Exit: 0. Elapsed: 0.348 seconds.

```text
chunk   1     0.07s
  chunk   2     0.07s
  chunk   3     0.01s
  chunk   4     0.07s
frames: 7 written (151 hand-time intervals of 0.2s crossed, 4 chunks) over 0.5 min of hand time → <fixture>/frames
wrote <fixture>/wait.png (4 chunks, painted in 0.3s, total 0.3s)
```

## default width second invocation

```text
<binaries>/easel-default run <fixture>/source.lua --out <fixture>/default-second.png
```

Exit: 0. Elapsed: 0.155 seconds.

```text
chunk   1     0.07s
  chunk   2     0.05s
wrote <fixture>/default-second.png (2 chunks, painted in 0.1s, total 0.1s)
```

## export friedrich

```text
<source>/scripts/export_r16_studio friedrich <fixture>/export-friedrich
```

Exit: 0. Elapsed: 4.473 seconds.

```text
exported the friedrich studio from 4e525e50897807e9b5f734071dfeb330f1a393d3 (4e525e5) to <fixture>/export-friedrich
  easel: <fixture>/export-friedrich/bin/easel (checked in $TMPDIR//studio-probe.KLMctW)
  notices: <fixture>/export-friedrich/bin/THIRD_PARTY_NOTICES.md
  box: the default tube box
  notes: easel_guide.md journal.md research/friedrich_materials.md research/oil_paint_physics.md
```

## export sargent

```text
<source>/scripts/export_r16_studio sargent <fixture>/export-sargent
```

Exit: 0. Elapsed: 92.434 seconds.

```text
exported the sargent studio from 4e525e50897807e9b5f734071dfeb330f1a393d3 (4e525e5) to <fixture>/export-sargent
  easel: <fixture>/export-sargent/bin/easel (checked in $TMPDIR//studio-probe.16u9Im)
  notices: <fixture>/export-sargent/bin/THIRD_PARTY_NOTICES.md
  box: sargent
  notes: easel_guide.md journal.md research/oil_paint_physics.md research/sargent_materials.md
```

## export hopper

```text
<source>/scripts/export_r16_studio hopper <fixture>/export-hopper
```

Exit: 0. Elapsed: 92.618 seconds.

```text
exported the hopper studio from 4e525e50897807e9b5f734071dfeb330f1a393d3 (4e525e5) to <fixture>/export-hopper
  easel: <fixture>/export-hopper/bin/easel (checked in $TMPDIR//studio-probe.utblaI)
  notices: <fixture>/export-hopper/bin/THIRD_PARTY_NOTICES.md
  box: hopper
  notes: easel_guide.md journal.md research/hopper_materials.md research/oil_paint_physics.md
```

## export tonn

```text
<source>/scripts/export_r16_studio tonn <fixture>/export-tonn
```

Exit: 0. Elapsed: 92.622 seconds.

```text
exported the tonn studio from 4e525e50897807e9b5f734071dfeb330f1a393d3 (4e525e5) to <fixture>/export-tonn
  easel: <fixture>/export-tonn/bin/easel (checked in $TMPDIR//studio-probe.LTl9z3)
  notices: <fixture>/export-tonn/bin/THIRD_PARTY_NOTICES.md
  box: tonn
  notes: easel_guide.md journal.md research/oil_paint_physics.md research/tonn_materials.md
```

## export alma-tadema

```text
<source>/scripts/export_r16_studio alma-tadema <fixture>/export-alma-tadema
```

Exit: 0. Elapsed: 92.725 seconds.

```text
exported the alma-tadema studio from 4e525e50897807e9b5f734071dfeb330f1a393d3 (4e525e5) to <fixture>/export-alma-tadema
  easel: <fixture>/export-alma-tadema/bin/easel (checked in $TMPDIR//studio-probe.1txIwy)
  notices: <fixture>/export-alma-tadema/bin/THIRD_PARTY_NOTICES.md
  box: alma-tadema
  notes: easel_guide.md journal.md research/alma_tadema_materials.md research/oil_paint_physics.md
```

## export inness

```text
<source>/scripts/export_r16_studio inness <fixture>/export-inness
```

Exit: 0. Elapsed: 93.518 seconds.

```text
exported the inness studio from 4e525e50897807e9b5f734071dfeb330f1a393d3 (4e525e5) to <fixture>/export-inness
  easel: <fixture>/export-inness/bin/easel (checked in $TMPDIR//studio-probe.6RCvEa)
  notices: <fixture>/export-inness/bin/THIRD_PARTY_NOTICES.md
  box: inness
  notes: easel_guide.md journal.md research/inness_materials.md research/oil_paint_physics.md
```

## check valid

```text
<source>/scripts/check_painting <fixture>/valid/paintings/lua/painting.lua <fixture>/check-valid
```

Exit: 1. Elapsed: 0.151 seconds.

```text
<source>/scripts/check_painting: line 43: cd: <fixture>/valid/paintings/lua/painting.lua: Not a directory
```

## check stripped

```text
<source>/scripts/check_painting <fixture>/stripped/paintings/lua/painting.lua <fixture>/check-stripped
```

Exit: 1. Elapsed: 0.02 seconds.

```text
<source>/scripts/check_painting: line 43: cd: <fixture>/stripped/paintings/lua/painting.lua: Not a directory
```

## check swapped

```text
<source>/scripts/check_painting <fixture>/swapped/paintings/lua/painting.lua <fixture>/check-swapped
```

Exit: 1. Elapsed: 0.011 seconds.

```text
<source>/scripts/check_painting: line 43: cd: <fixture>/swapped/paintings/lua/painting.lua: Not a directory
```

## script finish valid

```text
<source>/scripts/finish_painting <fixture>/valid/paintings/lua/painting.lua <fixture>/valid-finished.png
```

Exit: 0. Elapsed: 2.189 seconds.

```text
wrote <fixture>/valid-finished.png (restored 6 chunks in 0.0s, finished in 1.4s, total 1.5s)
finished from the save: <fixture>/valid-finished.png (log: <fixture>/valid-finished.lua; save compressed)
```

## script finish corrupt

```text
<source>/scripts/finish_painting <fixture>/corrupt/paintings/lua/painting.lua <fixture>/corrupt-finished.png
```

Exit: 0. Elapsed: 3.288 seconds.

```text
<fixture>/corrupt/out/easel/painting/live.ckpt: not a canvas checkpoint (or an older format)
finish_painting: the save didn't finish (above); replaying the log instead
  chunk   1     0.83s
  chunk   2     0.51s
frame one
  chunk   3     0.00s
frame two
  chunk   4     0.00s
frame three
  chunk   5     0.00s
frame four
  chunk   6     0.00s
  chunk   7     1.42s
wrote <fixture>/corrupt-finished.png (7 chunks, painted in 2.8s, total 2.9s)
finished: <fixture>/corrupt-finished.png (log: <fixture>/corrupt-finished.lua)
```

## script relief only

```text
<source>/scripts/finish_painting <fixture>/source.lua <fixture>/relief-only.png --no-varnish --no-cracks --relief
```

Exit: 0. Elapsed: 1.99 seconds.

```text
chunk   1     0.56s
  chunk   2     0.47s
  chunk   3     0.46s
wrote <fixture>/relief-only.png (3 chunks, painted in 1.5s, total 1.6s)
finished: <fixture>/relief-only.png (log: <fixture>/relief-only.lua)
```

## script coats 1.2

```text
<source>/scripts/finish_painting <fixture>/source.lua <fixture>/coats.png --coats 1.2
```

Exit: 0. Elapsed: 2.504 seconds.

```text
chunk   1     0.62s
  chunk   2     0.26s
  chunk   3     1.09s
wrote <fixture>/coats.png (3 chunks, painted in 2.0s, total 2.1s)
finished: <fixture>/coats.png (log: <fixture>/coats.lua)
```

## script nothing

```text
<source>/scripts/finish_painting <fixture>/source.lua <fixture>/nothing.png --no-varnish --no-cracks
```

Exit: 1. Elapsed: 0.026 seconds.

```text
nothing to do
```

## numeric video

```text
<source>/scripts/replay_clip <fixture>/source.lua <fixture>/numeric.mp4 --reuse --frames-dir <fixture>/frames --length 5 --max-hold 2 --width 160 --pace 1
```

Exit: 0. Elapsed: 18.384 seconds.

```text
3 moments, 0.5 min of hand time, pace 1: about 5.0 s of clip
stream|width=160|height=32|nb_frames=120
format|duration=5.000000|size=5158
```

## dynamic video

```text
<source>/scripts/replay_clip <fixture>/source.lua <fixture>/dynamic.mp4 --reuse --frames-dir <fixture>/frames --length 5 --max-hold 2 --width 160 --pace dynamic --floor .3 --gamma .8 --ramp 1.5 --ramp-span .4 --open-hold .2 --final-first .5
```

Exit: 0. Elapsed: 0.917 seconds.

```text
3 moments, 3 shown, 0.5 min of hand time, pace dynamic (floor 0.3, gamma 0.8): about 4.5 s of clip
dynamic    per tenth:  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 100.0   min 0.0 max 100.0 sd 30.0
hand^0.6   per tenth:  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 100.0   min 0.0 max 100.0 sd 30.0
stream|width=160|height=32|nb_frames=120
format|duration=5.000000|size=5084
```

## numeric ffprobe

```text
ffprobe -v error -show_entries format\=duration:stream\=codec_name,width,height -of json <fixture>/numeric.mp4
```

Exit: 0. Elapsed: 0.095 seconds.

```text
{
    "programs": [

    ],
    "stream_groups": [

    ],
    "streams": [
        {
            "codec_name": "h264",
            "width": 160,
            "height": 32
        }
    ],
    "format": {
        "duration": "5.000000"
    }
}
```

## dynamic ffprobe

```text
ffprobe -v error -show_entries format\=duration:stream\=codec_name,width,height -of json <fixture>/dynamic.mp4
```

Exit: 0. Elapsed: 0.105 seconds.

```text
{
    "programs": [

    ],
    "stream_groups": [

    ],
    "streams": [
        {
            "codec_name": "h264",
            "width": 160,
            "height": 32
        }
    ],
    "format": {
        "duration": "5.000000"
    }
}
```

## maximum width

```text
<binaries>/easel-default run <fixture>/wide.lua --width 9600 --out <fixture>/wide.png
```

Exit: 0. Elapsed: 3.112 seconds.

```text
chunk   1     2.45s
wrote <fixture>/wide.png (1 chunks, painted in 2.4s, total 3.1s)
```

## missing ffmpeg

```text
<source>/scripts/replay_clip <fixture>/source.lua <fixture>/missing.mp4
```

Exit: 1. Elapsed: 0.01 seconds.

```text
replay_clip: needs ffmpeg
```

## replay permission

```text
<binaries>/easel-default run <fixture>/source.lua --out <fixture>/denied/out.png
```

Exit: 1. Elapsed: 1.247 seconds.

```text
chunk   1     0.85s
  chunk   2     0.22s
<fixture>/denied/out.png: Permission denied (os error 13)
```

## finish permission

```text
<source>/scripts/finish_painting <fixture>/source.lua <fixture>/denied/finished.png
```

Exit: 1. Elapsed: 0.322 seconds.

```text
<source>/scripts/finish_painting: line 66: <fixture>/denied/finished.lua.part: Permission denied
```

## concurrent open

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.117 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## concurrent canvas

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}
```

Exit: 0. Elapsed: 0.64 seconds.

```text
ok · chunk 1 (0.62 s to compute)
```

## concurrent slow source

```text
<binaries>/easel-default do local\ n\=0\;for\ i\=1,500000000\ do\ n\=n+i\ end\;print\(\"completed\",n\)
```

Exit: 0. Elapsed: 6.877 seconds.

```text
completed	125000000250000000
ok · chunk 2 (6.84 s to compute)
```

## available during check

```text
<binaries>/easel-default status
```

Exit: 0. Elapsed: 0.041 seconds.

```text
2 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## future paint during check

```text
<binaries>/easel-default do p\=pile\{\{\"red\ earth\",1\}\}\;b\=brush\(\"round\",4\)\;b:load\(p,0.8\)\;b:stroke\(\{\{100,80\},\{600,80\}\}\)
```

Exit: 0. Elapsed: 0.337 seconds.

```text
ok · chunk 3 (0.31 s to compute)
```

## captured check A

```text
<binaries>/easel-default check
```

Exit: 0. Elapsed: 2.979 seconds.

```text
replay matches the live canvas exactly (2 chunks, 2.9s)
```

## superseded check

```text
<binaries>/easel-default check
```

Exit: 1. Elapsed: 0.206 seconds.

```text
check stopped before chunk 2 of 3
```

## replacement check

```text
<binaries>/easel-default check
```

Exit: 0. Elapsed: 3.925 seconds.

```text
replay matches the live canvas exactly (3 chunks, 3.9s)
```

## long ordinary source

```text
<binaries>/easel-default do local\ n\=0\;for\ i\=1,500000000\ do\ n\=n+i\ end\;print\(\"completed\",n\)
```

Exit: 0. Elapsed: 3.587 seconds.

```text
completed	125000000250000000
ok · chunk 4 (3.56 s to compute)
```

## queued frames

```text
<binaries>/easel-default frames on
```

Exit: 0. Elapsed: 3.479 seconds.

```text
frames on (<fixture>/concurrent/out/easel/painting/frames)
```

## queued save

```text
<binaries>/easel-default save <fixture>/queued-save.png
```

Exit: 0. Elapsed: 3.566 seconds.

```text
<fixture>/queued-save.png
```

## parallel save parallel-b.png

```text
<binaries>/easel-default save <fixture>/parallel-b.png
```

Exit: 0. Elapsed: 0.06 seconds.

```text
<fixture>/parallel-b.png
```

## parallel save parallel-a.png

```text
<binaries>/easel-default save <fixture>/parallel-a.png
```

Exit: 0. Elapsed: 0.166 seconds.

```text
<fixture>/parallel-a.png
```

## collision source.lua

```text
<binaries>/easel-default run <fixture>/source.lua --out <fixture>/collision.png
```

Exit: 0. Elapsed: 0.384 seconds.

```text
chunk   1     0.22s
  chunk   2     0.10s
wrote <fixture>/collision.png (2 chunks, painted in 0.3s, total 0.4s)
```

## collision wait.lua

```text
<binaries>/easel-default run <fixture>/wait.lua --out <fixture>/collision.png
```

Exit: 0. Elapsed: 0.604 seconds.

```text
chunk   1     0.21s
  chunk   2     0.12s
  chunk   3     0.06s
  chunk   4     0.17s
wrote <fixture>/collision.png (4 chunks, painted in 0.6s, total 0.6s)
```

## killed save artifact

```text
ls -l <fixture>/killed-save.png
```

Exit: 0. Elapsed: 0.008 seconds.

```text
-rw-r--r-- 1 <user> wheel 333561 Oct  4 13:11 <fixture>/killed-save.png
```

## concurrent close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.104 seconds.

```text
the live canvas is in <fixture>/concurrent/out/easel/painting/live.png
closed; the session is in <fixture>/concurrent/paintings/lua/painting.lua
```

## partial replay artifact

```text
ls -l <fixture>/partial.tsv
```

Exit: 0. Elapsed: 0.011 seconds.

```text
-rw-r--r-- 1 <user> wheel 103 Oct  4 13:11 <fixture>/partial.tsv
```

## fixed replay b

```text
<binaries>/easel-default run <fixture>/slow.lua --width 320 --out <fixture>/fixed-b.png --state-digest <fixture>/fixed-b.tsv --dump-surface <fixture>/fixed-b.bin --frames-every 2 --frames-dir <fixture>/fixed-b --frame-width 320
```

Exit: 0. Elapsed: 3.723 seconds.

```text
chunk   1     0.02s
  chunk   2     0.04s
completed	125000000250000000
  chunk   3     3.63s
frames: 5 written (12 hand-time intervals of 2s crossed, 3 chunks) over 0.4 min of hand time → <fixture>/fixed-b
wrote <fixture>/fixed-b.png (3 chunks, painted in 3.7s, total 3.7s)
surface 320x64 µm → <fixture>/fixed-b.bin
```

## fixed replay a

```text
<binaries>/easel-default run <fixture>/slow.lua --width 160 --out <fixture>/fixed-a.png --state-digest <fixture>/fixed-a.tsv --dump-surface <fixture>/fixed-a.bin --frames-every 1 --frames-dir <fixture>/fixed-a --frame-width 160
```

Exit: 0. Elapsed: 3.752 seconds.

```text
chunk   1     0.01s
  chunk   2     0.01s
completed	125000000250000000
  chunk   3     3.72s
frames: 5 written (25 hand-time intervals of 1s crossed, 3 chunks) over 0.4 min of hand time → <fixture>/fixed-a
wrote <fixture>/fixed-a.png (3 chunks, painted in 3.7s, total 3.7s)
surface 160x32 µm → <fixture>/fixed-a.bin
```

## check corrected valid

```text
<source>/scripts/check_painting <fixture>/valid <fixture>/check-valid
```

Exit: 0. Elapsed: 7.426 seconds.

```text
log: <fixture>/valid/paintings/lua/painting.lua (6 chunks); the studio's committed record equals its log
resuming chunk 1/6
resumed chunk 1/6 0.59s
resuming chunk 2/6
resumed chunk 2/6 0.09s
resuming chunk 3/6
resumed chunk 3/6 0.00s
resuming chunk 4/6
resumed chunk 4/6 0.00s
resuming chunk 5/6
resumed chunk 5/6 0.00s
resuming chunk 6/6
resumed chunk 6/6 0.00s
resumed 6 chunks from <fixture>/check-valid/paintings/lua/painting.lua
easel "painting" open: 6 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
the replay equals the live canvas saved at close (the session had replayed 5 of its 6 chunks from the log)
the replay's canvas state equals the save written at close (the session had replayed 5 of its 6 chunks from the log)
the replay's PNG equals the painter's last save
times: reopen 1 s, save and close 0 s
check: ok: 6 chunks replay to the live canvas; the replay equals the live canvas saved at close (the session had replayed 5 of its 6 chunks from the log); the replay's canvas state equals the save written at close (the session had replayed 5 of its 6 chunks from the log); the replay's PNG equals the painter's last save; the studio's committed record equals its log
```

## check corrected stripped

```text
<source>/scripts/check_painting <fixture>/stripped <fixture>/check-stripped
```

Exit: 0. Elapsed: 0.617 seconds.

```text
log: <fixture>/stripped/paintings/lua/painting.lua (6 chunks); the studio's committed record equals its log
resuming chunk 1/6
resumed chunk 1/6 0.08s
resuming chunk 2/6
resumed chunk 2/6 0.06s
resuming chunk 3/6
resumed chunk 3/6 0.00s
resuming chunk 4/6
resumed chunk 4/6 0.00s
resuming chunk 5/6
resumed chunk 5/6 0.00s
resuming chunk 6/6
resumed chunk 6/6 0.00s
resumed 6 chunks from <fixture>/check-stripped/paintings/lua/painting.lua
easel "painting" open: 6 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
the session saved no live canvas at close
the session wrote no save file at close
the replay's PNG equals the painter's last save
times: reopen 1 s, save and close 0 s
check: replay only: 6 chunks replay, but the session saved no live canvas at close; the session wrote no save file at close; the replay's PNG equals the painter's last save; the studio's committed record equals its log
```

## check corrected swapped

```text
<source>/scripts/check_painting <fixture>/swapped <fixture>/check-swapped
```

Exit: 1. Elapsed: 0.92 seconds.

```text
log: <fixture>/swapped/paintings/lua/painting.lua (6 chunks); the studio's committed record equals its log
resuming chunk 1/6
resumed chunk 1/6 0.10s
resuming chunk 2/6
resumed chunk 2/6 0.06s
resuming chunk 3/6
resumed chunk 3/6 0.00s
resuming chunk 4/6
resumed chunk 4/6 0.00s
resuming chunk 5/6
resumed chunk 5/6 0.00s
resuming chunk 6/6
resumed chunk 6/6 0.00s
resumed 6 chunks from <fixture>/check-swapped/paintings/lua/painting.lua
easel "painting" open: 6 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
the replay DIFFERS from the live canvas saved at close (the session had replayed 5 of its 6 chunks from the log)
the replay's canvas state equals the save written at close (the session had replayed 5 of its 6 chunks from the log)
the replay's PNG equals the painter's last save
times: reopen 0 s, save and close 0 s
check: DIFFERS from the live canvas: the replay DIFFERS from the live canvas saved at close (the session had replayed 5 of its 6 chunks from the log); the replay's canvas state equals the save written at close (the session had replayed 5 of its 6 chunks from the log); the replay's PNG equals the painter's last save; the studio's committed record equals its log
```

## check sentinel guard

```text
<source>/scripts/check_painting <fixture>/valid <fixture>/sentinel
```

Exit: 2. Elapsed: 0.014 seconds.

```text
<fixture>/sentinel exists and isn't empty
```

## export sentinel guard

```text
<source>/scripts/export_r16_studio blank <fixture>/sentinel
```

Exit: 1. Elapsed: 0.046 seconds.

```text
<fixture>/sentinel exists and isn't empty
```

## default historical export

```text
<source>/scripts/export_r16_studio blank <fixture>/historical
```

Exit: 1. Elapsed: 50.568 seconds.

```text
easel: no command "tubes"

easel: a live painting session (see notes/easel_guide.md)

  easel open          start or reattach; replays paintings/lua/painting.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,mirror] [--grid [step]] [--size 1000]
  easel log           the painting so far (= paintings/lua/painting.lua)
  easel status        chunks, width, canvas
  easel save [path]   the canvas as a PNG (default out/easel/painting/painting.png)
  easel frames on|off save a look after every chunk
  easel check         replay the log from scratch and compare with the live canvas
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
the exported easel can't name its box
```

## stale open

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.114 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## stale canvas

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}
```

Exit: 0. Elapsed: 0.079 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## stale save

```text
<binaries>/easel-default save
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/stalesave/out/easel/painting/painting.png
```

## stale later paint

```text
<binaries>/easel-default do p\=pile\{\{\"red\ earth\",1\}\}\;b\=brush\(\"round\",4\)\;b:load\(p,0.8\)\;b:stroke\(\{\{100,80\},\{600,80\}\}\)
```

Exit: 0. Elapsed: 0.074 seconds.

```text
ok · chunk 2 (0.05 s to compute)
```

## stale close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.142 seconds.

```text
the live canvas is in <fixture>/stalesave/out/easel/painting/live.png
closed; the session is in <fixture>/stalesave/paintings/lua/painting.lua
```

## stale explicit save comparison

```text
<source>/scripts/check_painting <fixture>/stalesave <fixture>/stalecheck
```

Exit: 0. Elapsed: 1.037 seconds.

```text
log: <fixture>/stalesave/paintings/lua/painting.lua (2 chunks); the studio's committed record equals its log
resuming chunk 1/2
resumed chunk 1/2 0.07s
resuming chunk 2/2
resumed chunk 2/2 0.05s
resumed 2 chunks from <fixture>/stalecheck/paintings/lua/painting.lua
easel "painting" open: 2 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
the replay equals the live canvas saved at close
the replay's canvas state equals the save written at close
the replay's PNG differs from the painter's last save (saved before the log's last change)
times: reopen 1 s, save and close 0 s
check: ok: 2 chunks replay to the live canvas; the replay equals the live canvas saved at close; the replay's canvas state equals the save written at close; the replay's PNG differs from the painter's last save (saved before the log's last change); the studio's committed record equals its log
```

## absent checkpoint reopen

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.22 seconds.

```text
resuming chunk 1/2
resumed chunk 1/2 0.06s
resuming chunk 2/2
resumed chunk 2/2 0.08s
resumed 2 chunks from <fixture>/stalesave/paintings/lua/painting.lua
easel "painting" open: 2 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## reopened pixels

```text
<binaries>/easel-default save <fixture>/reopened.png
```

Exit: 0. Elapsed: 0.028 seconds.

```text
<fixture>/reopened.png
```

## reopen close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.085 seconds.

```text
the live canvas is in <fixture>/stalesave/out/easel/painting/live.png
closed; the session is in <fixture>/stalesave/paintings/lua/painting.lua
```

## parallel direct finish

```text
<binaries>/easel-default finish <fixture>/live/out/easel/painting/live.ckpt <fixture>/parallel-direct.png --log <fixture>/live/paintings/lua/painting.lua --no-varnish
```

Exit: 0. Elapsed: 0.123 seconds.

```text
wrote <fixture>/parallel-direct.png (restored 6 chunks in 0.0s, finished in 0.1s, total 0.1s)
```

## parallel script finish

```text
<source>/scripts/finish_painting <fixture>/live/paintings/lua/painting.lua <fixture>/parallel-script.png --coats 1.2 --relief
```

Exit: 0. Elapsed: 0.475 seconds.

```text
wrote <fixture>/parallel-script.png (restored 6 chunks in 0.0s, finished in 0.1s, total 0.2s)
finished from the save: <fixture>/parallel-script.png (log: <fixture>/parallel-script.lua; save compressed)
```

## parallel video a

```text
<source>/scripts/replay_clip <fixture>/live/paintings/lua/painting.lua <fixture>/parallel-a.mp4 --reuse --frames-dir <fixture>/frames --length 4 --max-hold 2 --width 160 --pace 1
```

Exit: 0. Elapsed: 0.45 seconds.

```text
3 moments, 0.5 min of hand time, pace 1: about 4.0 s of clip
stream|width=160|height=32|nb_frames=96
format|duration=4.000000|size=4346
```

## parallel video b

```text
<source>/scripts/replay_clip <fixture>/live/paintings/lua/painting.lua <fixture>/parallel-b.mp4 --reuse --frames-dir <fixture>/frames-b --length 5 --max-hold 2 --width 160 --pace .6
```

Exit: 0. Elapsed: 0.452 seconds.

```text
3 moments, 0.5 min of hand time, pace .6: about 5.0 s of clip
stream|width=160|height=32|nb_frames=120
format|duration=5.000000|size=5165
```

## missing dynamic uv

```text
<source>/scripts/replay_clip <fixture>/live/paintings/lua/painting.lua <fixture>/missing-uv.mp4 --reuse --frames-dir <fixture>/frames-b --length 5 --max-hold 2 --pace dynamic
```

Exit: 1. Elapsed: 0.006 seconds.

```text
replay_clip: --pace dynamic needs uv
```

## offline replay

```text
/usr/bin/sandbox-exec -p \(version\ 1\)\(allow\ default\)\(deny\ network\*\) <binaries>/easel-default run <fixture>/source.lua --out <fixture>/offline.png
```

Exit: 0. Elapsed: 0.204 seconds.

```text
chunk   1     0.09s
  chunk   2     0.06s
wrote <fixture>/offline.png (2 chunks, painted in 0.2s, total 0.2s)
```

## companion before render

```text
ls -l <fixture>/slow-finished.lua
```

Exit: 0. Elapsed: 0.006 seconds.

```text
-rw-r--r-- 1 <user> wheel 844 Oct  4 13:14 <fixture>/slow-finished.lua
```

## slow companion finish

```text
<source>/scripts/finish_painting <fixture>/slow.lua <fixture>/slow-finished.png
```

Exit: 0. Elapsed: 2.803 seconds.

```text
chunk   1     0.10s
  chunk   2     0.06s
completed	125000000250000000
  chunk   3     2.23s
  chunk   4     0.11s
wrote <fixture>/slow-finished.png (4 chunks, painted in 2.5s, total 2.5s)
finished: <fixture>/slow-finished.png (log: <fixture>/slow-finished.lua)
```

## halfspace bounded16px

```text
<binaries>/easel-default run halfspace.lua --width 16
```

Exit: terminated or observer result; see output.

```text
still running at5sec=false
  chunk   1     0.00s
  chunk   2     0.14s
wrote <fixture>/halfspace.png (2 chunks, painted in 0.1s, total 0.1s)
```

## unstarted abort

Exit: terminated or observer result; see output.

```text
completed artifact exists=false
```

## destination replacement during run

```text
<binaries>/easel-default run <fixture>/slow.lua --out <fixture>/changed-target.png
```

Exit: 0. Elapsed: 4.064 seconds.

```text
chunk   1     0.26s
  chunk   2     0.19s
completed	125000000250000000
  chunk   3     3.56s
wrote <fixture>/changed-target.png (3 chunks, painted in 4.0s, total 4.1s)
```

## destination final bytes

Exit: 0.

```text
89504e470d0a1a0a
```

## independent exported files

Exit: 0.

```text
other journal unchanged=true
```

## missing build cargo

```text
<source>/scripts/export_r16_studio blank <fixture>/no-cargo-export
```

Exit: 1. Elapsed: 0.138 seconds.

```text
<source>/scripts/export_r16_studio: line 120: grep: command not found
<source>/scripts/export_r16_studio: line 118: cargo: command not found
```

## checkpoint source mismatch

```text
<binaries>/easel-default finish <fixture>/stalesave/out/easel/painting/live.ckpt <fixture>/mismatch.png --log <fixture>/wait.lua
```

Exit: 1. Elapsed: 0.032 seconds.

```text
<fixture>/stalesave/out/easel/painting/live.ckpt is not a save of <fixture>/wait.lua (2 chunks in the save, 4 in the log, or the text differs)
```

## selected A

```text
<binaries>/easel-default open a
```

Exit: 0. Elapsed: 0.113 seconds.

```text
easel "a" open: 0 chunks · 2400px · no canvas yet
```

## selected A canvas

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}
```

Exit: 0. Elapsed: 0.087 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## selected B

```text
<binaries>/easel-default open b
```

Exit: 0. Elapsed: 0.105 seconds.

```text
easel "b" open: 0 chunks · 2400px · no canvas yet
```

## selected B canvas

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}\;p\=pile\{\{\"red\ earth\",1\}\}\;b\=brush\(\"round\",4\)\;b:load\(p,0.8\)\;b:stroke\(\{\{100,80\},\{600,80\}\}\)
```

Exit: 0. Elapsed: 0.139 seconds.

```text
ok · chunk 1 (0.13 s to compute)
```

## explicit selected A save

```text
<binaries>/easel-default -s a save <fixture>/selected-a.png
```

Exit: 0. Elapsed: 0.028 seconds.

```text
<fixture>/selected-a.png
```

## default selected B save

```text
<binaries>/easel-default save <fixture>/selected-b.png
```

Exit: 0. Elapsed: 0.029 seconds.

```text
<fixture>/selected-b.png
```

## close A

```text
<binaries>/easel-default -s a close
```

Exit: 0. Elapsed: 0.088 seconds.

```text
the live canvas is in <fixture>/selected/out/easel/a/live.png
closed; the session is in <fixture>/selected/paintings/lua/a.lua
```

## close B

```text
<binaries>/easel-default -s b close
```

Exit: 0. Elapsed: 0.092 seconds.

```text
the live canvas is in <fixture>/selected/out/easel/b/live.png
closed; the session is in <fixture>/selected/paintings/lua/b.lua
```

## failed disposable replay

```text
<binaries>/easel-default run <fixture>/failed.lua --out <fixture>/failed.png
```

Exit: 1. Elapsed: 0.133 seconds.

```text
chunk   1     0.07s
  chunk   2     0.05s
chunk 3 failed:
runtime error: [string "chunk 3"]:1: late replay error
```

## failed replay no session

```text
<binaries>/easel-default status
```

Exit: 1. Elapsed: 0.004 seconds.

```text
no session: easel open <name> first (or pass -s <name>)
```

## interrupted encoder partial

Exit: terminated or observer result; see output.

```text
bytes=41728
```

## impossible duration

```text
<source>/scripts/replay_clip <fixture>/source.lua <fixture>/impossible.mp4 --reuse --frames-dir <fixture>/frames --length 1000 --max-hold 1
```

Exit: 1. Elapsed: 0.015 seconds.

```text
replay_clip: --length 1000 is longer than the clip can run: 2 moments held at most 1 s each (--max-hold) plus the 3 s final hold come to at most 5.00 s; raise --max-hold to at least 498.50 or lower --length
```

## peek whole

```text
<source>/scripts/peek <fixture>/wet.png <fixture>/peek.png
```

Exit: 0. Elapsed: 0.365 seconds.

```text
151729 <fixture>/peek.png
```

## peek crop

```text
<source>/scripts/peek <fixture>/wet.png <fixture>/peek-crop.png 100 200 0 0
```

Exit: 0. Elapsed: 0.069 seconds.

```text
4645 <fixture>/peek-crop.png
```

## independent field fall

```text
<binaries>/easel-default run <fixture>/field-only-fall.lua --out <fixture>/field-only-fall.png
```

Exit: 0. Elapsed: 0.165 seconds.

```text
chunk   1     0.06s
  chunk   2     0.00s
  chunk   3     0.06s
wrote <fixture>/field-only-fall.png (3 chunks, painted in 0.1s, total 0.2s)
```

## independent field across

```text
<binaries>/easel-default run <fixture>/field-only-across.lua --out <fixture>/field-only-across.png
```

Exit: 0. Elapsed: 0.165 seconds.

```text
chunk   1     0.06s
  chunk   2     0.00s
  chunk   3     0.06s
wrote <fixture>/field-only-across.png (3 chunks, painted in 0.1s, total 0.2s)
```

## independent field edge

```text
<binaries>/easel-default run <fixture>/field-only-edge.lua --out <fixture>/field-only-edge.png
```

Exit: 0. Elapsed: 0.168 seconds.

```text
chunk   1     0.06s
  chunk   2     0.00s
  chunk   3     0.07s
wrote <fixture>/field-only-edge.png (3 chunks, painted in 0.1s, total 0.2s)
```

## pure references image

```text
<binaries>/easel-default run <fixture>/pure-reference.lua --out <fixture>/pure-reference.png
```

Exit: 0. Elapsed: 0.111 seconds.

```text
chunk   1     0.07s
3769.444144875933
  chunk   2     0.01s
wrote <fixture>/pure-reference.png (2 chunks, painted in 0.1s, total 0.1s)
```

## artifact assertions

Exit: 0.

```text
{
  "save_equals_replay": true,
  "save_replaced_and_paths": true,
  "pure_reference_no_paint": true,
  "reopen_equal": true,
  "frames": [
    "0003.png",
    "0005.png"
  ]
}
```

## sargent open

```text
<fixture>/export-sargent/bin/easel open
```

Exit: 0. Elapsed: 0.11 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## sargent canvas

```text
<fixture>/export-sargent/bin/easel do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}
```

Exit: 0. Elapsed: 0.081 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## sargent tube stroke

```text
<fixture>/export-sargent/bin/easel do b\=brush\(\"round\",4\)\;b:load\(pile\{\{\"Indian\ yellow\",1\}\}\)\;b:stroke\(\{\{100,80\},\{600,80\}\}\)
```

Exit: 0. Elapsed: 0.068 seconds.

```text
ok · chunk 2 (0.05 s to compute)
```

## sargent close

```text
<fixture>/export-sargent/bin/easel close
```

Exit: 0. Elapsed: 0.098 seconds.

```text
the live canvas is in <fixture>/export-sargent/out/easel/painting/live.png
closed; the session is in <fixture>/export-sargent/paintings/lua/painting.lua
```

## recorded box replay

```text
<binaries>/easel-default run <fixture>/export-sargent/paintings/lua/painting.lua --out <fixture>/sargent-replay.png
```

Exit: 0. Elapsed: 0.147 seconds.

```text
chunk   1     0.06s
  chunk   2     0.05s
wrote <fixture>/sargent-replay.png (2 chunks, painted in 0.1s, total 0.1s)
```

## conflicting recorded box

```text
<binaries>/easel-default run <fixture>/export-sargent/paintings/lua/painting.lua --out <fixture>/wrongbox.png
```

Exit: 1. Elapsed: 0.003 seconds.

```text
<fixture>/export-sargent/paintings/lua/painting.lua: the log was painted from the box "sargent" (its --@ box line), but EASEL_BOX says "inness": a painting is replayed with the box it was painted from; nothing ran
```

## unaffected open

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.108 seconds.

```text
easel "painting" open: 0 chunks · 2400px · no canvas yet
```

## unaffected canvas

```text
<binaries>/easel-default do canvas\{size\=100,aspect\=5,linen\=12,ground\=\{\{pile\=\{\{\"lead\ white\",1\}\},um\=40,apply\=\"knife\"\}\}\}
```

Exit: 0. Elapsed: 0.079 seconds.

```text
ok · chunk 1 (0.07 s to compute)
```

## unaffected before

```text
<binaries>/easel-default save <fixture>/unaffected-before.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/unaffected-before.png
```

## disposable replay with live other

```text
<binaries>/easel-default run <fixture>/source.lua --out <fixture>/other-replay.png
```

Exit: 0. Elapsed: 0.151 seconds.

```text
chunk   1     0.06s
  chunk   2     0.05s
wrote <fixture>/other-replay.png (2 chunks, painted in 0.1s, total 0.1s)
```

## unaffected after

```text
<binaries>/easel-default save <fixture>/unaffected-after.png
```

Exit: 0. Elapsed: 0.027 seconds.

```text
<fixture>/unaffected-after.png
```

## unaffected close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.082 seconds.

```text
the live canvas is in <fixture>/unaffected/out/easel/painting/live.png
closed; the session is in <fixture>/unaffected/paintings/lua/painting.lua
```

## box and live comparisons

Exit: 0.

```text
{
  "box_live_equals_replay": true,
  "unaffected_live": true
}
```

## close failure reopen

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.11 seconds.

```text
resuming chunk 1/1
resumed chunk 1/1 0.06s
resumed 1 chunks from <fixture>/closefail/paintings/lua/painting.lua
easel "painting" open: 1 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## close failure full reply

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.032 seconds.

```text
the live canvas couldn't be saved: <fixture>/closefail/out/easel/painting/live.png: Is a directory (os error 21)
closed; the session is in <fixture>/closefail/paintings/lua/painting.lua
```

## restricted final open

```text
<fixture>/export-blank/bin/easel open
```

Exit: 0. Elapsed: 0.113 seconds.

```text
resuming chunk 1/1
resumed chunk 1/1 0.06s
resumed 1 chunks from <fixture>/export-blank/paintings/lua/painting.lua
easel "painting" open: 1 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## restricted absent varnish{}

```text
<fixture>/export-blank/bin/easel do varnish\{\}
```

Exit: 1. Elapsed: 0.014 seconds.

```text
runtime error: [string "chunk 2"]:1: attempt to call a nil value (global 'varnish')
(the chunk failed and changed nothing)
```

## restricted absent cracks{}

```text
<fixture>/export-blank/bin/easel do cracks\{\}
```

Exit: 1. Elapsed: 0.008 seconds.

```text
runtime error: [string "chunk 2"]:1: attempt to call a nil value (global 'cracks')
(the chunk failed and changed nothing)
```

## restricted absent relief()

```text
<fixture>/export-blank/bin/easel do relief\(\)
```

Exit: 1. Elapsed: 0.007 seconds.

```text
runtime error: [string "chunk 2"]:1: attempt to call a nil value (global 'relief')
(the chunk failed and changed nothing)
```

## restricted final close

```text
<fixture>/export-blank/bin/easel close
```

Exit: 0. Elapsed: 0.087 seconds.

```text
the live canvas is in <fixture>/export-blank/out/easel/painting/live.png
closed; the session is in <fixture>/export-blank/paintings/lua/painting.lua
```

## check permission denied

```text
<source>/scripts/check_painting <fixture>/valid <fixture>/check-denied/work
```

Exit: 1. Elapsed: 0.015 seconds.

```text
mkdir: cannot create directory ‘<fixture>/check-denied/work’: Permission denied
mkdir: cannot create directory ‘<fixture>/check-denied/work’: Permission denied
mkdir: cannot create directory ‘<fixture>/check-denied/work’: Permission denied
```

## concurrent selected ref past

```text
<source>/scripts/export_r16_studio blank <fixture>/ref-past
```

Exit: 1. Elapsed: 0.324 seconds.

```text
easel: no command "tubes"

easel: a live painting session (see notes/easel_guide.md)

  easel open          start or reattach; replays paintings/lua/painting.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,mirror] [--grid [step]] [--size 1000]
  easel log           the painting so far (= paintings/lua/painting.lua)
  easel status        chunks, width, canvas
  easel save [path]   the canvas as a PNG (default out/easel/painting/painting.png)
  easel frames on|off save a look after every chunk
  easel check         replay the log from scratch and compare with the live canvas
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
the exported easel can't name its box
```

## concurrent selected ref current

```text
<source>/scripts/export_r16_studio blank <fixture>/ref-current
```

Exit: 0. Elapsed: 1.84 seconds.

```text
exported the blank studio from 4e525e5 (4e525e5) to <fixture>/ref-current
  easel: <fixture>/ref-current/bin/easel (checked in $TMPDIR//studio-probe.TOX7tQ)
  notices: <fixture>/ref-current/bin/THIRD_PARTY_NOTICES.md
  box: the default tube box
  notes: easel_guide.md journal.md research/oil_paint_physics.md
```

## crop open

```text
<binaries>/easel-default open painting
```

Exit: 0. Elapsed: 0.213 seconds.

```text
resuming chunk 1/2
resumed chunk 1/2 0.07s
resuming chunk 2/2
resumed chunk 2/2 0.05s
resumed 2 chunks from <fixture>/stalesave/paintings/lua/painting.lua
easel "painting" open: 2 chunks · 2400px · size=100, aspect=5, linen={12, 12}, seed=1
```

## cropped look

```text
<binaries>/easel-default look --crop 100,60,300,120
```

Exit: 0. Elapsed: 0.014 seconds.

```text
<fixture>/stalesave/out/easel/painting/look-0001.png (480x144, 0.01s)
```

## full save after crop

```text
<binaries>/easel-default save <fixture>/full-after-crop.png
```

Exit: 0. Elapsed: 0.026 seconds.

```text
<fixture>/full-after-crop.png
```

## crop close

```text
<binaries>/easel-default close
```

Exit: 0. Elapsed: 0.098 seconds.

```text
the live canvas is in <fixture>/stalesave/out/easel/painting/live.png
closed; the session is in <fixture>/stalesave/paintings/lua/painting.lua
```

## Storage and restricted-build probes

A24MB mounted HFS image was filled, then128KB freed before the333613-byte PNG write. It returned ENOSPC and left159744bytes. Only the isolated image was filled. Supplemental transcript (a concurrent log writer omitted these rows from the JSON; this captured process transcript is retained):

```text
ENOSPC partial PNG: 1 <isolated-quota>/partial.png: No space left on device (os error 28)
ENOSPC artifact: 0 -rw-r--r-- 1 <user> staff 159744 Oct  4 13:11 <isolated-quota>/partial.png
close failure open: 0 easel "painting" open: 0 chunks · 2400px · no canvas yet
close failure canvas: 0 ok · chunk 1 (0.07 s to compute)
close failure: 0 closed; the session is in <fixture>/closefail/paintings/lua/painting.lua
PNG is not session: 1 <fixture>/wet.png: stream did not contain valid UTF-8
painter open: 0 easel "painting" open: 0 chunks · 2400px · no canvas yet
painter canvas: 0 ok · chunk 1 (0.07 s to compute)
painter varnish{}: 1 (the chunk failed and changed nothing)
painter cracks{}: 1 (the chunk failed and changed nothing)
painter relief(): 1 (the chunk failed and changed nothing)
painter close: 0 closed; the session is in <fixture>/export-blank/paintings/lua/painting.lua
```

The close-failure fixture created a directory at `out/easel/painting/live.png`; close reported its save failure while retaining the Lua log. The exported painter binary actually attempted each absent finishing global.
## Image analysis

Independent Pillow decoding and SHA256 comparisons:

```json
{
  "wet": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "4f47cfbcc610c9abc851ea2ba85759c6164bb621a713dba0e4e6b68aa7537b3c"
  },
  "replay": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "4f47cfbcc610c9abc851ea2ba85759c6164bb621a713dba0e4e6b68aa7537b3c"
  },
  "small": {
    "size": [
      160,
      32
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "fe1c8731781fac540209227eebac23bcf8e6f685af92e5454fc4c767536aef3f"
  },
  "default-second": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "4f47cfbcc610c9abc851ea2ba85759c6164bb621a713dba0e4e6b68aa7537b3c"
  },
  "wide": {
    "size": [
      9600,
      1920
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "609490c1c40e5ae0e0080ba9964054e4f6604095937e03943022a0dc6bb89ff4"
  },
  "finish-default": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "d8072efbcbe895c0fa527a651b797ab5118abb70b3663fe805b06e7b2dda46e4"
  },
  "finish-no-varnish": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "716480fe8daa693738c8f75155600bfc1cd954c95cecd70602dfcb4b5da64d3a"
  },
  "finish-no-cracks": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "a529885e58c7daf3c2fcaf122fbea2bd7658487242910e77334ab9f579db3f93"
  },
  "finish-relief": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "2364f9b6c3dbaf539a6e6bd8cea132ea61a114e3f01ec3c109ff4957de933c26"
  },
  "finish-coats": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "d2135817e778729db648159721770c3001162d0ac003683b12520497c4b61e2c"
  },
  "offline": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "4f47cfbcc610c9abc851ea2ba85759c6164bb621a713dba0e4e6b68aa7537b3c"
  },
  "reopened": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "4f47cfbcc610c9abc851ea2ba85759c6164bb621a713dba0e4e6b68aa7537b3c"
  },
  "selected-a": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "e095c2338960dcb3ae735c26dc259cd829b3675f1274eeab2d81eef69f82e627"
  },
  "selected-b": {
    "size": [
      2400,
      480
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "fa6e6d80d33e64850e3c562351505929ebe4c6094e7755e7252ac4a243b2c98d"
  },
  "fixed-a": {
    "size": [
      160,
      32
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "fe1c8731781fac540209227eebac23bcf8e6f685af92e5454fc4c767536aef3f"
  },
  "fixed-b": {
    "size": [
      320,
      64
    ],
    "mode": "RGB",
    "info": {},
    "sha256": "22ea75ffc35d154a0c1d49a9ae540f7ade747d3fb6b064d926db8282e7175a8b"
  },
  "small_is_resize": false,
  "finish_all_different": true,
  "frames": {
    "00004.png": [
      160,
      32
    ],
    "00005.png": [
      160,
      32
    ],
    "00007.png": [
      160,
      32
    ],
    "00006.png": [
      160,
      32
    ],
    "00002.png": [
      160,
      32
    ],
    "00003.png": [
      160,
      32
    ],
    "00001.png": [
      160,
      32
    ]
  }
}
```
## Static and runnable exports

Unmodified `studio/export_static.py` was imported, its existing `studio.SESSIONS` setting pointed at the committed synthetic sessions fixture and `main()` invoked with the output directory. No source edits or provider calls.

```text
paint-studio-decafe: 9 events, 2 images
exported 1 painters, 2 images (2 new web copies) to /private/tmp/cm.B7260e/delivery/static (0 stale files removed)
```

The static artifact contains `index.html`, `stream.css`, `data/sessions.json`, per-painter `events.json`, original images and JPEG thumbnail/view copies. The actual runnable exports contain `bin/easel`, notices, notes and `paintings/lua`, with no source tree. These are different artifacts.

## Source log preservation

The initial source log was SHA256-hashed before replay. The post-replay comparison printed `source preserved true`; replay, save and finishing outputs used distinct paths. The finishing companion preserves the source prefix and adds its finishing chunk. Reopening after the initial two chunks and two saves still reported2 chunks.

## Restricted state

The prior runtime pass tried varnish, cracks and relief on wet paint and recorded all three refusals with unchanged live pixels. This pass exercised default finishing, skipped varnish, skipped cracks, relief and coat1.2 on closed wet copies. All recipes succeeded after their generated drying loop. See [earlier wet refusals](runtime-delivery.md#replay-and-comparison). The known checkpoint-counter defect is separate from pixel rollback.

## Browser PNG interpretation

Independent Pillow RGBA decode:

```json
{"dimensions": [2400, 480], "rgba_sha256": "834fc33fe62aed4f266758729a287a54859385f1fb89b5f5fba5b4b183c72a5f", "pixels": {"(0, 0)": [233, 229, 219, 255], "(300, 192)": [156, 74, 48, 255], "(500, 192)": [175, 74, 48, 255], "(1200, 240)": [234, 230, 219, 255], "(2399, 479)": [234, 230, 219, 255]}}
```

Actual Chromium decoded the same2400×480 PNG, reported `colorSpace: srgb` and matched all five samples exactly. Browser cryptographic hashing was unavailable on the private HTTP origin, so no full-browser-byte hash is claimed. [Browser receipt](matrix-viewer.md#actual-export-and-png-decoding).

## Bounded loop reproducer

The generated production finishing chunk ran with these public Lua overrides. The assertion tests its retry bound; the final CLI no-canvas error is expected because this deliberately makes no physical canvas.

```lua
--@ chunk 1
count=0;wait=function(n)assert(n==43200);count=count+1 end;varnish=function()error('not all dry')end;local ok,e=pcall(function()
-- finishing, applied after the session by scripts/finish_painting
local function when_dry(f)
  for _ = 1, 120 do
    local ok, e = pcall(f)
    if ok then return end
    if not tostring(e):find('not all dry', 1, true) then error(e, 0) end
    wait(30 * 24 * 60)
  end
  error('still not dry after ten years', 0)
end
when_dry(function() varnish{coats=0.4} end)
when_dry(function() cracks{} end)

end);print(ok,e,count);assert(count==120 and not ok)
```

## Diagnostic and frame artifacts

Two-chunk state digest:

```text
chunk 1 secs=0.067 canvas=18889facb1b1dae7 brushes=cbf29ce484222325 nbrushes=0 studio=c4c3947573e943d9
chunk 2 secs=0.051 canvas=dea09bd0fd085a7a brushes=2d437f0e05823031 nbrushes=1 studio=a818c45b80ae4bde
```

Wait-excluding frame manifest (chunk3 is the43200-minute drying wait and retains hand_secs25.2):

```text
file	hand_secs	ticks	chunks_done	kind
00001.png	0.0	0	1	chunk
00002.png	25.2	125	1	tick
00003.png	25.2	0	2	chunk
00004.png	25.2	0	3	chunk
00005.png	30.3	26	3	tick
00006.png	30.3	0	4	chunk
00007.png	30.3	0	4	final
```

Artifact sizes and independent hashes:

```json
{
  "parallel-a.png": {
    "bytes": 333561,
    "sha256": "ad88777df3af13bc7cf8cf0f7f7fdd7d58056e17280e8b2d06d921650b9dbe7f"
  },
  "parallel-b.png": {
    "bytes": 333561,
    "sha256": "ad88777df3af13bc7cf8cf0f7f7fdd7d58056e17280e8b2d06d921650b9dbe7f"
  },
  "fixed-a.tsv": {
    "bytes": 309,
    "sha256": "05d696dae55d5bab0dc847bd50c995a5878d177f0f10b2ebee9820861546734a"
  },
  "fixed-b.tsv": {
    "bytes": 309,
    "sha256": "820cf4c3b4acf5caba02b008ed2239c90adb8a18236451a922f74553dcf0c86d"
  },
  "fixed-a.bin": {
    "bytes": 20480,
    "sha256": "ab95ff0aaf9d19d96f228e00bf1a2e0c2134511791bc200fe94265fad803f14b"
  },
  "fixed-b.bin": {
    "bytes": 81920,
    "sha256": "11e58a381979a3061812e7d13bc2b3d66bc679728bc385e54307b0c7ff093132"
  }
}
```
