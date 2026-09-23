# Easel review, round 3

1. **High — `--undo 0` disables failure rollback, not just undo history.**

   **Source:** `crates/easel/src/session.rs:129–130,149–152`; the live option is accepted at `crates/easel/src/main.rs:162`. The server nevertheless promises rollback at `main.rs:306`.

   `Session::run` treats an undo depth of zero as proof that this is a disposable replay session. A live session can also have zero undo depth. A failed chunk then leaves its paint, globals, brush changes and clock changes in memory without logging them. Continuing or saving uses state that reopening the log cannot recover. This contradicts the unconditional failure guarantee in `crates/easel/README.md:65–69`.

   **Evidence:** `~/tmp/review-easel-a5287622/undo-zero.log`. Started a scratch session with:
   ```sh
   timeout 1800 target-easel/release/easel serve ~/tmp/review-easel-a5287622/zero --width 80 --undo 0
   ```
   In another shell, with `EASEL_SESSION` set to that absolute scratch session name:
   ```sh
   timeout 30 target-easel/release/easel do 'canvas{}; a=1'
   timeout 30 target-easel/release/easel do 'a=2; glaze(everywhere(), {color="#ff0000", coats=0.5}); error("stop")'
   timeout 10 target-easel/release/easel do 'print(a)'
   timeout 30 target-easel/release/easel check
   ```
   Output includes `(rolled back: the canvas is as it was before this chunk)`, then `2`, then `replay DIFFERS from the live canvas`. The failed glaze survives.

   **Suggested fix:** separate disposable replay mode from undo retention. Live sessions need a pre-chunk snapshot even when they retain zero successful-chunk snapshots. Add a live-depth-zero regression covering failed painting and Lua mutations.

2. **Medium — a fixed string hash seed does not make object-keyed `pairs` deterministic.**

   **Source:** `crates/easel/src/session.rs:232–271` only creates and checks fixed string-key hashing; `crates/easel/README.md:99–100` promises the same table traversal order in every session and replay without restricting key types.

   Lua hashes table keys by object address, not the fixed string hash seed. Tables keyed by painter objects or Lua tables therefore still have process-dependent iteration order. Any order-sensitive painting driven by that traversal can replay differently, even with no failed or undone chunks.

   **Evidence:** `~/tmp/review-easel-a5287622/object-pairs.lua`:
   ```lua
   --@ chunk 1
   canvas{aspect=1,seed=7}
   items={}
   for i=1,32 do items[{index=i}]=i end
   --@ chunk 2
   local first=next(items)
   print("first",first.index)
   glaze(everywhere(),{color=rgb(first.index*7,0,0),coats=1})
   ```
   `next(items)` selects the first entry that ordinary `pairs(items)` would visit. Ran in three fresh processes:
   ```sh
   S=~/tmp/review-easel-a5287622
   for i in 1 2 3; do
     timeout 30 target-easel/release/easel run "$S/object-pairs.lua" --width 80 --out "$S/object-pairs-$i.png" > "$S/object-pairs-$i.log" 2>&1
   done
   timeout 10 shasum -a 256 "$S"/object-pairs-*.png
   ```
   The first indices were `10`, `21` and `25`. All three PNG hashes differ:
   ```text
   46f55d5d79f320fc185dd7a4553daea2b87830f0c8dd34fca4a7526801988465
   b3443a8648942bc98219db93c383edb5d0fd60ee8c2f42b5a28c5522e162c2e1
   068e8927dc957d33dcea075601bbfaaa477dc3cf42e174203cb6cd17679cc5ce
   ```
   **Suggested fix:** narrow the determinism contract to supported key types and direct painters to ordered arrays/`ipairs` for object collections. If arbitrary table-key traversal must be deterministic, provide stable object identities and deterministic iteration rather than relying on Lua's seed. Add a cross-process test.

3. **Medium — failed chunks permanently advance built-in string iterators.**

   **Source:** `crates/easel/src/heap.lua:42–53` explicitly skips C functions and `heap.lua:18` does not walk userdata. The promise at `crates/easel/README.md:65–68` includes closures from earlier chunks; the documented limitations at `README.md:488–490` do not mention C iterators.

   `string.gmatch` returns a stateful C closure. Calling a saved iterator advances state that the heap snapshot neither records nor restores. This is a rollback gap beyond the already documented coroutine/table-order limits, and it can change later painting.

   **Evidence:** `~/tmp/review-easel-a5287622/gmatch-paint.log`, using a live session at width 80 with the normal undo depth of 8 and an existing `canvas{}`:
   ```sh
   timeout 10 target-easel/release/easel do 'it=string.gmatch("red green blue", "%a+")'
   timeout 10 target-easel/release/easel do 'print(it()); error("stop")'
   timeout 30 target-easel/release/easel do 'local word=it(); print(word); glaze(everywhere(), {color=word == "green" and "#00ff00" or "#ff0000", coats=0.5})'
   timeout 30 target-easel/release/easel check
   ```
   The failing chunk prints `red`; after purported rollback the next successful chunk prints `green` instead of `red`. `check` reports `replay DIFFERS from the live canvas`.

   **Suggested fix:** wrap stateful library iterators in rollback-aware state, or explicitly prohibit retaining these iterators across chunks and narrow the exact-rollback guarantee. Copying C upvalue references alone is not enough when the referenced iterator state is mutable userdata. Add failure and undo tests for `string.gmatch`.

4. **Low — a view's form reports shadow proxies as visible form parts.**

   **Source:** `crates/easel/src/world.rs:194,504` initializes `FormU.parts` from the total world body count. However, `crates/paint/src/scene.rs:819–828` adds a form part only for visible bodies. `crates/easel/src/form.rs:243,278,288` exposes and uses this count.

   A proxy-only world returns `v.form.parts == 1` despite having no visible form part (`v:part(1) == 0`). Painters enumerating parts receive nonexistent part IDs; mixed visible/proxy scenes overreport the count too.

   **Evidence:** `~/tmp/review-easel-guide-e5917982/bindings.lua` and `bindings.log`:
   ```lua
   local w = world{horizon=300}
   local s = w:spot_at(0, 10)
   w = w:proxy(s, body.ellipsoid(s:p(0, 1, 0), s:size(1, 1, 1)))
   local v = w:view()
   print("proxy-only form parts", v.form.parts, "proxy part", v:part(1))
   ```
   Reproduction command:
   ```sh
   timeout 60 target-easel/release/easel run ~/tmp/review-easel-guide-e5917982/bindings.lua --width 100 --out ~/tmp/review-easel-guide-e5917982/bindings.png
   ```
   Output: `proxy-only form parts 1 proxy part 0`.

   **Suggested fix:** derive the count from actual visible form parts, not all world bodies. Test proxy-only worlds and mixtures of visible bodies and proxies.

## Verification and limits

- `CARGO_TARGET_DIR=target-easel timeout 240 cargo test -p easel --release`: all three existing tests passed. Receipt: `~/tmp/review-easel-a5287622/tests.log`. Release binary rebuilt with the same target directory (`build.log`).
- All three shipped guide studies (`example.lua`, `rocks.lua`, `meadow.lua`) ran at width 100. Receipts: `~/tmp/review-easel-guide-e5917982/{example,rocks,meadow}.log`. Selected palette/world reference expressions were exercised in `bindings.lua`; this is not a claim that every illustrative reference block is independently executable.
- The ten-chunk `example.lua` also ran at width 160. A separately resumed live session's saved PNG compared byte-identical with `easel run`, and `check` passed. Combined value/squint/mirror, crop, dried and relief look options produced a JPEG. Receipt: `~/tmp/review-easel-a5287622/example-check.log`.
- A three-process probe produced identical string-key traversal, sort comparison counts and PNGs. The Lua color callback stayed strictly within the canvas and its whole-canvas grid sampled 251,001 nodes at the documented 2-unit spacing. Receipts: `~/tmp/review-easel-a5287622/determinism.lua` and `determinism-{1,2,3}.log`. This does not establish determinism for all key types, sorting comparators or crop-sensitive fields.
- No 1000px/3200px replay-equality claim was independently rechecked in this review. No tracked files were edited; scratch sessions were closed. `git diff --exit-code` passed after testing (empty `~/tmp/review-easel-a5287622/tracked-diff.log`; untracked build directories listed in `git-status.log`).
