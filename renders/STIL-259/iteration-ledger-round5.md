# Round 5 source iteration ledger

All targets are unchanged measured white-field spectra converted with the original k1=0.04 display subtraction. Main and pictures are absent from calibration. These are float64 source experiments; the final score file uses the built Rust decoder. All rows were viewed during model selection, so the revised test is not a blind validation.

The final common partitions are used below even for earlier candidates. Revision 1 predeclared 16D/17D as fits; revision 2 added 13F and excluded pure 13A; revision 3 added 16F/17F and previously selection-touched 16E. The final 12 contain no extender mixtures. Old-17 scores include five now-fitted rows.

| Candidate | split | old 17 mean / worst | revised 12 mean / worst | all 33 worst |
|---|---:|---|---|---|
| Finite decoder only | 0 | 0.048414 / 0.176135 (17D) | 0.026132 / 0.050243 (13G) | 0.176135 (17D) |
| Cochineal extender K/S only | 1 | 0.047074 / 0.158645 (17D) | 0.026132 / 0.050243 (13G) | 0.158645 (17D) |
| Early joint K/S, including pure 13A | 1 | 0.040735 / 0.086699 (16F) | 0.035019 / 0.062485 (08D) | 0.086699 (16F) |
| Shared fitted surface, fixed display targets | 1 | 0.040913 / 0.086166 (16F) | 0.035271 / 0.062283 (08D) | 0.086166 (16F) |
| Constant weights fitted to white-field colors | 1 | 0.029336 / 0.085908 (17F) | 0.026097 / 0.049538 (13G) | 0.142828 (16E) |
| Replace Prussian pure anchor with 13F | 2 | 0.024364 / 0.085908 (17F) | 0.022976 / 0.040802 (13G) | 0.142828 (16E) |
| White-field color refinement 1 | 2 | 0.024055 / 0.082597 (17F) | 0.023004 / 0.040802 (13G) | 0.143446 (16E) |
| White-field color refinement 2 | 2 | 0.024102 / 0.082272 (17F) | 0.023101 / 0.040802 (13G) | 0.143359 (16E) |
| Both-field weights, partial extender fit, pass 5 | 2 | 0.022365 / 0.066435 (17F) | 0.021747 / 0.052123 (08F) | 0.147983 (16E) |
| Both-field weights, partial extender fit, pass 6 | 2 | 0.022368 / 0.066433 (17F) | 0.021752 / 0.052142 (08F) | 0.147978 (16E) |
| Complete declared spectra, local pass 0 | 3 | 0.027124 / 0.060702 (17D) | 0.021809 / 0.045365 (08F) | 0.073567 (16E) |
| Complete declared spectra, local pass 1 | 3 | 0.025789 / 0.048135 (16D) | 0.021464 / 0.046319 (08F) | 0.075383 (16E) |
| Complete declared spectra, local pass 2 | 3 | 0.025535 / 0.047680 (16D) | 0.021241 / 0.046712 (08F) | 0.075422 (16E) |
| Complete declared spectra, local pass 3 | 3 | 0.025416 / 0.047612 (16D) | 0.021096 / 0.046912 (08F) | 0.075420 (16E) |
| Complete declared spectra, local pass 4 | 3 | 0.025349 / 0.047600 (16D) | 0.021006 / 0.047006 (08F) | 0.075416 (16E) |
| Complete declared spectra, local pass 5 | 3 | 0.025327 / 0.047596 (16D) | 0.020976 / 0.047050 (08F) | 0.075412 (16E) |
| Complete declared spectra, local pass 6 | 3 | 0.025334 / 0.047594 (16D) | 0.020986 / 0.047069 (08F) | 0.075409 (16E) |
| Joint K/S after local pass 3 | 3 | 0.021635 / 0.045676 (16F) | 0.023212 / 0.041499 (13G) | 0.065510 (01C) |
| Recalibrate weights after that joint solve | 3 | 0.023020 / 0.048573 (16F) | 0.022439 / 0.041499 (13G) | 0.056162 (01C) |
| Joint K/S after local pass 5 | 3 | 0.021687 / 0.046236 (16F) | 0.023299 / 0.041500 (13G) | 0.061210 (01C) |
| Recalibrate weights after local-5 joint solve | 3 | 0.022941 / 0.047982 (16F) | 0.022466 / 0.041500 (13G) | 0.055083 (01C) |
| Another shared global weight/K/S alternation | 3 | 0.023134 / 0.044661 (04C) | 0.024113 / 0.044661 (04C) | 0.063312 (01C) |
| Shipped generator, tighter joint solve | 3 | 0.021635 / 0.045676 (16F) | 0.023212 / 0.041499 (13G) | 0.065510 (01C) |

The shipped joint solve after local pass 3 trades a slightly higher revised mean for lower worst errors and much smaller extender/cochineal errors than the sequential-only candidate. The later local-5 joint solve worsens both old and revised means. Recalibrating constants after the selected joint solve improves the revised mean but worsens the old mean, old worst and the priority extender rows. A further shared global weight/K/S alternation worsens both means and the revised worst. Those changes are not shipped.

This is the stopping evidence for the tried source refinements, not a claim of a global optimum or mathematical impossibility. No concentration law, sample-specific backing, per-pair correction or changed measurement is used. The shared-surface experiment was rejected and retains the fixed original display targets in its score.

Every row error and group statistic is in iteration-metrics-round5.json. The compressed source-model snapshots preserve the candidate K/S, fixed backing and weights independently of scratch scripts whose imports have since changed. The old round-4 runtime baseline is in score.md and reports/STIL-259.md, not reinterpreted through the finite decoder here.
