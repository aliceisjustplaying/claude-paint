1. **Medium — dead sections become live again when nodes are exported as a limb.** `crates/paint/src/growth.rs:841` determines the entire limb's `dead` flag from its starting node (the second expression checks the same starting node again). The traversal at `growth.rs:804–816` continues through live-to-dead transitions without splitting. Decline explicitly kills upper leader nodes (`growth.rs:700–704`), but the root never becomes dead, so the exported trunk is always live even where its upper portion is dead. `paintings/src/trees.rs:64` consequently paints that dead section with `Bark.dark`, not the supplied `Bark.dead`; downstream consumers also cannot identify it. **Evidence:** current-source probe with default `Habit::dead_oak()`, seed 7: the exact trunk continuation path contains **20 dead nodes out of 55**, yet `limbs[0].dead == false`. This loses the stag-headed state promised by `notes/motifs.md`. **Fix:** split the limb when dead state changes and preserve connectivity/order, or export per-segment dead state and consume it in the painter. Add a test comparing node decline state to exported segments.

2. **Medium — two-internode decay claws retain unpruned sibling branches.** `crates/paint/src/growth.rs:722–730` follows the strongest child when `keep == 2`, then deletes only that child's children. Other children of the first thin node remain attached with their entire subtrees. The subsequent decay iterations skip them at `growth.rs:714–715` because their parent is thin and dead, assuming they were already removed. Thus the default dead oak retains fine dead branches longer than the promised one- or two-internode claws. **Evidence:** current-source probe measures remaining thin-dead subtrees after decline for default `Habit::dead_oak()`: seed 2 has one depth-3 subtree, seed 7 has one depth-4 subtree and seed 11 has two depth-3 subtrees. **Fix:** at each retained claw node, remove and kill every child except the chosen continuation; at the last retained node remove all children. Assert every surviving thin-dead subtree has maximum depth <= 2.

### Reproduction receipt

No tracked edits or Cargo build. Reviewed `growth.rs`, `paintings/src/{trees,figures}.rs` and the motif/tree study bins against `notes/motifs.md`; source probe only, no visual rendering claim. No high/critical defect established in this scope.

Scratch sources: `~/tmp/review-api-42caee19/motif_probe.rs` (current `growth.rs` copied verbatim plus imports and probe), `~/tmp/review-api-42caee19/motif_probe_suffix.rs`.

From repository root:

```sh
export TMPDIR=~/tmp/review-api-42caee19 TMP=~/tmp/review-api-42caee19 TEMP=~/tmp/review-api-42caee19 CARGO_TARGET_DIR=target-api
timeout 40 rustc --edition=2021 -O "$TMPDIR/motif_probe.rs" --extern paint=target-api/release/deps/libpaint-d20806e489302b5e.rlib -L dependency=target-api/release/deps -o "$TMPDIR/motif_probe"
timeout 30 "$TMPDIR/motif_probe"
```

Output saved at `~/tmp/review-api-42caee19/motif_probe_output.txt`:

```text
seed=2 retained fine-dead subtrees deeper than 2=1 max_depth=3; tips returns 12 explicitly dead limb ends; trunk dead=false
seed=7 retained fine-dead subtrees deeper than 2=1 max_depth=4; tips returns 27 explicitly dead limb ends; trunk dead=false
seed=11 retained fine-dead subtrees deeper than 2=2 max_depth=3; tips returns 32 explicitly dead limb ends; trunk dead=false
seed=7 trunk chain has 20/55 dead nodes but exported Limb.dead=false
```

The additional `tips()` counts are incidental probe output, not included as a separate severity finding.
