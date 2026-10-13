# agy at the easel

This setup uses the official agy CLI directly. Nothing here launches a model, signs in or handles an account. The studio already contains its approved BRIEF.md and reading list. They are unchanged.

## Start

From the studio folder, `agy --agent easel-painter` selects the restricted painter. Argon is selected in agy's model picker before the first message. The exact Argon CLI selector and this account's access are not verified yet. The selected agent's tool list contains only `mcp_easel_paint`, `mcp_easel_look`, `mcp_easel_note`, `mcp_easel_status`, `mcp_easel_log`, `mcp_easel_read` and `mcp_easel_end_sitting`. Shell, web, file writes and subagents are absent. MCP approvals are shown by agy; approvals for those seven tools can be granted through its permission panel. No skip-permissions flag is used. The first session confirms agy accepts these canonical MCP tool names before painting.

The launch message below is copied word for word from the house runner's `PAINTER_MSG` (`r38/r21_chains.py:52–61`). The painter reads BRIEF.md through the easel's read tool.

> Your brief is in BRIEF.md in this folder. Your last message is your reply: the painting's title if you give it one and a few sentences about the picture. At the end of each sitting, leave a handoff in your journal and last reply: passages still unresolved, practical lessons to carry forward from what you observed and recipes that failed. Before retrying a failed recipe, identify what has changed and why that could address the observed failure. Carry forward the conditions of material trials and repairs: substrate and wetness, recipe ratios, medium or thinner, load, coverage and viewed result; label broader rules as hypotheses. Record what you observed and what remains uncertain.

## Sittings

`end_sitting` keeps the tool call open through the configured break. It then returns the sitting number, painting clock, journal and a fresh whole-canvas PNG. The house runner has no scheduled break or sitting timer, so the defaults are a zero-second break and unlimited sitting time. Setup accepts `--break-seconds` and `--sitting-seconds`; the latter is a displayed time budget, not an automatic cutoff. A no-new-paint sitting with successful whole and detail looks in normal color completes the painting. Quota errors and CLI exits do not establish completion.

The server sends MCP progress notifications when the client supplies a progress token and pings every 15 seconds during `end_sitting`. agy 1.3.4 contains a tool-call timeout path but its public docs do not establish its deadline or whether pings reset it. A configured long break therefore still needs validation in the first real session. Interrupted waits keep their due time on disk and can resume with another `end_sitting` call.

## Recording

Tool calls, returned text and PNGs are recorded live in `out/easel/mcp/session.jsonl`. Lua stays in `paintings/lua/painting.lua`; journal entries stay in `notes/journal.md`. `.agents/recording.json` names the raw recorder and the merged viewer output. The raw recorder uses a private sessions root outside the viewer's configured root to avoid showing duplicate sessions.

A separate converter can follow agy's transcript and the recorder; it never starts agy. The status-line documentation exposes `transcript_path`, usually under `~/.gemini/antigravity/brain/<conversation-id>/.system_generated/logs/transcript.jsonl`. With the real path and exact model label, the operator command is:

```sh
uv run --no-project /path/to/claude-paint/harness/agy/convert.py \
  --recorder "$PWD/out/easel/mcp/session.jsonl" --transcript /path/to/transcript.jsonl \
  --output /sessions-root/--mcp-src-a-paint-studio-HEX--/agy.jsonl \
  --model 'EXACT SELECTED MODEL' --follow
```

The converter maps documented stream-json response fields, keeps recorder tools and images once and retains unknown events without displaying them as prose. Thinking is included only when the transcript exposes it. The native interactive transcript schema is not documented here; prose and thinking conversion is provisional until the first real session. Raw files remain intact for that validation. The synthetic test is not a real-session receipt.

## Stop

Ctrl+C in agy's terminal interrupts the active turn. Exiting agy disconnects the stdio server and cancels its active call. The easel may stay open, as it does between house sittings. A running chunk's cancellation is sent to its own process group. Actual agy cancellation and absence of post-stop painting still need a first-session check. No stop command targets another painter or a viewer service.

Sources: [MCP](https://antigravity.google/docs/cli/mcp/), [custom agents](https://antigravity.google/docs/subagents/), [permissions](https://antigravity.google/docs/cli/permissions/), [headless output](https://antigravity.google/docs/cli/headless/) and [transcript path](https://antigravity.google/docs/cli/statusline/).
