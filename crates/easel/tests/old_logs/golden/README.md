# Goldens of the old-log cases (scripts/tests/old_logs.sh)

Each `<case>.txt` is the PNG's sha256 and the per-chunk `--state-digest`
lines (`secs=` dropped) of one case in `../cases.tsv`, as af49348's
unchanged release easel replays it. Recorded under decision 2 (option B):
numbers computed by the unchanged af49348 code in a release build, to be
checked by a reviewer who is not the builder (the speed agent built them).

- easel: `cargo build --release -p easel` at af49348 (crates, Cargo.lock and
  .cargo identical to af49348; notes/thinner/baseline/MANIFEST.md, build B),
  binary sha256 `4201cec1f9ddcc8bcd48106ab5b36a3e337bb9619f138f384eb0e81b1ae040f6`,
  rustc 1.97.1, release profile, CFLAGS `-Dluai_makeseed()=0x5eedu`.
- command (2026-10-04 03:04 BST, 4.9 s, `notes/speed/logs/p2_old_logs_record.log`):
  `scripts/lockrun --timeout 60 -- scripts/tests/old_logs.sh <that easel> --record crates/easel/tests/old_logs/golden`
- the same check with the dumper-branch release easel (a5f37758…, codex/speed
  a97c3a6) passes all ten, as does the codex/speed test run.
- inputs: `engine1_tiny.lua` is synthetic; the other `*.firstN.lua` are the
  first N chunks of the logs named in `../cases.tsv`, cut with
  `notes/speed/tools/log_prefix.py <log> N <out>` (head kept, bytes
  unchanged); r19 is `../r19_default_box.lua` itself.

sha256 of the goldens:

```
718a0bea0871e7c6488d1b28808142c9ea8e704a30e9b0496af536dd6211aec8  easel3_free.txt
8dfcb5bf69a6103988fd709bcf5e265fce38a981bd932f8a5151b8ea2fe5ff1f  easel3_green.txt
84076ba47bc3441af024c493d4ca05024418e54ba21a44d284fc9d0e1ab71451  easel3_near.txt
08b36463d1f6e3480c5ac72f35835100a226aefa36132a4d40ee51f738cc3a0d  easel4_free.txt
9556fc583973957a719aa8d87c4650ac77f87213208f30a4278df0f34220e7db  easel4_green.txt
76434325d5063bba1f517a9002e4a90a934a7457553a585d9d3f8cbcaaa05cc2  easel4_near.txt
e5a8506ebc61266b2d574d1cf8449c6132554666e650ad9b0eee90ed3312d8b5  engine1_tiny.txt
7bba65b9571e03e8293d5a8b0b6e226a150393b74333b63b25c75d3a3aece803  r19.txt
4f6913542bc281f37a250f91bc00dc34ced60b7f121899d8a2607640f1c1733b  studio_6399ad.txt
c306bdc2079ab54ee7e8c767f2b8fd7a5d848be7348e0822c8011f6feca08392  studio_db6324.txt
```

sha256 of the inputs:

```
004e1f94a9c80e4126a7edc3de9b7be01697068ddc3733b38ae64802c15b0712  easel3_free.first3.lua
a9ba1510252b2b3b140ee83a0c6a5fc7bad23cf016bd94ba7596e2db857bc017  easel3_green.first3.lua
6fe66135eba6ab2f101c6bb561bfe5ebf39c11835fc8788786cea049b28a63fb  easel3_near.first2.lua
8348868c3cac0b4a67057f08e74d74398e6bda5d213249f027eaf126464a654e  easel4_free.first3.lua
b6a3ff61d18398e30a614a4941c975128e2e6120f6c3508461c8af038f8c778e  easel4_green.first3.lua
157d37784a7c6e6dccbba2c6db5d0838e5e820c6e24fa28529143657e054b260  easel4_near.first3.lua
2008650c208b4bc8f7e39c860c6d5481eb6145cabeffa76a61cb04375d4ca9c3  engine1_tiny.lua
96cb8e4608d995f336588c6d3a0919637afffd56bee1ee78f3b2950feb9ec3f3  studio_6399ad.first3.lua
e0d80d66507e8304be33bd29107dd5876a00722424b5809db2ac9dff8c699c52  studio_db6324.first3.lua
e7280cc46cd4ef5d73b2254901a8b9cb4af12163f3238dae8bbfffd4ad7f9e7a  ../r19_default_box.lua
2e663f7f3fa4bd033c0c47b88e119ee3b8d5fcf908b8b3b8894dd630152dceb8  cases.tsv
```
