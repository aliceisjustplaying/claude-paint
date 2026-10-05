Five finished paintings: local media delivery

All five inventory candidates now have final PNGs and social MP4s. A runner-completed painting in a live studio but absent from the final gallery was counted as missing. Completion evidence and exclusions remain in [inventory.json](inventory.json).

| Painting | Finished PNG | Normal view | MP4 | Verification |
|---|---|---|---|---|
| r27 Hopper | [2400×1500](../round27/social/r27-hopr-finished.png) | [PNG](../round27/social/r27-hopr-normal.png) | [1920×1200](../round27/social/r27-hopr.mp4) | [Receipt](../round27/social/delivery-check.json) |
| r26 Monet | [2400×2400](assets/r26-mont/r26-mont-finished.png) | [PNG](assets/r26-mont/r26-mont-normal.png) | [1920×1920](assets/r26-mont/r26-mont.mp4) | [Receipt](assets/r26-mont/delivery-check.json) |
| r25 Friedrich | [2400×1714](assets/r25-frdc/r25-frdc-finished.png) | [PNG](assets/r25-frdc/r25-frdc-normal.png) | [1920×1372](assets/r25-frdc/r25-frdc.mp4) | [Receipt](assets/r25-frdc/delivery-check.json) |
| r24 Inness | [2400×1600](assets/r24-inns/r24-inns-finished.png) | [PNG](assets/r24-inns/r24-inns-normal.png) | [1920×1280](assets/r24-inns/r24-inns.mp4) | [Receipt](assets/r24-inns/delivery-check.json) |
| r22.1 Inness | [Original 2400×1600](assets/r22.1-inns/r22.1-inns-finished.png) | [Original PNG](assets/r22.1-inns/r22.1-inns-normal.png) | [Substitute 1920×1280](assets/r22.1-inns/r22.1-inns-substitute-f334aef.mp4) | [Receipt](assets/r22.1-inns/delivery-check.json) |

The finished PNGs are the deliverables: original automatic drying, 0.4 varnish coats and cracks. The preserved normal PNGs show the painter's last normal-color view. Videos are 20 seconds, 24 fps, H.264/yuv420p with faststart. Whole paintings are retained; Friedrich's video height uses even-pixel rounding. Exact PNGs, extracted video contact sheets and metadata were inspected for protected identity and home paths. Individual verification receipts record the checks.

The first four items replayed every original chunk plus automatic finishing with their pinned matching engines. Their final replay PNGs matched the existing finished PNGs byte for byte: Hopper 137+1 chunks, Monet 147+1, Friedrich 234+1 and r24 Inness 119+1. Source commits and binary hashes remain in the receipts.

r22.1 uses the explicitly authorized closest historical substitute, immutable commit `f334aef11d0c069b083371d2cb8ceb4865664e83`. Original source `0e00118` remains unavailable and unverified. All 311 original chunks plus finishing replayed successfully. The [substitute PNG](assets/r22.1-inns/r22.1-inns-substitute-f334aef.png) matches the preserved original finished PNG byte for byte; RGB MAE, RMSE, maximum channel difference and changed pixels are all zero. This verifies the final output, not original source identity or exact intermediate states. [Source choice](r22-engine-choice.json), [replay receipt](assets/r22.1-inns/substitute-receipt.json), [measured differences](assets/r22.1-inns/substitute-image-difference.json) and [contact sheet](assets/r22.1-inns/substitute-video-review.png) preserve provenance. The earlier static-only receipt is superseded for video availability by the substitute receipt.

[Final verification](final-verification.json) records artifact hashes, durable relative paths and cleanup. Task temporary source/build/frame directories and partial videos are gone. Existing worktrees and other agents' files were preserved.

Delivery remains local. No gallery entries, descriptions, quotes or deployment were added. Large media stays outside Git. Execution scripts, queues, inventory and detailed replay receipts/logs remain local task material; scripts contain the protected-name privacy match and queues contain machine-local paths, so they are excluded from the public documentation commit.
