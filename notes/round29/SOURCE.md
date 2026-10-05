# Approved prompt and launch source

The approved combined prompt is preserved in [the working template](runner/brief_template.md), [initial and return messages](runner/r21_chains.py) and [the reviewed R27-to-R28 diff](../round28/prompt-changes.diff).

The stopped R28 Hopper source snapshot is `f2b324457e36fe762cb65dc30aefd0428b735d79`. The running R29 Monet source snapshot is `3f3f21150249688c4421aa763c7f4e3f5ed3ff91`. Both descend from R27 source `4b9bef6a5872803f4f20623c15b7677a152db70c`; their additions are round-specific prompt and runner files. Engine, materials and harness code are unchanged by these snapshots.

The maintained R29 runner pins the R29 snapshot, selects `anthropic/claude-opus-5-5` with high thinking and pi-black, exports the 15-tube Giverny profile and has no reader or sitting cap. The R28 source remains available for the canceled Hopper configuration. Runtime snapshots preserve their original preceding source pins; maintained runner copies pin the actual snapshots used at launch.

Launch receipts, session transcripts, generated media and machine-local process configuration remain local artifacts. They are excluded from this source checkpoint. No active runtime or frozen snapshot is changed by committing these source copies.
