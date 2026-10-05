( •̀ᴗ•́ ) The runner now continues without a sitting-count cap. It carries the painter's last handoff into the return prompt, requests inspection and explicit judgment of acknowledged unresolved passages and keeps painter completion separate from final-render export. Painter adherence to the new instructions remains unmeasured. [Runner](../runner/r21_chains.py), [watcher](../night-watch/watch.mjs), [viewer](../../../studio/studio.py).

**Exact launch prompt**

```text
Your brief is in BRIEF.md in this folder. Your last message is your reply: the painting's title if you give it one and a few sentences about the picture. At the end of each sitting, leave a handoff in your journal and last reply: passages still unresolved, practical lessons to carry forward from what you observed and recipes that failed. Before retrying a failed recipe, identify what has changed and why that could address the observed failure. Record what you observed and what remains uncertain.
```

**Exact return prompt, rendered with the actual previous Opus reply**

This is an offline rendering for review. The appended reply comes from the existing fourth sitting; no fifth sitting was launched. [Previous reply receipt](evidence.txt#L138).

```text
You're back at the easel. The painting is as you left it. Your brief is in BRIEF.md and your journal in notes/journal.md. Take your time with a fresh look at the whole canvas and detail crops in normal color. Read your journal handoff and apply its practical lessons before acting. Inspect each passage you previously acknowledged as unresolved in an appropriate detail crop, then explicitly judge it in your journal: improve it or accept it as it is, with your reason. You may finish with imperfections you deliberately accept. Before retrying a failed recipe, identify what has changed and why that could address the observed failure. Record what you observed and what remains uncertain. Is your heart happy with this? Is there something you would enjoy taking further? You can keep working for hours if you like, and the painting's simulated time is yours to use, whether that means days or years. Enjoy it. When you are happy with the painting, record your reflections and an updated handoff in your journal and last reply, carrying forward the practical lessons that still apply, then finish.

Your previous sitting's reply:
**October Dusk, the River Meadow**

The painting is finished, and I'm content with it. It shows a river meadow about twenty minutes after sunset. A lemon glow fades upward into violet-gray, and the pale pond below catches the same light. A dark grove of elms stands on the right, with the glow showing under its canopy between the trunks. A hazy far tree line runs behind the water, with a farmhouse and a thread of smoke. A small woman in a red shawl walks the worn path down toward the water. She is the one warm note in the picture and gives it its scale.

This visit was short:
- **Right-hand tree:** I took the pale streaks out and stippled a little dark over the dry paint, so the tree now reads as quiet shadow.
- **Left meadow:** I tried an opaque olive band near the shore, and it went down as a flat patch. It took many passes with a rag dipped in spirits to lift it, and a slight warm tint is left over the texture. This is the third time opaque paint over the textured meadow has turned into a patch; only transparent glazes work there.
- **Final glaze:** A graded transparent glaze darkens the left meadow from the shore toward the foreground. The field now rolls down into shadow, and the lit ground stays by the water and around the figure.

The crown edges are still more scalloped than Inness would have left them. Still, the glow caught twice, in the sky and in the water, with the dark trees between, is the place I set out to paint. My reflections are in the journal.
```

**Finish behavior**

There is no separate finish prompt. The ending instructions above are delivered on each return. A normal whole-canvas view and successful detail view after the last paint call remain required before the runner accepts a no-paint sitting as completion. The instruction also asks for a reasoned judgment of each acknowledged weak passage; the painter records this judgment in its own words. [Completion evidence owner](../runner/r21_chains.py#L678).

**What the return changes in practice**

The Opus reply above supplies both its remaining scalloped crowns and its observation about repeated opaque meadow patches. The new prompt calls for a detail inspection of those crowns and an explicit improve-or-accept judgment. A judgment such as “I inspected the crown crop and accept the scalloped edge because the silhouette serves the dusk scene” is an illustrative possible response. The painter may finish after that judgment. Before another opaque meadow attempt, the painter must identify what changed and why that could address the recorded flat-patch result. The journal remains the complete observation record and the return explicitly requests reading it; the preceding reply is carried verbatim. [Prompt source](../runner/r21_chains.py#L181).

**Actual offline runner output**

The fixture invokes the real runner with an offline replacement for the painter subprocess: five sittings add painting counts, then the sixth supplies a reviewed no-paint stop. An old cap file set to 1 does not stop it. No model or easel painting was run. [Saved output](runner-behavior-preview.json).

```json
{
  "evidence_kind": "Offline synthetic painter subprocess, no provider or painting launched",
  "old_cap_file": "1 (ignored)",
  "sittings": 6,
  "outcome": {
    "status": "finished",
    "reason": "the painter is done: sitting 6 reviewed whole and detail views and added no painting"
  },
  "six_crash_decision": [
    null,
    "NOT FINISHED: 6 crashes (MAX_CRASHES)"
  ]
}
```

**Completion and export outputs**

Historical Opus is represented as `cap_reached` in the offline fixture even though its PNG and Lua were exported. New voluntary completion is `finished`; six crashes are `crash_limit_reached`. The watcher preserves final-render metadata separately and refreshes the outcome even when pixels are unchanged. The live and static viewer retain those outcomes. [Actual watcher output](watcher-check.txt), [watcher boundary test](../night-watch/test_watch.mjs), [offline live/static outputs](outcome-preview.json).

**Actual offline error outputs and recovery**

These are captured from the current crop and Lua owners using synthetic test canvases. The old painting's crop failures concerned its exported version; the current crop limit is native pixels, so the new advice calculates the canvas-unit bound. [Executed errors and successful recovery examples](actionable-api-errors.txt), [baseline regression proof](actionable-api-errors-baseline.txt).

```text
local m = o - other: runtime error: outline arithmetic needs masks: use o:mask() - other:mask() for closed outlines; open outlines use o:below(), o:above() or o:band(10)
blend(o, {}): runtime error: want a mask, got userdata; use rect(100,100,200,200) or convert a drawn outline with o:mask() (closed), o:below()/o:above() (open), or o:band(10). Call the method: o.mask is not a mask
--crop exceeds 1200 pixels per side (crops stay 1:1); coordinates are canvas units, x0,y0,x1,y1 (opposite corners). At this canvas resolution each side may span at most 923.08 units; example: --crop 0.00,0.00,461.00,461.00
```

The Lua owner tests execute `o:mask()` conversion and the advertised mask-operation repairs successfully. The crop owner tests parse and render the advertised smaller coordinates successfully. These are offline tool-recovery fixtures. [Offline test receipt](actionable-api-errors.txt).

**Remaining operation safeguards**

There is no new tool-call count limit. Six crashes still stop the runner; individual Lua chunks retain a 10-minute execution timeout and easel tool operations have 12-minute paint, 3-minute other-operation and 30-minute rebuild-stall budgets. The runner's 30-minute process watchdog excludes the painter, easel server, check and finishing processes. Usage-limit recovery retains its 24-hour retry window. [Lua execution timeout](../../../crates/easel/src/session.rs#L49), [tool budgets](../../../harness/painter/easel-client.ts#L23), [runner safeguards](../runner/r21_chains.py).

Verification: [101 runner tests](runner-check.txt), [watcher subprocess regression](watcher-check.txt) and [runner regression failures on the baseline](runner-baseline-check.txt).

These are local source changes. The running viewer backend has not been restarted, so its adoption of new metadata is not verified. [Viewer verification](outcome-preview.json).
