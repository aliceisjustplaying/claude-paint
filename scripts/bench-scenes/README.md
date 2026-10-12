# Exact-state timing report

Run `scripts/bench_exact` or `scripts/test --bench-exact`. Both report timings and per-chunk state digests for r19, mixed passes, a thinned wash followed by a wait and a brushed ground at 600 and 2400 pixels. There are no timing thresholds and this report is separate from the test gate.

The command builds the release easel, runs one scene at a time at low priority and deletes its PNGs and digest files. It requires 15 GiB of free disk space. Use `--easel PATH` to measure an existing binary and `--output FILE` to retain the JSON report. `--scenes sampler` adds a small p9-style sampler plate for parity checks. The scenes use a short 4:1 canvas to keep the report small; this is not a finished-painting benchmark.

The report disables the prepared-ground cache so repeated runs measure painting work. To measure cache cold/warm behavior, run the easel directly with `EASEL_GROUND_CACHE=DIR`. The default cache is `$XDG_CACHE_HOME/claude-paint/grounds` or `~/.cache/claude-paint/grounds`, partitioned by the executable's FNV build identity (the same identity recorded in `live.txt`). `EASEL_GROUND_CACHE=off` disables it. Only brushed preparations are cached. Keys hash the pixel frame and crop, seed, physical size, raw color, linen, complete ground stack, palette name and both the requested and clamped preparation engines. Cache files carry a SHA-256 checksum and a matching key header; read/write failures fall back to preparation. A different executable never reads an earlier build's entries.

Scratch buffers are reused per painting thread, outside checkpoint state and canvas clones. They retain the largest solvent, drying and blur working regions seen on that thread until the process exits. No arithmetic or summation order is changed.
