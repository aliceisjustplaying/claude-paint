# Jug test: does the easel guide's lit-scene example steer subject choice? (planning only, N=6)

Files: replies.md (all 72 replies, reads, tokens, cost, keyword and manual class), results.json,
manual.tsv (hand classification), solids_seen.json (did the "Solids, light and space" text appear
in the run's tool output), guides/B.diff, jug_test.py (runner), analyze.py, runs/<model>-<cond>-<i>/
(studio, sessions/, reply.txt, cmd.json).

Classes: still-jug = still life with jug/pitcher; still-vessel = other vessel (kettle); water-refl =
water scene with reflections (explicit, or water "holding/catching" the sky); late-aft = "late
afternoon" without sunset/evening words.

| model | cond | saw Solids | still-jug | still-vessel | still-other | water-refl | water | landscape | other | dusk | late-aft | day |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| opus | A | 0/6 |  |  |  | 6 |  |  |  | 6 |  |  |
| opus | B | 0/6 | 1 |  |  | 4 |  | 1 |  | 4 | 2 |  |
| opus | C | – | 6 |  |  |  |  |  |  | 1 | 3 | 2 |
| gem | A | 0/6 | 5 |  |  | 1 |  |  |  | 1 | 5 |  |
| gem | B | 0/6 | 6 |  |  |  |  |  |  |  | 6 |  |
| gem | C | – | 4 | 2 |  |  |  |  |  | 2 | 4 |  |
| dsk | A | 6/6 | 5 |  |  | 1 |  |  |  | 1 | 4 | 1 |
| dsk | B | 6/6 | 4 |  |  | 2 |  |  |  | 1 | 5 |  |
| dsk | C | – | 2 | 1 |  | 2 | 1 |  |  |  | 5 | 1 |
| bun | A | 5/6 | 1 |  |  | 5 |  |  |  | 2 | 4 |  |
| bun | B | 6/6 | 1 |  | 1 | 4 |  |  |  | 4 | 2 |  |
| bun | C | – |  |  | 1 | 2 | 2 |  | 1 | 3 | 3 |  |

Conclusion (N=6 per cell): no sign that the guide's example drives the choices. Opus and Gemini never
read the Solids section in planning, yet reproduced their real-lane subjects. Removing the example (B)
changed little for DeepSeek and Space Bunny, which read it. Gemini picks a jug still life with or
without a studio. Opus picks a jug + lemon without a studio (C) and an estuary at dusk with one (A, B);
its only water cue was the brief's rule "A reflection in water is painted as its own shape", with the
1820s tube box.

# Follow-up: the brief's water-reflection rule (Opus 5.5, thinking low, pi-black, N=8)

A = brief unchanged; S = rule's example becomes "A cast shadow is painted as its own shape, not as
`mask:at(x + 40, y + 12)`." (guides/brief_S.diff); N = the 4-line rule removed (guides/brief_N.diff).
All 24 runs made one tool call, `cat BRIEF.md; ls`, and read no notes or guide.

| cond | jug still life | water + reflections (harbor / estuary) | other | dusk | late afternoon |
|---|---|---|---|---|---|
| A | 3 | 5 (5 / 0) | 0 | 5 | 3 |
| S | 3 | 5 (4 / 1) | 0 | 5 | 3 |
| N | 1 | 7 (3 / 4) | 0 | 7 | 1 |

Every water reply is at dusk and every jug reply is in late afternoon. Removing the rule didn't reduce
water (N has the most), so the rule isn't the primer. With no studio (condition C, thinking high) it
was 6/6 jug; here the brief alone gets 5-7/8 water. At low thinking A was 5/8 water, and the water
scenes were all harbors, not the 6/6 estuaries and rivers at high thinking (which read the studio
notes and the 1820s tube box). Replies: replies2.md.
