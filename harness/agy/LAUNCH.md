# agy at the easel

This setup configures the official agy CLI for an approved studio. An operator starts agy and signs in by hand; the setup starts no model and handles no account. The studio's BRIEF.md and reading list are the approved ones, unchanged.

## Start

From the studio folder, `agy --agent easel-painter` starts the restricted painter. The operator picks Argon in agy's model picker before the first message. The exact Argon CLI selector and this account's access are unverified. The agent's tool list holds only `mcp_easel_paint`, `mcp_easel_look`, `mcp_easel_note`, `mcp_easel_status`, `mcp_easel_log`, `mcp_easel_read` and `mcp_easel_end_sitting`. It has no shell, web access, agy file writes or subagents. agy asks for approval of MCP tool calls, and its permission panel can grant the seven easel tools. No skip-permissions flag is used. The first session checks that agy accepts these MCP tool names before any painting.

The launch message below is the house runner's `PAINTER_MSG`, copied word for word from `r38/r21_chains.py` lines 54–61. The painter reads BRIEF.md with the easel's read tool.

> Your brief is in BRIEF.md in this folder. Your last message is your reply: the painting's title if you give it one and a few sentences about the picture. At the end of each sitting, leave a handoff in your journal and last reply: passages still unresolved, practical lessons to carry forward from what you observed and recipes that failed. Before retrying a failed recipe, identify what has changed and why that could address the observed failure. Carry forward the conditions of material trials and repairs: substrate and wetness, recipe ratios, medium or thinner, load, coverage and viewed result; label broader rules as hypotheses. Record what you observed and what remains uncertain.

## Sittings

`end_sitting` holds the tool call open through the configured break, then returns the house runner's `SITTING_MESSAGE` (word for word, from `sitting-message.txt`), the next sitting's number, the painting clock, the journal and a fresh whole-canvas PNG. The house runner has no scheduled break or sitting timer, so the defaults are a zero-second break and an untimed sitting. Setup accepts `--break-seconds` and `--sitting-seconds`. `--sitting-seconds` sets a time budget that `status` reports; painting continues past it. The painting is complete when `end_sitting` closes a sitting that added no paint and had a whole-canvas look and a detail crop in normal color. `out/easel/sittings.json` records completion; quota errors and CLI exits leave it unset.

During `end_sitting` the server pings every 15 seconds and sends MCP progress notifications when the client supplies a progress token. agy 1.3.4 contains a tool-call timeout path, and its public docs give neither the deadline nor whether pings reset it, so a long break needs checking in the first real session. An interrupted wait keeps its due time on disk, and the next `end_sitting` call resumes it.

## Recording

`out/easel/mcp/session.jsonl` records tool calls, returned text and PNGs live. The Lua is in `paintings/lua/painting.lua` and the journal in `notes/journal.md`. `.agents/recording.json` names the raw recorder and the merged viewer output. The raw recorder writes to a private sessions root outside the viewer's root, so the viewer shows each session once.

A separate converter follows agy's transcript and the recorder; it never starts agy. agy's status-line documentation gives `transcript_path`, usually `~/.gemini/antigravity/brain/<conversation-id>/.system_generated/logs/transcript.jsonl`. With the real path and the exact model label, the operator command is:

```sh
uv run --no-project /path/to/claude-paint/harness/agy/convert.py \
  --recorder "$PWD/out/easel/mcp/session.jsonl" --transcript /path/to/transcript.jsonl \
  --output /sessions-root/--mcp-src-a-paint-studio-HEX--/agy.jsonl \
  --model 'EXACT SELECTED MODEL' --follow
```

The converter maps the documented stream-json response fields, keeps each recorder tool call and image once and keeps unknown events without showing them as prose. Thinking appears only when the transcript exposes it. agy's docs don't describe the interactive transcript schema, so prose and thinking conversion stays provisional until the first real session, and the raw files stay intact for that check. The synthetic test covers made-up events only.

## Stop

Ctrl+C in agy's terminal interrupts the active turn. Exiting agy disconnects the stdio server and cancels its active call. The easel can stay open, as it does between house sittings. A running chunk's cancellation goes to its own process group. The first session checks that agy really cancels and that no paint lands after a stop. These steps affect only this agy session and its easel.

Sources: [MCP](https://antigravity.google/docs/cli/mcp/), [custom agents](https://antigravity.google/docs/subagents/), [permissions](https://antigravity.google/docs/cli/permissions/), [headless output](https://antigravity.google/docs/cli/headless/) and [transcript path](https://antigravity.google/docs/cli/statusline/).
