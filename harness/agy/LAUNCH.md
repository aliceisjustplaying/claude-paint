# agy at the easel

This setup configures the official agy CLI for an approved studio. An operator starts agy and signs in by hand; the setup starts no model and handles no account. The studio's BRIEF.md and reading list are the approved ones, unchanged.

## Start

From the studio folder, `agy --agent easel-painter` starts the restricted painter. The operator picks Argon in agy's model picker before the first message. The exact Argon CLI selector and this account's access are unverified. The agent's tool list holds only `mcp_easel_paint`, `mcp_easel_look`, `mcp_easel_note`, `mcp_easel_status`, `mcp_easel_log`, `mcp_easel_read` and `mcp_easel_end_sitting`. It has no shell, web access, agy file writes or subagents. agy asks for approval of MCP tool calls, and its permission panel can grant the seven easel tools. No skip-permissions flag is used. The first session checks that agy accepts these MCP tool names before any painting.

agy attaches MCP tools only from the second turn, so the first message is this greeting and the launch message goes second:

> Hi, welcome to the studio, it's good to have you here. A quick hello back is plenty for now.

The setup starts the easel server with `--agy`. Every tool then carries `_meta` `{"antigravity/alwaysLoad": true}`, and text replies over 8,000 bytes come back in pages, because agy 1.3.5 cuts a text tool result over about 9.9 KB to its head and tail. `read` and `log` take a `page`. A longer `paint` or `end_sitting` reply is saved whole under `out/easel/replies/`, and its first page names the `read` call for the rest. Each page ends with one line naming the next call.

The launch message below, sent second, is the house runner's `PAINTER_MSG`, copied word for word from `r38/r21_chains.py` lines 54–61. The painter reads BRIEF.md with the easel's read tool.

> Your brief is in BRIEF.md in this folder. Your last message is your reply: the painting's title if you give it one and a few sentences about the picture. At the end of each sitting, leave a handoff in your journal and last reply: passages still unresolved, practical lessons to carry forward from what you observed and recipes that failed. Before retrying a failed recipe, identify what has changed and why that could address the observed failure. Carry forward the conditions of material trials and repairs: substrate and wetness, recipe ratios, medium or thinner, load, coverage and viewed result; label broader rules as hypotheses. Record what you observed and what remains uncertain.

## Sittings

`end_sitting` holds the tool call open through the configured break, then returns the house runner's `SITTING_MESSAGE` (word for word, from `sitting-message.txt`), the next sitting's number, the painting clock, the journal and a fresh whole-canvas PNG. The house runner has no scheduled break or sitting timer, so the defaults are a zero-second break and an untimed sitting. Setup accepts `--break-seconds` and `--sitting-seconds`. `--sitting-seconds` sets a time budget that `status` reports; painting continues past it. The painting is complete when `end_sitting` closes a sitting that added no paint and had a whole-canvas look and a detail crop in normal color. `out/easel/sittings.json` records completion; quota errors and CLI exits leave it unset.

During `end_sitting` the server pings every 15 seconds and sends MCP progress notifications when the client supplies a progress token. agy 1.3.4 contains a tool-call timeout path, and its public docs give neither the deadline nor whether pings reset it, so a long break needs checking in the first real session. An interrupted wait keeps its due time on disk, and the next `end_sitting` call resumes it.

## Recording

`out/easel/mcp/session.jsonl` records tool calls, returned text and PNGs live. The Lua is in `paintings/lua/painting.lua` and the journal in `notes/journal.md`. `.agents/recording.json` names the raw recorder and the merged viewer output. Tool calls and looks appear in the viewer as soon as the stdio server starts recording. The converter writes the merged file into the same session folder and renames each viewer link to the raw recorder as `.jsonl.raw`, leaving the recorder file intact. The viewer then shows the merged timeline.

A separate converter follows agy's transcript and the recorder; it never starts agy. agy's status-line documentation gives `transcript_path`, usually `~/.gemini/antigravity/brain/<conversation-id>/.system_generated/logs/transcript.jsonl`. An interactive agy 1.3.5 session with Gemini wrote `~/.gemini/antigravity-cli/brain/<conversation-id>/.system_generated/logs/transcript_full.jsonl`, with `transcript.jsonl` beside it. Both hold the same user and planner prose. `transcript_full.jsonl` keeps full tool output and decoded tool arguments, while `transcript.jsonl` may truncate tool output and stores arguments as JSON strings. The output path is the `output` field in `.agents/recording.json`. With the real paths and the exact model label, the operator command is:

```sh
uv run --no-project /path/to/claude-paint/harness/agy/convert.py \
  --recorder "$PWD/out/easel/mcp/session.jsonl" --transcript /path/to/transcript.jsonl \
  --output /path/to/merged/agy.jsonl \
  --model 'EXACT SELECTED MODEL' --follow
```

The converter maps the documented stream-json response fields, keeps each recorder tool call and image once and keeps unknown events without showing them as prose. Thinking appears only when the transcript exposes it. In the interactive transcript, `USER_INPUT.content` becomes user text and `PLANNER_RESPONSE.thinking` and `PLANNER_RESPONSE.content` become assistant thinking and text. Transcript `created_at` times have second precision and recorder times have millisecond precision, so the converter matches easel tool names in step order and places the prose after each tool step behind the matching recorder result. Tool calls, results and images come from the recorder alone, so none appear twice. The raw files stay intact. The tests cover made-up events and a shortened real transcript with no token counts or secrets, including partial and follow updates.

## Stop

Ctrl+C in agy's terminal interrupts the active turn. Exiting agy disconnects the stdio server and cancels its active MCP request. A break wait stops there. An oil-paint chunk the easel daemon has already started runs to its end after agy stops, and if it succeeds its marks stay on the painting; a chunk still waiting for the daemon is skipped. The easel can stay open, as it does between house sittings. The first session checks that agy really cancels the request. These steps affect only this agy session and its easel.

Sources: [MCP](https://antigravity.google/docs/cli/mcp/), [custom agents](https://antigravity.google/docs/subagents/), [permissions](https://antigravity.google/docs/cli/permissions/), [headless output](https://antigravity.google/docs/cli/headless/) and [transcript path](https://antigravity.google/docs/cli/statusline/).
