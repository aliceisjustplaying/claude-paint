# Background display companions

`EASEL_ASYNC_DISPLAY=1` opts a live oil session into background Gallery and
Relief rendering. It is read when the server starts. The painter still gets
the same Color view, with an opaque, stable filename. Without the switch,
the existing synchronous path and numbered filenames remain available.

The TypeScript and MCP clients forward this switch and
`EASEL_DISPLAY_THREADS` from their operator environment. The latter defaults
to at most four workers and cannot exceed the foreground Rayon worker
budget (`RAYON_NUM_THREADS`). Replays and clip rendering do not use this pool.

One pool is shared by the painting and its scratch canvas. A job captures
owned prepared optics before the reply, not a material Canvas or a serialized
checkpoint. At most two captures can be in flight, including a capture being
built. A third look waits for capacity before allocating. Jobs run in order,
so every accepted look keeps its own complete companions rather than having
its history replaced by a newer revision.

Each Color PNG has a matching `.display/<filename>` record. Clients preserve
its filename even if the companions have not arrived. Gallery and Relief
PNGs are written through temporary files; the record becomes `ready` only
after both are complete. Errors and cancellation leave the record `failed`.
The viewer discovers completion even when no new session-log bytes arrive.
In a lit view it keeps the last completed revision, including across scratch
resets. Color can show the painter's new reply immediately. A failed request
for an asynchronously published light does not fall back to Color.

Closing drains accepted display work before returning. Other server teardown
cancels queued work and joins the worker. Cancellation is checked between
lighting kernels and PNGs, not inside a lighting kernel. A forcibly killed
process can leave a `pending` record; that revision stays out of lit display.

The checkpoint-backed latency pilot is an ignored test. For a retained oil
checkpoint, set `EASEL_DISPLAY_BENCH_CHECKPOINT` and a fresh
`EASEL_DISPLAY_BENCH_OUTPUT` directory, then run
`cargo test --release -p easel --bin easel measure_server_look_latency -- --ignored --nocapture`.
It measures five actual `Server::look` calls after restoration, reporting
reply wall time and wall time through completion separately. It does not
measure checkpoint loading, replay, socket transport or subsequent painting.
