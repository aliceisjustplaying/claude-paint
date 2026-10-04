# User decisions given in chat before sleep (supplement to claude-paint-overnight-plan.md)

1. Golden-approval scope: the user's approval gate (`refs/notes/golden-approvals`) covers the protected golden set: `notes/thinner/baseline/` and the approved thinner acceptance tests/helpers/answers. Other existing tests shrunk by the speed agent get another agent's review with written reasoning; assertions never weaken.
2. Option B (user priority: "tests should not take 6-10 minutes"): new hard-coded expected numbers for shrunk existing tests count as verified when computed by the UNCHANGED af49348 code in a release build, with commands + hashes recorded in `refs/notes/golden-approvals` and checked by a reviewer who is not the builder. Numbers produced by changed code wait for the user ("needs the owner" list).
3. Plan text saved verbatim at `~/src/a/claude-paint-overnight-plan.md`; its SHA-256 is the plan hash.
4. Push: YES. Push feature branches plus `refs/notes/test-receipts` and `refs/notes/golden-approvals` to origin (`aliceisjustplaying/claude-paint`, PUBLIC) through the normal privacy hooks. Never main, never tags.
5. The old r24-fixes worker is long done; round-24 is not being edited.
6. User goal for the morning: working engine 3 with all fixes, the thinner and faster tests, ready to kick off a painting (painting itself NOT launched tonight).

## Morning decisions (2026-10-04, chat)
7. Engine 3 has never made a real painting: NO backwards compatibility is required for engine 3. Its rag/thinner behavior may change; engine-3 baseline answers may change with the user's sign-off after seeing results. Engines 1 and 2 must stay byte-identical (old paintings).
8. Try: rag "less tidy" fixes; thinned-stroke rim fixes a (less plough for thinned paint) and b (ceiling caps the pixel's held wet film, not just additions); rerun the sienna comparison with the standard hiding ratio (over-black / over-white). Run the tests, then review where we are.
9. (morning) All Opus subagents run on MEDIUM thinking from now on (user is at 35% of the 7-day limit).
