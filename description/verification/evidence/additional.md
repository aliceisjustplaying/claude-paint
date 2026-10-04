# Additional isolated observations

2026-10-04, same release binary and isolated root as [CLI observations](cli.md). The original three-chunk [probe log](probe.lua) is preserved separately; these probes ran afterward.

| Input | Observed result |
|---|---|
| `bb=brush("round",4); bb:load(p,1); print("first",bb:fullness()); bb:load(p,1); print("second",bb:fullness())` | First 0.964333713054657; second 1.928667426109314; successful chunk 4 |
| `easel frames typo` | Exit 0; `frames off` |
| `print(wait(0)); print(wait(-1))` | Printed `day 1, 09:03`, then rejected -1 with range 0–5259600; reported chunk failed and changed nothing |
| `easel close` | Wrote live canvas and closed with log retained |

These observations confirm additive fullness and permissive frame arguments. They do not establish whether either behavior is the intended product contract. Negative/nonfinite brush-load amounts, timeout interruption and filesystem failure were not exercised.
